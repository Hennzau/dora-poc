use crate::daemon::{Daemon, DaemonLabel};

pub mod inputs;
pub mod outputs;

pub type NodeId = String;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,

    pub daemon: Daemon,

    pub inputs: inputs::NodeInputs,
    pub outputs: outputs::NodeOutputs,
}
