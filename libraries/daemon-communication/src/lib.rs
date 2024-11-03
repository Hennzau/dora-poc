pub type ListenAddress = String;

pub struct DaemonCommunication {
    pub session_config: zenoh::Config,
}

impl DaemonCommunication {
    pub fn new() -> Self {
        DaemonCommunication {
            session_config: zenoh::Config::default(),
        }
    }

    pub fn listen_to(self, listen_addresses: Vec<ListenAddress>) -> eyre::Result<Self> {
        let mut config = self.session_config;

        config
            .insert_json5(
                "listen/endpoints",
                &serde_json::json!(listen_addresses).to_string(),
            )
            .map_err(|e| eyre::eyre!("Failed to insert listen addresses: {}", e))?;

        Ok(DaemonCommunication {
            session_config: config,
        })
    }
}
