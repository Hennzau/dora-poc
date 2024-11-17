use std::sync::Arc;

use narr_core::daemon::address::DaemonAddress;
use tokio::sync::mpsc::{self, Sender};

pub mod handler;
pub mod queries;

pub struct Daemon {
    pub id: String,
    pub session: Arc<zenoh::Session>,
    pub abort_tx: Sender<()>,
}

impl Daemon {
    pub async fn spawn(
        id: String,
        listen: Vec<DaemonAddress>,
        connect: Vec<DaemonAddress>,
    ) -> eyre::Result<Daemon> {
        let mut zenoh_config = zenoh::config::Config::default();

        let connect = connect
            .iter()
            .map(|address| address.to_string())
            .collect::<Vec<String>>();

        let listen = listen
            .iter()
            .map(|address| address.to_string())
            .collect::<Vec<String>>();

        zenoh_config
            .insert_json5("connect/endpoints", &serde_json::json!(connect).to_string())
            .map_err(eyre::Report::msg)?;

        zenoh_config
            .insert_json5("listen/endpoints", &serde_json::json!(listen).to_string())
            .map_err(eyre::Report::msg)?;

        zenoh_config
            .insert_json5(
                "scouting/multicast/enabled",
                &serde_json::json!(false).to_string(),
            )
            .map_err(eyre::Report::msg)?;

        let session = Arc::new(zenoh::open(zenoh_config).await.map_err(eyre::Report::msg)?);

        let (abort_tx, abort_rx) = mpsc::channel(8);

        if let Err(e) = handler::spawn_daemon_handler(session.clone(), id.clone(), abort_rx).await {
            tracing::error!("Fatal error spawning daemon handler: {:?}", e);
        }

        let daemon = Daemon {
            id,
            session,
            abort_tx,
        };

        Ok(daemon)
    }
}
