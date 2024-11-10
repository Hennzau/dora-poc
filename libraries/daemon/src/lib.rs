use daemon_to_daemon::DaemonToDaemonCommunication;
use eyre::bail;

use narr_core::application::Application;

pub mod daemon_to_daemon;
pub mod event;

pub type ListenAddress = String;
pub type ConnectAddress = String;

pub struct Daemon {
    daemon_id: String,
    application: Option<Application>,

    daemon_to_daemon_com: DaemonToDaemonCommunication,

    event_tx: tokio::sync::mpsc::Sender<event::DaemonEvent>,
    event_rx: tokio::sync::mpsc::Receiver<event::DaemonEvent>,
}

impl Daemon {
    pub async fn new_with_application(
        daemon_id: String,
        application: Application,
    ) -> eyre::Result<Self> {
        let (event_tx, event_rx) = tokio::sync::mpsc::channel(100);

        let listen_addresses = vec!["udp/0.0.0.0:0".to_string()];
        let mut connect_addresses = vec![];

        for (label, remote_daemon) in &application.daemons {
            if label == "LOCAL" {
                continue;
            }

            connect_addresses.push(remote_daemon.address.to_string());
        }

        Ok(Daemon {
            daemon_id,
            application: Some(application),
            daemon_to_daemon_com: DaemonToDaemonCommunication::new(
                listen_addresses,
                connect_addresses,
            )
            .await?,
            event_tx,
            event_rx,
        })
    }

    pub async fn new_without_application(
        daemon_id: String,
        listen_addresses: Vec<ListenAddress>,
    ) -> eyre::Result<Self> {
        let (event_tx, event_rx) = tokio::sync::mpsc::channel(100);

        Ok(Daemon {
            daemon_id,
            application: None,
            daemon_to_daemon_com: DaemonToDaemonCommunication::new(listen_addresses, vec![])
                .await?,
            event_tx,
            event_rx,
        })
    }

    pub async fn distribute(&self) -> eyre::Result<()> {
        let application = self
            .application
            .as_ref()
            .ok_or_else(|| eyre::eyre!("No application"))?;

        bail!("Not implemented")
    }

    pub async fn run(&self) -> eyre::Result<()> {
        bail!("Not implemented")
    }
}
