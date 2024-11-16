use std::sync::Arc;

use eyre::OptionExt;
use tokio::sync::mpsc::Receiver;
use zenoh::{query::Query, Session};

use crate::queries::{DaemonQuery, DaemonReply};

async fn handle_query(query: Query) -> eyre::Result<()> {
    let message = DaemonQuery::from_bytes(
        query
            .payload()
            .ok_or_eyre(eyre::eyre!("Query doesn't contain any payload"))?
            .to_bytes()
            .as_ref(),
    )?;

    match message {
        DaemonQuery::Check => {
            if let Err(e) = query
                .reply(query.key_expr(), DaemonReply::Ok.to_bytes()?.as_slice())
                .await
                .map_err(eyre::Report::msg)
            {
                tracing::error!("Error replying to query: {:?}", e);
            }
        }
    }

    Ok(())
}

pub async fn spawn_daemon_handler(
    session: Arc<Session>,
    id: String,
    mut abort_rx: Receiver<()>,
) -> eyre::Result<()> {
    let daemon_id = id.clone();

    let queryable = session
        .declare_queryable(format!("narr/daemon/{}/query", id))
        .await
        .map_err(eyre::Report::msg)?;

    tokio::task::spawn(async move {
        loop {
            tokio::select! {
                query = queryable.recv_async() => {
                    if let Ok(query) = query {
                        tracing::info!("Received query: {:?}", query);

                        if let Err(e) = handle_query(query).await {
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

        tracing::info!("Daemon handler for {} stopped", daemon_id);
    });

    Ok(())
}
