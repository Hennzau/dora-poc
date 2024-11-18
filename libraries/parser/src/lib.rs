use narr_core::address::DaemonAddress;
use narr_core::{Application, Distribution, Flows, Network, Node, NodeID};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ApplicationConfig {
    pub application: String,
    pub network: HashMap<String, String>,
    pub vars: Option<HashMap<String, String>>,
    pub nodes: Vec<NodeConfig>,
    pub flows: HashMap<String, String>,
    pub distributed: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct NodeConfig {
    pub id: String,
    #[serde(default)]
    pub files: HashMap<String, String>,
    #[serde(default)]
    pub start: String,
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(default)]
    pub outputs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct FileConfig {
    #[serde(flatten)]
    pub mapping: HashMap<String, String>,
}

impl ApplicationConfig {
    pub fn parse_yaml(yaml_content: &str) -> eyre::Result<Self> {
        // Replace variables in the YAML string before parsing
        let processed_yaml = Self::replace_variables(yaml_content)?;

        // Parse the processed YAML
        let config: ApplicationConfig = serde_yaml::from_str(&processed_yaml)?;
        Ok(config)
    }

    fn replace_variables(yaml_content: &str) -> eyre::Result<String> {
        // First, parse the YAML to extract vars
        let yaml: serde_yaml::Value = serde_yaml::from_str(yaml_content)?;

        // Extract vars from the YAML
        let vars: HashMap<String, String> = yaml
            .get("vars")
            .and_then(|v| v.as_mapping())
            .map(|mapping| {
                mapping
                    .iter()
                    .filter_map(|(k, v)| {
                        k.as_str().and_then(|key| {
                            v.as_str().map(|val| (key.to_string(), val.to_string()))
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Create a regex to find variable placeholders
        let var_regex = regex::Regex::new(r"\{\{([^}]+)\}\}")?;

        // Replace variables in the YAML content
        let processed_yaml = var_regex.replace_all(yaml_content, |caps: &regex::Captures| {
            let var_name = caps.get(1).map_or("", |m| m.as_str());
            vars.get(var_name)
                .cloned()
                .unwrap_or_else(|| format!("{{{{{}}}}}", var_name))
        });

        Ok(processed_yaml.into_owned())
    }

    // Convert to the original Application struct
    pub fn into_application(self) -> eyre::Result<Application> {
        // Convert network
        let mut network: Network = HashMap::new();

        for (label, address) in self.network.iter() {
            let daemon_address = DaemonAddress::from_string(address.clone())?;
            network.insert(label.clone(), daemon_address);
        }

        // Convert nodes
        let nodes: HashMap<NodeID, Node> = self
            .nodes
            .iter()
            .map(|node_config| {
                let files: HashMap<String, PathBuf> = node_config
                    .files
                    .iter()
                    .map(|(label, path)| (label.clone(), PathBuf::from(path)))
                    .collect();

                (
                    node_config.id.clone(),
                    crate::Node {
                        files,
                        start: node_config.start.clone(),
                        inputs: node_config.inputs.clone(),
                        outputs: node_config.outputs.clone(),
                    },
                )
            })
            .collect();

        // Convert flows
        let flows: Flows = self
            .flows
            .iter()
            .map(|(input, output)| {
                let input_split = input.split('/').collect::<Vec<_>>();
                let output_split = output.split('/').collect::<Vec<_>>();

                let (input_node, input_port) = input_split.split_at(1);
                let (output_node, output_port) = output_split.split_at(1);

                (
                    (input_node[0].to_string(), input_port[0].to_string()),
                    (output_node[0].to_string(), output_port[0].to_string()),
                )
            })
            .collect();

        // Convert distribution
        let distributed: Distribution = self.distributed.clone();

        Ok(Application {
            id: self.application.clone(),
            network,
            nodes,
            flows,
            distributed,
        })
    }
}

pub async fn parse_application(application: String) -> eyre::Result<Application> {
    let config = ApplicationConfig::parse_yaml(&application)?;

    config.into_application()
}

pub async fn read_and_parse_application(file_path: PathBuf) -> eyre::Result<Application> {
    let file_content = tokio::fs::read_to_string(file_path.clone()).await?;

    parse_application(file_content).await
}
