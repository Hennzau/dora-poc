use std::sync::Arc;

use narr_rs::prelude::DaemonAddress;

pub mod check;
pub mod list;

async fn create_cli_session(connect: DaemonAddress) -> eyre::Result<Arc<zenoh::Session>> {
    let mut zenoh_config = zenoh::Config::default();

    zenoh_config
        .insert_json5(
            "connect/endpoints",
            &serde_json::json!(vec![connect.to_string()]).to_string(),
        )
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
