use std::{collections::HashMap, path::PathBuf};

use address::DaemonAddress;

pub mod address;

pub type DaemonLabel = String;

pub type Network = HashMap<DaemonLabel, DaemonAddress>;

pub type Input = String;
pub type Output = String;

pub type NodeID = String;

#[derive(Debug, Clone)]
pub struct Node {
    pub files: HashMap<DaemonLabel, PathBuf>,
    pub start: String,

    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

pub type NodeInput = (NodeID, Input);
pub type NodeOutput = (NodeID, Output);

pub type Flows = HashMap<NodeInput, NodeOutput>;

pub type Distribution = HashMap<NodeID, DaemonLabel>;

#[derive(Debug, Clone)]
pub struct Application {
    pub id: String,
    pub network: Network,

    pub nodes: HashMap<NodeID, Node>,

    pub flows: Flows,

    pub distribution: Distribution,
}
