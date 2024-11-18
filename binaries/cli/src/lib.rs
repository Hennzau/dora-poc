use std::sync::Arc;

use narr_rs::prelude::DaemonAddress;

pub mod check;
pub mod distribute;
pub mod list;
pub mod validate;

async fn create_cli_session(connect: Vec<DaemonAddress>) -> eyre::Result<Arc<zenoh::Session>> {
    let mut zenoh_config = zenoh::Config::default();

    let connect = connect
        .iter()
        .map(|address| address.to_string())
        .collect::<Vec<_>>();

    zenoh_config
        .insert_json5("connect/endpoints", &serde_json::json!(connect).to_string())
        .map_err(eyre::Report::msg)?;

    zenoh_config
        .insert_json5(
            "scouting/multicast/enabled",
            &serde_json::json!(false).to_string(),
        )
        .map_err(eyre::Report::msg)?;

    let session = zenoh::open(zenoh_config).await.map_err(eyre::Report::msg)?;

    Ok(Arc::new(session))
}
