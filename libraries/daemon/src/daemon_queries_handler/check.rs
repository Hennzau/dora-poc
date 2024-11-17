use std::sync::Arc;

use zenoh::{query::Query, Session};

use crate::{queries::DaemonReply, DaemonInfo};

pub async fn handle_check(
    info: DaemonInfo,
    session: Arc<Session>,
    query: Query,
) -> eyre::Result<()> {
    let zid = session.zid().to_string();

    let reachable = format!("{:?}", info.listen);

    if let Err(e) = query
        .reply(
            format!("narr/daemon/{}/query", info.id),
            DaemonReply::Ok(info.id, reachable).to_bytes()?.as_ref(),
        )
        .await
        .map_err(eyre::Report::msg)
    {
        tracing::error!("Error replying to query: {:?}", e);
    }

    Ok(())
}
