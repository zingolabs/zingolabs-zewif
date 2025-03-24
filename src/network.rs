use anyhow::{Result, bail};

pub type Network = zcash_address::Network;

pub fn network_for_identifier(identifier: &str) -> Result<Network> {
    if identifier == "main" {
        Ok(Network::Main)
    } else if identifier == "test" {
        Ok(Network::Test)
    } else if identifier == "regtest" {
        Ok(Network::Regtest)
    } else {
        bail!("Invalid network identifier: {}", identifier)
    }
}
