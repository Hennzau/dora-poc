use std::sync::Arc;

use tokio::sync::mpsc::Receiver;
use zenoh::{query::Query, Session};

use crate::{queries::DaemonReply, DaemonInfo};

async fn handle_download_file(info: DaemonInfo, query: Query) -> eyre::Result<()> {
    tracing::info!("Handling query");

    if let Err(e) = query
        .reply(
            format!("narr/daemon/{}/download_file", info.id),
            DaemonReply::FileOk.to_bytes()?,
        )
        .await
        .map_err(eyre::Report::msg)
    {
        tracing::error!("Error replying to query: {:?}", e);
    }

    Ok(())
}

pub async fn spawn(
    session: Arc<Session>,
    info: DaemonInfo,
    mut abort_rx: Receiver<()>,
) -> eyre::Result<()> {
    let download_file = session
        .declare_queryable(format!("narr/daemon/{}/download_file", info.id))
        .await
        .map_err(eyre::Report::msg)?;

    tokio::task::spawn(async move {
        loop {
            tokio::select! {
                query = download_file.recv_async() => {
                    if let Ok(query) = query {
                        if let Err(e) = handle_download_file(info.clone(), query).await {
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
