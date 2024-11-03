use std::path::PathBuf;

use eyre::OptionExt;
use narr_core::{
    application::Application,
    machine::{address::MachineAddress, Machine},
    node::{inputs::NodeInputs, outputs::NodeOutputs, Node},
};

pub async fn read_toml_and_parse_to_application(path: PathBuf) -> eyre::Result<Application> {
    let contents = tokio::fs::read_to_string(path.clone()).await?;
    let contents_toml: toml::Value = toml::from_str(&contents)?;

    let metadata = contents_toml
        .get("narr")
        .ok_or_eyre(format!("Missing 'narr' key in {:?}", path))?;

    let name = metadata
        .get("name")
        .ok_or_eyre(format!("Missing 'name' key in app metadata {:?}", path))?
        .as_str()
        .ok_or_eyre(format!("Invalid 'name' value in app metadata {:?}", path))?;

    let id = rand::random::<u64>();

    let mut application = Application::new(format!("{}-{}", name, id));

    let machines = contents_toml
        .get("machine")
        .ok_or_eyre(format!("Missing 'machine' key in {:?}", path))?;

    let nodes = contents_toml
        .get("node")
        .ok_or_eyre(format!("Missing 'node' key in {:?}", path))?;

    for machine in machines
        .as_array()
        .ok_or_eyre(format!("Invalid 'machine' value in {:?}", path))?
    {
        let name = machine
            .get("name")
            .ok_or_eyre("Missing 'name' key in machine")?
            .as_str()
            .ok_or_eyre("Invalid 'name' value in machine")?
            .to_string();

        let address = machine
            .get("address")
            .ok_or_eyre(format!("Missing 'address' key in machine {}", name))?
            .as_str()
            .ok_or_eyre(format!("Invalid 'address' value in machine {}", name))?;

        let address = MachineAddress::from_string(address.to_string())?;

        let working_dir = machine
            .get("working_directory")
            .ok_or_eyre(format!(
                "Missing 'working_directory' key in machine {}",
                name
            ))?
            .as_str()
            .ok_or_eyre(format!(
                "Invalid 'working_directory' value in machine {}",
                name
            ))?;

        let working_dir = PathBuf::from(working_dir);

        application.add_machine(Machine::new(address, name, working_dir));
    }

    for node in nodes
        .as_array()
        .ok_or_eyre(format!("Invalid 'node' value in {:?}", path))?
    {
        let id = node
            .get("id")
            .ok_or_eyre("Missing 'id' key in node")?
            .as_str()
            .ok_or_eyre("Invalid 'id' value in node")?
            .to_string();

        let machine_name = node
            .get("machine")
            .ok_or_eyre(format!("Missing 'machine' key in node {}", id))?
            .as_str()
            .ok_or_eyre(format!("Invalid 'machine' value in node {}", id))?
            .to_string();

        let machine = application
            .machines
            .get(&machine_name)
            .ok_or_eyre(format!("Machine {} not found", machine_name))?;

        let inputs = node
            .get("inputs")
            .ok_or_eyre(format!("Missing 'inputs' key in node {}", id))?
            .as_array()
            .ok_or_eyre(format!("Invalid 'inputs' value in node {}", id))?;

        let outputs = node
            .get("outputs")
            .ok_or_eyre(format!("Missing 'outputs' key in node {}", id))?
            .as_array()
            .ok_or_eyre(format!("Invalid 'outputs' value in node {}", id))?;

        let mut inputs_vec = Vec::new();
        for input in inputs {
            let input = input
                .as_str()
                .ok_or_eyre(format!("Invalid 'input' value in node {}", id))?
                .to_string();
            inputs_vec.push(input);
        }

        let mut outputs_vec = Vec::new();
        for output in outputs {
            let output = output
                .as_str()
                .ok_or_eyre(format!("Invalid 'output' value in node {}", id))?
                .to_string();
            outputs_vec.push(output);
        }

        application.add_node(Node {
            id,
            machine: machine.clone(),
            inputs: NodeInputs { ids: inputs_vec },
            outputs: NodeOutputs { ids: outputs_vec },
        });
    }

    Ok(application)
}
