use crate::machine::Machine;

pub mod inputs;
pub mod outputs;

pub type NodeId = String;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub machine: Machine,

    pub inputs: inputs::NodeInputs,
    pub outputs: outputs::NodeOutputs,
}
