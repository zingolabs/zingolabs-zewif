use anyhow::{Result, bail};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Network {
    /// Zcash Mainnet.
    Main,
    /// Zcash Testnet.
    Test,
    /// Private integration / regression testing, used in `zcashd`.
    ///
    /// For some address types there is no distinction between test and regtest encodings;
    /// those will always be parsed as `Network::Test`.
    Regtest,
}

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
