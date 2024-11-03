use eyre::{bail, OptionExt};
use narr_core::application::Application;
use narr_daemon_communication::{DaemonCommunication, ListenAddress};

pub struct Daemon {
    application: Option<Application>,
    communication: DaemonCommunication,
}

impl Daemon {
    pub async fn new_with_application(application: Application) -> Self {
        let daemon_communication = DaemonCommunication::new();

        Daemon {
            application: Some(application),
            communication: daemon_communication,
        }
    }

    pub async fn new_without_application(
        listen_addresses: Vec<ListenAddress>,
    ) -> eyre::Result<Self> {
        let daemon_communication = DaemonCommunication::new().listen_to(listen_addresses)?;

        println!(
            "DaemonCommunication: {:?}",
            daemon_communication.session_config
        );

        Ok(Daemon {
            application: None,
            communication: daemon_communication,
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
