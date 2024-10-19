use std::path::PathBuf;

use zenoh::{
    query::Query,
    shm::{BlockOn, GarbageCollect, PosixShmProviderBackend, ShmProvider, StaticProtocolID},
};

use crate::DaemonInfo;

pub async fn handle_send_file(
    info: DaemonInfo,
    file: PathBuf,
    query: Query,
    provider: &ShmProvider<StaticProtocolID<0>, PosixShmProviderBackend>,
) -> eyre::Result<()> {
    if !file.exists() {
        if let Err(e) = query
            .reply(format!("dpoc/daemon/{}/query", info.id), vec![])
            .await
            .map_err(eyre::Report::msg)
        {
            tracing::error!("Error replying to query: {:?}", e);
        }
    }

    let size = file.metadata()?.len() as usize;

    let mut sbuf = provider
        .alloc(size)
        .with_policy::<BlockOn<GarbageCollect>>()
        .await
        .map_err(|e| eyre::eyre!("Error allocating shared memory: {:?}", e))?;

    let bytes = tokio::fs::read(file).await?;

    sbuf.copy_from_slice(&bytes);

    if let Err(e) = query
        .reply(format!("dpoc/daemon/{}/query", info.id), sbuf)
        .await
        .map_err(eyre::Report::msg)
    {
        tracing::error!("Error replying to query: {:?}", e);
    }

    Ok(())
}
