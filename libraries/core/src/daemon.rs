use core::fmt;
use std::path::PathBuf;

use address::DaemonAddress;

pub mod address;

pub type DaemonLabel = String;

#[derive(Debug, Clone)]
pub struct Daemon {
    pub label: DaemonLabel,

    pub address: DaemonAddress,
    pub working_dir: PathBuf,
}

impl Daemon {
    pub fn new(address: address::DaemonAddress, label: String, working_dir: PathBuf) -> Self {
        Daemon {
            address,
            label,
            working_dir,
        }
    }
}

impl fmt::Display for Daemon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.label, self.address)
    }
}
