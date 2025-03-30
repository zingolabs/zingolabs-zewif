use anyhow::{Result, bail};

/// Represents a Zcash network environment (mainnet, testnet, or regtest).
///
/// The `Network` enum identifies which Zcash network a wallet, address,
/// or transaction belongs to. Each network has different consensus rules,
/// address encodings, and initial blockchain parameters.
///
/// # Zcash Concept Relation
/// Zcash, like Bitcoin, operates on multiple networks:
///
/// - **Mainnet**: The primary Zcash network where real ZEC with monetary value is transferred
/// - **Testnet**: A testing network that simulates mainnet but uses worthless test coins
/// - **Regtest**: A private "regression test" network for local development and testing
///
/// These networks are isolated from each other, with different genesis blocks,
/// address formats, and consensus parameters.
///
/// # Data Preservation
/// The `Network` value is critical during wallet migration to ensure addresses and
/// transactions are reconstructed for the correct network. Address formats differ 
/// between networks, and migrating a wallet to an incorrect network would render 
/// it unusable.
///
/// # Examples
/// In the ZeWIF format, the Network value is stored at the wallet level:
/// ```
/// use zewif::{ZewifWallet, Network};
///
/// // Wallet on the main Zcash network
/// let network = Network::Main;
///
/// // Wallets on mainnet and testnet have incompatible address formats
/// match network {
///     Network::Main => println!("This wallet stores real ZEC"),
///     Network::Test => println!("This wallet stores test coins only"),
///     Network::Regtest => println!("This wallet is for local testing"),
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Network {
    /// Zcash Mainnet.
    /// The production network where ZEC with actual value is transferred.
    Main,
    
    /// Zcash Testnet.
    /// A public testing network with worthless coins for development.
    Test,
    
    /// Private integration / regression testing, used in `zcashd`.
    ///
    /// For some address types there is no distinction between test and regtest encodings;
    /// those will always be parsed as `Network::Test`.
    ///
    /// Regtest allows developers to create a private blockchain for testing,
    /// with immediate block generation on demand.
    Regtest,
}

/// Converts a network name string into a `Network` enum value.
///
/// This function parses common network identifier strings ("main", "test", "regtest")
/// and returns the corresponding `Network` enum value.
///
/// # Arguments
/// * `identifier` - A string representing the network name
///
/// # Returns
/// A `Result<Network>` containing the parsed network or an error
///
/// # Errors
/// Returns an error if the provided identifier is not one of the recognized
/// network names ("main", "test", or "regtest").
///
/// # Examples
/// ```
/// use zewif::Network;
/// use zewif::network_for_identifier;
/// use anyhow::Result;
///
/// # fn example() -> Result<()> {
/// // Parse network from configuration string
/// let network = network_for_identifier("main")?;
/// assert_eq!(network, Network::Main);
/// # Ok(())
/// # }
/// ```
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
