use std::collections::HashMap;

use bc_components::ARID;

use crate::{impl_attachable, NoQuotesDebugOption};
use super::Network;

use super::{Account, Attachments, Identifiable, SeedMaterial};

/// A complete Zcash wallet with multiple accounts and cryptographic key material.
///
/// `ZewifWallet` represents an entire wallet consisting of multiple accounts, all operating
/// on the same Zcash network. It can optionally include seed material for generating keys.
/// This structure is the primary container for user wallet data but is not the top level
/// of the interchange format hierarchy (that's `ZewifTop`).
///
/// # Zcash Concept Relation
///
/// In Zcash wallet architecture:
///
/// - **Wallet Identity**: Each wallet has a unique identifier (ARID) for consistent
///   reference across migrations
/// - **Network Context**: Wallets operate within a specific Zcash network environment
///   (mainnet, testnet, regtest)
/// - **Multi-Account Organization**: A single wallet can contain multiple accounts
///   for different purposes or users
/// - **HD Wallet Structure**: When a seed is present, the wallet follows hierarchical
///   deterministic (HD) principles for key derivation
///
/// # Data Preservation
///
/// During wallet migration, the following wallet data must be preserved:
///
/// - **Identity**: The wallet's unique ARID identifier
/// - **Network**: The Zcash network context (mainnet, testnet, regtest)
/// - **Seed Material**: When available, the cryptographic material used for key generation
/// - **Accounts**: All accounts contained within the wallet, with their full structure
/// - **Vendor-Specific Information**: Custom metadata stored in attachments
///
/// # Examples
/// ```no_run
/// use zewif::{ZewifWallet, Network, Account, SeedMaterial};
///
/// // Create a new wallet for the Zcash mainnet
/// let mut wallet = ZewifWallet::new(Network::Main);
///
/// // Add a new account to the wallet
/// let account = Account::new();
/// wallet.add_account(account);
///
/// // Access the wallet's network
/// assert_eq!(wallet.network(), Network::Main);
/// 
/// // If seed material were available, you could add it:
/// // wallet.set_seed_material(seed_material);
/// ```
#[derive(Clone)]
pub struct ZewifWallet {
    id: ARID,
    network: Network,
    seed_material: Option<SeedMaterial>,
    accounts: HashMap<ARID, Account>,
    attachments: Attachments,
}

impl std::fmt::Debug for ZewifWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZewifWallet")
            .field("id", &self.id)
            .field("network", &self.network)
            .field("seed_material", &NoQuotesDebugOption(&self.seed_material))
            .field("accounts", &self.accounts)
            .field("attachments", &self.attachments)
            .finish()
    }
}

impl Identifiable for ZewifWallet {
    fn id(&self) -> ARID {
        self.id
    }
}

impl_attachable!(ZewifWallet);

impl ZewifWallet {
    pub fn new(network: Network) -> Self {
        Self {
            id: ARID::new(),
            network,
            seed_material: None,
            accounts: HashMap::new(),
            attachments: Attachments::new(),
        }
    }

    pub fn id(&self) -> ARID {
        self.id
    }

    pub fn network(&self) -> Network {
        self.network
    }

    pub fn seed_material(&self) -> Option<&SeedMaterial> {
        self.seed_material.as_ref()
    }

    pub fn set_seed_material(&mut self, seed_material: SeedMaterial) {
        self.seed_material = Some(seed_material);
    }

    pub fn accounts(&self) -> &HashMap<ARID, Account> {
        &self.accounts
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.insert(account.id(), account);
    }
}
