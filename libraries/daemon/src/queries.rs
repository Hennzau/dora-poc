use rkyv::{util::AlignedVec, Archive, Deserialize, Serialize};

#[derive(Archive, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DaemonQuery {
    Check,
    CheckFile(String),
}

impl DaemonQuery {
    pub fn to_bytes(self) -> eyre::Result<AlignedVec> {
        rkyv::to_bytes::<rkyv::rancor::Error>(&self).map_err(eyre::Report::msg)
    }

    pub fn from_bytes(bytes: &[u8]) -> eyre::Result<DaemonQuery> {
        let archived = rkyv::access::<ArchivedDaemonQuery, rkyv::rancor::Error>(&bytes[..])
            .map_err(eyre::Report::msg)?;

        rkyv::deserialize::<DaemonQuery, rkyv::rancor::Error>(archived).map_err(eyre::Report::msg)
    }
}

#[derive(Archive, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InfoReply {
    pub id: String,
    pub reachable: String,
}

#[derive(Archive, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DaemonReply {
    Ok(InfoReply),
    FileOk,
    FileNotFound,
}

impl DaemonReply {
    pub fn to_bytes(self) -> eyre::Result<AlignedVec> {
        rkyv::to_bytes::<rkyv::rancor::Error>(&self).map_err(eyre::Report::msg)
    }

    pub fn from_bytes(bytes: &[u8]) -> eyre::Result<DaemonReply> {
        let archived = rkyv::access::<ArchivedDaemonReply, rkyv::rancor::Error>(&bytes[..])
            .map_err(eyre::Report::msg)?;

        rkyv::deserialize::<DaemonReply, rkyv::rancor::Error>(archived).map_err(eyre::Report::msg)
    }
}

#[derive(Archive, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DataFlowQuery {
    Test,
}

impl DataFlowQuery {
    pub fn to_bytes(self) -> eyre::Result<AlignedVec> {
        rkyv::to_bytes::<rkyv::rancor::Error>(&self).map_err(eyre::Report::msg)
    }

    pub fn from_bytes(bytes: &[u8]) -> eyre::Result<DataFlowQuery> {
        let archived = rkyv::access::<ArchivedDataFlowQuery, rkyv::rancor::Error>(&bytes[..])
            .map_err(eyre::Report::msg)?;

        rkyv::deserialize::<DataFlowQuery, rkyv::rancor::Error>(archived).map_err(eyre::Report::msg)
    }
}

#[derive(Archive, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DataFlowReply {
    Test,
}

impl DataFlowReply {
    pub fn to_bytes(self) -> eyre::Result<AlignedVec> {
        rkyv::to_bytes::<rkyv::rancor::Error>(&self).map_err(eyre::Report::msg)
    }

    pub fn from_bytes(bytes: &[u8]) -> eyre::Result<DataFlowReply> {
        let archived = rkyv::access::<ArchivedDataFlowReply, rkyv::rancor::Error>(&bytes[..])
            .map_err(eyre::Report::msg)?;

        rkyv::deserialize::<DataFlowReply, rkyv::rancor::Error>(archived).map_err(eyre::Report::msg)
    }
}
