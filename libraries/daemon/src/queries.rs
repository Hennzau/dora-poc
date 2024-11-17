use rkyv::{util::AlignedVec, Archive, Deserialize, Serialize};

#[derive(Archive, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DaemonQuery {
    Check,
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
pub enum DaemonReply {
    Ok(String),
    Test,
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
