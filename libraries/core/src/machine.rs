use core::fmt;
use std::path::PathBuf;

pub mod address;

pub type MachineName = String;

#[derive(Debug, Clone)]
pub struct Machine {
    pub address: address::MachineAddress,
    pub name: MachineName,

    pub working_dir: PathBuf,
}

impl Machine {
    pub fn new(address: address::MachineAddress, name: String, working_dir: PathBuf) -> Self {
        Machine {
            address,
            name,
            working_dir,
        }
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.address)
    }
}
