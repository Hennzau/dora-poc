use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DaemonQuery {
    Check,
    CheckFile(String),
}

impl DaemonQuery {
    pub fn to_bytes(self) -> eyre::Result<Vec<u8>> {
        bincode::serialize(&self).map_err(eyre::Report::msg)
    }

    pub fn from_bytes(bytes: &[u8]) -> eyre::Result<DaemonQuery> {
        bincode::deserialize(&bytes).map_err(eyre::Report::msg)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InfoReply {
    pub id: String,
    pub reachable: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DaemonReply {
    Ok(InfoReply),
    FileOk,
    FileNotFound,
}

impl DaemonReply {
    pub fn to_bytes(self) -> eyre::Result<Vec<u8>> {
        bincode::serialize(&self).map_err(eyre::Report::msg)
    }

    pub fn from_bytes(bytes: &[u8]) -> eyre::Result<DaemonReply> {
        bincode::deserialize(&bytes).map_err(eyre::Report::msg)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DataFlowQuery {
    Test,
}

impl DataFlowQuery {
    pub fn to_bytes(self) -> eyre::Result<Vec<u8>> {
        bincode::serialize(&self).map_err(eyre::Report::msg)
    }

    pub fn from_bytes(bytes: &[u8]) -> eyre::Result<DataFlowQuery> {
        bincode::deserialize(&bytes).map_err(eyre::Report::msg)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DataFlowReply {
    Test,
}

impl DataFlowReply {
    pub fn to_bytes(self) -> eyre::Result<Vec<u8>> {
        bincode::serialize(&self).map_err(eyre::Report::msg)
    }

    pub fn from_bytes(bytes: &[u8]) -> eyre::Result<DataFlowReply> {
        bincode::deserialize(&bytes).map_err(eyre::Report::msg)
    }
}
