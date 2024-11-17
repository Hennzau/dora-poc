use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

use narr_rs::prelude::*;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .parse("")?,
        )
        .init();

    tracing::info!("Starting daemon");

    let mut zenoh_config = zenoh::Config::default();

    zenoh_config
        .insert_json5(
            "connect/endpoints",
            &serde_json::json!(vec!["udp/127.0.0.1:7447"]).to_string(),
        )
        .map_err(eyre::Report::msg)?;

    zenoh_config
        .insert_json5(
            "scouting/multicast/enabled",
            &serde_json::json!(false).to_string(),
        )
        .map_err(eyre::Report::msg)?;

    let session = zenoh::open(zenoh_config).await.map_err(eyre::Report::msg)?;

    let query = session
        .get("narr/daemon/LOCAL_1/query")
        .payload(DaemonQuery::Check.to_bytes()?.as_ref())
        .await
        .map_err(eyre::Report::msg)?;

    while let Ok(reply) = query.recv_async().await {
        match reply.result() {
            Ok(reply) => {
                tracing::info!("Received reply: {:?}", reply);

                if let Ok(reply) = DaemonReply::from_bytes(reply.payload().to_bytes().as_ref()) {
                    if reply == DaemonReply::Ok {
                        tracing::info!("Received OK response");
                    }
                }
            }
            Err(err) => {
                tracing::error!("Error receiving reply: {:?}", err);
            }
        }
    }

    Ok(())
}
