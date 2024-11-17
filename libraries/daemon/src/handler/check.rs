use zenoh::query::Query;

use crate::queries::DaemonReply;

pub async fn handle_check(id: String, query: Query) -> eyre::Result<()> {
    if let Err(e) = query
        .reply(
            format!("narr/daemon/{}/query", id),
            DaemonReply::Ok(id).to_bytes()?.as_ref(),
        )
        .await
        .map_err(eyre::Report::msg)
    {
        tracing::error!("Error replying to query: {:?}", e);
    }

    Ok(())
}
