use std::collections::HashMap;

use bc_components::ARID;

use crate::impl_attachable;
use super::{Attachments, Transaction, TxId, ZewifWallet};

/// The top-level container for the Zcash Wallet Interchange Format (ZeWIF).
///
/// `ZewifTop` is the root structure of the ZeWIF hierarchy, serving as a container
/// for multiple wallets and a global transaction history. This structure represents
/// the entirety of the data that would be migrated between different Zcash wallet
/// implementations.
///
/// # Zcash Concept Relation
///
/// In the Zcash wallet ecosystem:
///
/// - **Interchange Container**: `ZewifTop` serves as the standardized format for
///   moving wallet data between different implementations
/// - **Multi-Wallet Support**: A single interchange file can contain multiple wallets,
///   each with its own accounts and configuration
/// - **Global Transaction History**: Transactions are stored at the top level and
///   referenced by accounts in wallets, avoiding duplication
/// - **Migration Target**: This structure is the complete package that can be
///   serialized/deserialized during wallet migration
///
/// # Data Preservation
///
/// During wallet migration, the ZeWIF top-level container preserves:
///
/// - **Complete Wallet Collection**: All wallets with their unique identities and configurations
/// - **Full Transaction Graph**: The complete transaction history across all wallets
/// - **Relationship Structure**: The connections between wallets, accounts, and transactions
/// - **Vendor-Specific Extensions**: Custom metadata through the attachments system
///
/// # Examples
/// ```no_run
/// use zewif::{ZewifTop, ZewifWallet, Network, Transaction, TxId};
///
/// // Create the top-level container
/// let mut zewif = ZewifTop::new();
///
/// // Add a wallet
/// let wallet = ZewifWallet::new(Network::Main);
/// zewif.add_wallet(wallet);
///
/// // Add a transaction to the global history
/// let txid = TxId::from_bytes([0u8; 32]); // In practice, a real transaction ID
/// let tx = Transaction::new(txid);
/// zewif.add_transaction(txid, tx);
///
/// // Access transactions
/// let tx_count = zewif.transactions().len();
/// ```
#[derive(Debug, Clone)]
pub struct ZewifTop {
    wallets: HashMap<ARID, ZewifWallet>,
    transactions: HashMap<TxId, Transaction>,
    attachments: Attachments,
}

impl_attachable!(ZewifTop);

impl ZewifTop {
    pub fn new() -> Self {
        Self {
            wallets: HashMap::new(),
            transactions: HashMap::new(),
            attachments: Attachments::new(),
        }
    }

    pub fn wallets(&self) -> &HashMap<ARID, ZewifWallet> {
        &self.wallets
    }

    pub fn add_wallet(&mut self, wallet: ZewifWallet) {
        self.wallets.insert(wallet.id(), wallet);
    }

    pub fn transactions(&self) -> &HashMap<TxId, Transaction> {
        &self.transactions
    }

    pub fn transactions_mut(&mut self) -> &mut HashMap<TxId, Transaction> {
        &mut self.transactions
    }

    pub fn add_transaction(&mut self, txid: TxId, transaction: Transaction) {
        self.transactions.insert(txid, transaction);
    }

    pub fn get_transaction(&self, txid: TxId) -> Option<&Transaction> {
        self.transactions.get(&txid)
    }

    pub fn set_transactions(&mut self, transactions: HashMap<TxId, Transaction>) {
        self.transactions = transactions;
    }
}

impl Default for ZewifTop {
    fn default() -> Self {
        Self::new()
    }
}
