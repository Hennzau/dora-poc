use std::sync::Arc;

use dpoc_core::address::DaemonAddress;
use handler::{files_handler, queries_handler};
use tokio::{signal::ctrl_c, sync::mpsc};

pub mod handler;
pub mod queries;

#[derive(Debug, Clone)]
pub struct DaemonInfo {
    id: String,
    listen: Vec<DaemonAddress>,
    #[allow(dead_code)]
    connect: Vec<DaemonAddress>,
}

pub struct Daemon {
    session: Arc<zenoh::Session>,

    info: DaemonInfo,
}

impl Daemon {
    pub async fn spawn(
        id: String,
        listen: Vec<DaemonAddress>,
        connect: Vec<DaemonAddress>,
    ) -> eyre::Result<Daemon> {
        let mut zenoh_config = zenoh::config::Config::default();

        let info = DaemonInfo {
            id: id.clone(),
            listen: listen.clone(),
            connect: connect.clone(),
        };

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

        let daemon = Daemon { session, info };

        Ok(daemon)
    }

    pub async fn run(&mut self) -> eyre::Result<()> {
        let (abort_tx_queries, abort_rx_queries) = mpsc::channel(1);
        let (abort_tx_files, abort_rx_files) = mpsc::channel(1);

        let queries_handler =
            queries_handler::spawn(self.session.clone(), self.info.clone(), abort_rx_queries)
                .await?;

        let files_handler =
            files_handler::spawn(self.session.clone(), self.info.clone(), abort_rx_files).await?;

        ctrl_c().await?;

        abort_tx_queries.send(()).await?;
        abort_tx_files.send(()).await?;

        queries_handler.await?;
        files_handler.await?;

        Ok(())
    }
}
