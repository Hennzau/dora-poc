use std::sync::Arc;

use check::{handle_check, handle_check_file};
use eyre::OptionExt;
use files::handle_send_file;
use tokio::sync::mpsc::Receiver;
use zenoh::shm::{ShmProvider, StaticProtocolID};
use zenoh::{query::Query, Session};

use zenoh::{
    shm::{PosixShmProviderBackend, ShmProviderBuilder, POSIX_PROTOCOL_ID},
    Wait,
};

use crate::{queries::DaemonQuery, DaemonInfo};

mod check;
mod files;

async fn handle_query(
    info: DaemonInfo,
    _session: Arc<Session>,
    query: Query,
    provider: &ShmProvider<StaticProtocolID<0>, PosixShmProviderBackend>,
) -> eyre::Result<()> {
    let message = DaemonQuery::from_bytes(
        query
            .payload()
            .ok_or_eyre(eyre::eyre!("Query doesn't contain any payload"))?
            .to_bytes()
            .as_ref(),
    )?;

    match message {
        DaemonQuery::Check => handle_check(info, query).await?,
        DaemonQuery::CheckFile(path) => handle_check_file(info, path, query).await?,
        DaemonQuery::File(path) => handle_send_file(info, path, query, provider).await?,
    }

    Ok(())
}

pub async fn spawn(
    session: Arc<Session>,
    info: DaemonInfo,

    mut abort_rx: Receiver<()>,
) -> eyre::Result<()> {
    let queryable = session
        .declare_queryable(format!("dpoc/daemon/{}/query", info.id))
        .await
        .map_err(eyre::Report::msg)?;

    let backend = PosixShmProviderBackend::builder()
        .with_size(1024 * 1024 * 256)
        .map_err(|e| eyre::eyre!("Error creating backend: {:?}", e))?
        .wait()
        .map_err(eyre::Report::msg)?;

    let provider = ShmProviderBuilder::builder()
        .protocol_id::<POSIX_PROTOCOL_ID>()
        .backend(backend)
        .wait();

    tokio::task::spawn(async move {
        loop {
            tokio::select! {
                query = queryable.recv_async() => {
                    if let Ok(query) = query {
                        if let Err(e) = handle_query(info.clone(), session.clone(), query, &provider).await {
                            tracing::error!("Error handling query: {}", e)
                        }
                    } else {
                        tracing::error!("Error receiving query");
                    }
                }

                _ = abort_rx.recv() => {
                    tracing::info!("Received abort signal");
                    break;
                }
            }
        }
    });

    Ok(())
}
