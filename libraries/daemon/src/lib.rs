use std::sync::Arc;

use narr_core::address::DaemonAddress;
use tokio::sync::mpsc;

pub mod daemon_queries_handler;
pub mod dataflow_queries_handler;
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
        let (abort_tx, abort_rx) = mpsc::channel(8);

        if let Err(e) =
            daemon_queries_handler::spawn(self.session.clone(), self.info.clone(), abort_rx).await
        {
            tracing::error!("Fatal error spawning daemon handler: {:?}", e);
        }

        let queryable = self
            .session
            .declare_queryable(format!("narr/daemon/{}/dataflow", self.info.id))
            .await
            .map_err(eyre::Report::msg)?;

        loop {
            tokio::select! {
                query = queryable.recv_async() => {
                    if let Ok(query) = query {
                        dataflow_queries_handler::handle_query(self, query).await?;
                    } else {
                        tracing::error!("Error receiving query");
                    }
                }

                _ = tokio::signal::ctrl_c() => {
                    tracing::info!("Received abort signal");

                    if let Err(e) = abort_tx.send(()).await {
                        tracing::error!("Error sending abort signal: {:?}", e);
                    }

                    break;
                }
            }
        }

        Ok(())
    }
}
