use std::sync::Arc;

use check::handle_check;
use eyre::OptionExt;
use tokio::sync::mpsc::Receiver;
use zenoh::{query::Query, Session};

use crate::queries::DaemonQuery;

mod check;

async fn handle_query(id: String, query: Query) -> eyre::Result<()> {
    let message = DaemonQuery::from_bytes(
        query
            .payload()
            .ok_or_eyre(eyre::eyre!("Query doesn't contain any payload"))?
            .to_bytes()
            .as_ref(),
    )?;

    match message {
        DaemonQuery::Check => handle_check(id, query).await?,
    }

    Ok(())
}

pub async fn spawn_daemon_handler(
    session: Arc<Session>,
    id: String,
    mut abort_rx: Receiver<()>,
) -> eyre::Result<()> {
    let queryable = session
        .declare_queryable(format!("narr/daemon/{}/query", id))
        .await
        .map_err(eyre::Report::msg)?;

    tokio::task::spawn(async move {
        loop {
            tokio::select! {
                query = queryable.recv_async() => {
                    if let Ok(query) = query {
                        if let Err(e) = handle_query(id.clone(), query).await {
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
