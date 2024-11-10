use crate::{ConnectAddress, ListenAddress};

pub struct DaemonToDaemonCommunication {
    pub zenoh_session: zenoh::Session,
}

impl DaemonToDaemonCommunication {
    pub async fn new(
        listen_addresses: Vec<ListenAddress>,
        connect_addresses: Vec<ConnectAddress>,
    ) -> eyre::Result<Self> {
        let mut zenoh_config = zenoh::config::Config::default();

        zenoh_config
            .insert_json5(
                "connect/endpoints",
                &serde_json::json!(connect_addresses).to_string(),
            )
            .map_err(eyre::Report::msg)?;

        zenoh_config
            .insert_json5(
                "listen/endpoints",
                &serde_json::json!(listen_addresses).to_string(),
            )
            .map_err(eyre::Report::msg)?;

        let zenoh_session = zenoh::open(zenoh_config).await.map_err(eyre::Report::msg)?;

        Ok(DaemonToDaemonCommunication { zenoh_session })
    }
}
