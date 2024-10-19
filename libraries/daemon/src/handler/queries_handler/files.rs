use std::{path::PathBuf, sync::Arc};

use zenoh::{
    query::Query,
    shm::{BlockOn, GarbageCollect, PosixShmProviderBackend, ShmProvider, StaticProtocolID},
    Session,
};

use crate::{queries::DaemonReply, DaemonInfo};

pub async fn handle_send_file(
    info: DaemonInfo,
    session: Arc<Session>,
    daemon: String,
    file: PathBuf,
    new_name: String,
    query: Query,
    provider: &ShmProvider<StaticProtocolID<0>, PosixShmProviderBackend>,
) -> eyre::Result<()> {
    if !file.exists() {
        query
            .reply(
                format!("dpoc/daemon/{}/query", info.id),
                DaemonReply::FileNotFound.to_bytes()?,
            )
            .await
            .map_err(eyre::Report::msg)?;

        return Err(eyre::eyre!("File {:?} not found", file));
    }

    let size = file.metadata()?.len() as usize;

    let mut sbuf = provider
        .alloc(size)
        .with_policy::<BlockOn<GarbageCollect>>()
        .await
        .map_err(|e| eyre::eyre!("Error allocating shared memory: {:?}", e))?;

    let bytes = tokio::fs::read(file).await?;

    sbuf.copy_from_slice(&bytes);

    let intermediate_query = session
        .get(format!("dpoc/daemon/{}/file", daemon))
        .attachment(new_name)
        .payload(sbuf)
        .await
        .map_err(eyre::Report::msg)?;

    let reply = intermediate_query
        .recv_async()
        .await
        .map_err(eyre::Report::msg)?;

    match reply.result() {
        Ok(reply) => {
            if let Ok(reply) = DaemonReply::from_bytes(&reply.payload().to_bytes()) {
                query
                    .reply(format!("dpoc/daemon/{}/query", info.id), reply.to_bytes()?)
                    .await
                    .map_err(eyre::Report::msg)?;
            } else {
                tracing::error!("Received invalid reply: {:?}", reply);
            }
        }
        Err(e) => {
            query
                .reply(
                    format!("dpoc/daemon/{}/query", info.id),
                    DaemonReply::FileSendFailed.to_bytes()?,
                )
                .await
                .map_err(eyre::Report::msg)?;

            tracing::error!("Error receiving reply: {:?}", e);
        }
    }

    Ok(())
}
