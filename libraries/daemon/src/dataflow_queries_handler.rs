use eyre::OptionExt;
use zenoh::query::Query;

use crate::{queries::DataFlowQuery, Daemon};

pub async fn handle_query(_daemon: &mut Daemon, query: Query) -> eyre::Result<()> {
    let message = DataFlowQuery::from_bytes(
        query
            .payload()
            .ok_or_eyre(eyre::eyre!("Query doesn't contain any payload"))?
            .to_bytes()
            .as_ref(),
    )?;

    match message {
        DataFlowQuery::Test => {}
    }

    Ok(())
}
