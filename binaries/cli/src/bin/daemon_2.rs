use narr_rs::prelude::*;
use tokio::signal::ctrl_c;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

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

    let daemon = Daemon::spawn(
        "LOCAL_2".to_string(),
        vec![DaemonAddress::from_string(
            "udp/127.0.0.1:7446".to_string(),
        )?],
        vec![DaemonAddress::from_string(
            "udp/127.0.0.1:7447".to_string(),
        )?],
    )
    .await?;

    let _ = ctrl_c().await;

    daemon.abort_tx.send(()).await?;

    Ok(())
}
