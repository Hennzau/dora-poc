use std::sync::Arc;

use check::{handle_check, handle_check_file};
use eyre::OptionExt;
use tokio::sync::mpsc::Receiver;
use zenoh::{query::Query, Session};

use crate::{queries::DaemonQuery, DaemonInfo};

mod check;

async fn handle_query(info: DaemonInfo, _session: Arc<Session>, query: Query) -> eyre::Result<()> {
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
    }

    Ok(())
}

pub async fn spawn(
    session: Arc<Session>,
    info: DaemonInfo,

    mut abort_rx: Receiver<()>,
) -> eyre::Result<()> {
    let queryable = session
        .declare_queryable(format!("narr/daemon/{}/query", info.id))
        .await
        .map_err(eyre::Report::msg)?;

    tokio::task::spawn(async move {
        loop {
            tokio::select! {
                query = queryable.recv_async() => {
                    if let Ok(query) = query {
                        if let Err(e) = handle_query(info.clone(), session.clone(), query).await {
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
