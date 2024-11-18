use std::sync::Arc;

use zenoh::{query::Query, Session};

use crate::{queries::DaemonReply, DaemonInfo};

pub async fn handle_check(
    info: DaemonInfo,
    _session: Arc<Session>,
    query: Query,
) -> eyre::Result<()> {
    let listen = info
        .listen
        .iter()
        .map(|address| address.to_string())
        .collect::<Vec<String>>();

    let reachable = format!("{:?}", listen);
    let id = info.id.clone();

    if let Err(e) = query
        .reply(
            format!("narr/daemon/{}/query", info.id),
            DaemonReply::Ok(crate::queries::InfoReply { id, reachable }).to_bytes()?,
        )
        .await
        .map_err(eyre::Report::msg)
    {
        tracing::error!("Error replying to query: {:?}", e);
    }

    Ok(())
}
