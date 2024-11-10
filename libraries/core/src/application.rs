use std::collections::HashMap;

use crate::{
    daemon::{Daemon, DaemonLabel},
    node::{Node, NodeId},
};

#[derive(Debug, Clone)]
pub struct Application {
    pub id: String,

    pub daemons: HashMap<DaemonLabel, Daemon>,
    pub nodes: HashMap<NodeId, Node>,
}

#[derive(Debug, Clone)]
pub struct ApplicationGraph {
    pub id: String,
    pub daemons: HashMap<DaemonLabel, Daemon>,
    pub nodes: HashMap<DaemonLabel, Vec<Node>>,
}

impl Application {
    pub fn new(id: String) -> Self {
        Application {
            id,
            daemons: HashMap::new(),
            nodes: HashMap::new(),
        }
    }

    pub fn add_daemon(&mut self, machine: Daemon) {
        self.daemons.insert(machine.label.clone(), machine);
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn build_graph(&self) -> ApplicationGraph {
        let mut graph = ApplicationGraph {
            id: self.id.clone(),
            daemons: self.daemons.clone(),
            nodes: HashMap::new(),
        };

        for node in self.nodes.values() {
            let daemon_name = node.daemon.label.clone();
            let nodes = graph.nodes.entry(daemon_name).or_insert(vec![]);
            nodes.push(node.clone());
        }

        graph
    }
}
