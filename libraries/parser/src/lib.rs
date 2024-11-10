use std::collections::HashMap;
use std::path::PathBuf;

use eyre::OptionExt;

use narr_core::application::Application;
use narr_core::daemon::address::DaemonAddress;
use narr_core::daemon::DaemonLabel;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    narr: Narr,
}

#[derive(Debug, Deserialize)]
struct Narr {
    name: String,
    local: String,
    working_directory: String,
    remote: Option<Vec<Remote>>,
    node: Vec<Node>,
}

#[derive(Debug, Deserialize)]
struct Remote {
    label: String,
    endpoint: String,
    working_directory: String,
}

#[derive(Debug, Deserialize)]
struct Node {
    id: String,
    #[serde(default)]
    remote: Option<String>,
    #[serde(default)]
    build: Option<String>,
    #[serde(default)]
    distribute: Option<String>,
    #[serde(default)]
    run: Option<String>,
    inputs: Vec<String>,
    outputs: Vec<String>,
}

pub async fn read_toml_and_parse_to_application(path: PathBuf) -> eyre::Result<Application> {
    let contents = tokio::fs::read_to_string(path.clone()).await?;
    let contents_toml: Config = toml::from_str(&contents)?;
    let narr = contents_toml.narr;

    let mut application = Application::new(narr.name);

    let local_daemon = narr_core::daemon::Daemon::new(
        DaemonAddress::from_string(narr.local)?,
        "LOCAL".to_string(),
        PathBuf::from(narr.working_directory),
    );

    let mut remote_daemons = HashMap::new();
    for remote in narr.remote.unwrap_or_default() {
        let daemon = narr_core::daemon::Daemon::new(
            DaemonAddress::from_string(remote.endpoint)?,
            remote.label.clone(),
            PathBuf::from(remote.working_directory),
        );

        application.add_daemon(daemon.clone());
        remote_daemons.insert(remote.label, daemon);
    }

    application.add_daemon(local_daemon.clone());

    for node in narr.node {
        let daemon = match node.remote {
            Some(remote) => remote_daemons
                .get(&remote)
                .cloned()
                .ok_or_eyre(eyre::eyre!("Remote daemon not found"))?,
            None => local_daemon.clone(),
        };

        let node = narr_core::node::Node {
            id: node.id,
            daemon,
            inputs: narr_core::node::inputs::NodeInputs { ids: node.inputs },
            outputs: narr_core::node::outputs::NodeOutputs { ids: node.outputs },
        };

        application.add_node(node);
    }

    Ok(application)
}
