use rkyv::{util::AlignedVec, Archive, Deserialize, Serialize};

pub enum DaemonEvent {}

#[derive(Archive, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DaemonToDaemonMessage {
    StopAll,
}

impl DaemonToDaemonMessage {
    pub fn to_bytes(self) -> eyre::Result<AlignedVec> {
        rkyv::to_bytes::<rkyv::rancor::Error>(&self).map_err(eyre::Report::msg)
    }

    pub fn from_bytes(bytes: &[u8]) -> eyre::Result<DaemonToDaemonMessage> {
        let archived =
            rkyv::access::<ArchivedDaemonToDaemonMessage, rkyv::rancor::Error>(&bytes[..])
                .map_err(eyre::Report::msg)?;

        rkyv::deserialize::<DaemonToDaemonMessage, rkyv::rancor::Error>(archived)
            .map_err(eyre::Report::msg)
    }
}
