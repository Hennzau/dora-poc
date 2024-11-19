use core::fmt;

#[derive(Debug, Clone)]
pub struct DaemonAddress {
    pub protocol: String,
    pub address: String,
    pub port: u16,
}

impl DaemonAddress {
    pub fn new(protocol: String, address: String, port: u16) -> Self {
        DaemonAddress {
            protocol,
            address,
            port,
        }
    }

    pub fn from_string(address: String) -> eyre::Result<Self> {
        let parts: Vec<&str> = address.split('/').collect();
        if parts.len() != 2 {
            return Err(eyre::eyre!("Invalid address format"));
        }

        let protocol = parts[0].to_string();
        let parts: Vec<&str> = parts[1].split(':').collect();
        if parts.len() != 2 {
            return Err(eyre::eyre!("Invalid address format"));
        }

        let address = parts[0].to_string();
        let port = parts[1].parse::<u16>()?;

        Ok(DaemonAddress {
            protocol,
            address,
            port,
        })
    }
}

impl fmt::Display for DaemonAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}:{}", self.protocol, self.address, self.port)
    }
}
