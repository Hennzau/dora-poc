use core::fmt;
use std::path::PathBuf;

pub mod address;

pub type DaemonName = String;

#[derive(Debug, Clone)]
pub struct Daemon {
    pub label: DaemonName,

    pub address: address::MachineAddress,
    pub working_dir: PathBuf,
}

impl Daemon {
    pub fn new(address: address::MachineAddress, name: String, working_dir: PathBuf) -> Self {
        Daemon {
            address,
            label: name,
            working_dir,
        }
    }
}

impl fmt::Display for Daemon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.label, self.address)
    }
}
