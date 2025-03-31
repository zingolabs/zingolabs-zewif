use std::collections::{HashMap, HashSet};

use bc_components::ARID;

use crate::{impl_attachable, NoQuotesDebugOption};

use super::{
    Address, Attachments, OrchardSentOutput, 
    TxId,
    sapling::SaplingSentOutput,
};

/// A logical grouping of addresses and transaction history within a wallet.
///
/// `Account` represents a distinct subdivision of wallet functionality, similar to how
/// bank accounts work within a single banking relationship. Each account can have its own set of
/// addresses, relevant transactions, and sent output information that cannot be recovered
/// from the blockchain.
///
/// # Zcash Concept Relation
///
/// In Zcash's hierarchical deterministic (HD) wallet structure:
///
/// - **Account Subdivision**: Follows the BIP-44/ZIP-32 HD hierarchy model (m/44'/133'/account'/*), 
///   where each account represents a separate logical collection of funds
/// - **Isolation**: Different accounts provide isolation for different purposes or users
/// - **Consistent Identifiers**: Each account has a unique ARID (Apparently Random Identifier) 
///   to maintain consistent identity across migrations
/// - **ZIP-32 Integration**: When available, accounts may include ZIP-32 account indexes to 
///   maintain the hierarchical derivation structure
///
/// # Data Preservation
///
/// During wallet migration, the following account data must be preserved:
///
/// - **Identity**: The unique account identifier and human-readable name
/// - **Addresses**: All addresses associated with the account, including metadata
/// - **Transaction Relationships**: Which transactions are relevant to this account
/// - **Sent Output Information**: Data that can't be recovered from the blockchain, 
///   such as outgoing transaction metadata
/// - **Extended Key Information**: HD wallet derivation path information when available
///
/// # Examples
/// ```no_run
/// use zewif::{Account, Address, TxId, ProtocolAddress, TransparentAddress};
/// 
/// // Create a new account
/// let mut account = Account::new();
/// 
/// // Set account properties
/// account.set_name("Savings");
/// account.set_zip32_account_id(0);
/// 
/// // In practice, you would have real addresses and transaction IDs
/// // Create a transparent address (as an example)
/// let t_addr = TransparentAddress::new("t1ExampleAddress".to_string());
/// let address = Address::new(ProtocolAddress::Transparent(t_addr));
/// let txid = TxId::from_bytes([0u8; 32]);
/// 
/// // Add the address and transaction to the account
/// account.add_address(address);
/// account.add_relevant_transaction(txid);
/// ```
#[derive(Clone)]
pub struct Account {
    id: ARID,

    // User-defined, may not be unique.
    name: String,

    zip32_account_id: Option<u32>,
    addresses: HashMap<String, Address>,

    // Subset of the global transaction history.
    relevant_transactions: HashSet<TxId>,

    // The following are intended for storage of information that may not be
    // recoverable from the chain.
    sapling_sent_outputs: Vec<SaplingSentOutput>,
    orchard_sent_outputs: Vec<OrchardSentOutput>,
    attachments: Attachments,
}


impl std::fmt::Debug for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Account")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("zip32_account_id", &NoQuotesDebugOption(&self.zip32_account_id))
            .field("addresses", &self.addresses)
            .field("relevant_transactions", &self.relevant_transactions)
            .field("sapling_sent_outputs", &self.sapling_sent_outputs)
            .field("orchard_sent_outputs", &self.orchard_sent_outputs)
            .field("attachments", &self.attachments)
            .finish()
    }
}

impl_attachable!(Account);

impl Account {
    pub fn new() -> Self {
        Self {
            id: ARID::new(),
            name: String::default(),
            zip32_account_id: None,
            addresses: HashMap::new(),
            relevant_transactions: HashSet::new(),
            sapling_sent_outputs: Vec::new(),
            orchard_sent_outputs: Vec::new(),
            attachments: Attachments::new(),
        }
    }

    pub fn id(&self) -> ARID {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn zip32_account_id(&self) -> Option<u32> {
        self.zip32_account_id
    }

    pub fn set_zip32_account_id(&mut self, id: u32) {
        self.zip32_account_id = Some(id);
    }

    pub fn addresses(&self) -> &HashMap<String, Address> {
        &self.addresses
    }

    pub fn addresses_mut(&mut self) -> &mut HashMap<String, Address> {
        &mut self.addresses
    }

    pub fn add_address(&mut self, address: Address) {
        self.addresses.insert(address.as_string(), address);
    }

    pub fn relevant_transactions(&self) -> &HashSet<TxId> {
        &self.relevant_transactions
    }

    pub fn relevant_transactions_mut(&mut self) -> &mut HashSet<TxId> {
        &mut self.relevant_transactions
    }

    pub fn add_relevant_transaction(&mut self, txid: TxId) {
        self.relevant_transactions.insert(txid);
    }

    pub fn sapling_sent_outputs(&self) -> &Vec<SaplingSentOutput> {
        &self.sapling_sent_outputs
    }

    pub fn sapling_sent_outputs_mut(&mut self) -> &mut Vec<SaplingSentOutput> {
        &mut self.sapling_sent_outputs
    }

    pub fn add_sapling_sent_output(&mut self, output: SaplingSentOutput) {
        self.sapling_sent_outputs.push(output);
    }

    pub fn orchard_sent_outputs(&self) -> &Vec<OrchardSentOutput> {
        &self.orchard_sent_outputs
    }

    pub fn orchard_sent_outputs_mut(&mut self) -> &mut Vec<OrchardSentOutput> {
        &mut self.orchard_sent_outputs
    }

    pub fn add_orchard_sent_output(&mut self, output: OrchardSentOutput) {
        self.orchard_sent_outputs.push(output);
    }
}

impl Default for Account {
    fn default() -> Self {
        Self::new()
    }
}
