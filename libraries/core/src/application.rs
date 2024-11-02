use std::collections::HashMap;

use crate::{
    machine::{Machine, MachineName},
    node::{Node, NodeId},
};

#[derive(Debug, Clone)]
pub struct Application {
    pub id: String,
    pub machines: HashMap<MachineName, Machine>,
    pub nodes: HashMap<NodeId, Node>,
}

#[derive(Debug, Clone)]
pub struct ApplicationGraph {
    pub id: String,
    pub machines: HashMap<MachineName, Machine>,
    pub nodes: HashMap<MachineName, Vec<Node>>,
}

impl Application {
    pub fn new(id: String) -> Self {
        Application {
            id,
            machines: HashMap::new(),
            nodes: HashMap::new(),
        }
    }

    pub fn add_machine(&mut self, machine: Machine) {
        self.machines.insert(machine.name.clone(), machine);
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn build_graph(&self) -> ApplicationGraph {
        let mut graph = ApplicationGraph {
            id: self.id.clone(),
            machines: self.machines.clone(),
            nodes: HashMap::new(),
        };

        for node in self.nodes.values() {
            let machine_name = node.machine.name.clone();
            let nodes = graph.nodes.entry(machine_name).or_insert(vec![]);
            nodes.push(node.clone());
        }

        graph
    }
}
