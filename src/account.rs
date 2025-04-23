use anyhow::{Context, Result};
use bc_envelope::prelude::*;
use std::collections::HashSet;

use crate::{
    envelope_indexed_objects_for_predicate, test_envelope_roundtrip, Indexed, NoQuotesDebugOption
};

use super::{Address, OrchardSentOutput, TxId, sapling::SaplingSentOutput};

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
/// # use zewif::{Account, Address, TxId, ProtocolAddress, transparent};
/// #
/// // Create a new account
/// let mut account = Account::new();
///
/// // Set account properties
/// account.set_name("Savings");
/// account.set_zip32_account_id(0);
///
/// // In practice, you would have real addresses and transaction IDs
/// // Create a transparent address (as an example)
/// let t_addr = transparent::Address::new("t1ExampleAddress".to_string());
/// let address = Address::new(ProtocolAddress::Transparent(t_addr));
/// let txid = TxId::from_bytes([0u8; 32]);
///
/// // Add the address and transaction to the account
/// account.add_address(address);
/// account.add_relevant_transaction(txid);
/// ```
#[derive(Clone, PartialEq)]
pub struct Account {
    index: usize,

    // User-defined, may not be unique.
    name: String,

    zip32_account_id: Option<u32>,
    addresses: Vec<Address>,

    // Subset of the global transaction history.
    relevant_transactions: HashSet<TxId>,

    // The following are intended for storage of information that may not be
    // recoverable from the chain.
    sapling_sent_outputs: Vec<SaplingSentOutput>,
    orchard_sent_outputs: Vec<OrchardSentOutput>,
    attachments: Attachments,
}

#[rustfmt::skip]
impl std::fmt::Debug for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Account")
            .field("index", &self.index)
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

bc_envelope::impl_attachable!(Account);

impl Indexed for Account {
    fn index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}

impl Account {
    pub fn new() -> Self {
        Self {
            index: 0,
            name: String::default(),
            zip32_account_id: None,
            addresses: Vec::new(),
            relevant_transactions: HashSet::new(),
            sapling_sent_outputs: Vec::new(),
            orchard_sent_outputs: Vec::new(),
            attachments: Attachments::new(),
        }
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

    pub fn addresses(&self) -> &Vec<Address> {
        &self.addresses
    }

    pub fn addresses_len(&self) -> usize {
        self.addresses.len()
    }

    pub fn add_address(&mut self, mut address: Address) {
        address.set_index(self.addresses.len());
        self.addresses.push(address);
    }

    pub fn relevant_transactions(&self) -> &HashSet<TxId> {
        &self.relevant_transactions
    }

    pub fn relevant_transactions_len(&self) -> usize {
        self.relevant_transactions.len()
    }

    pub fn add_relevant_transaction(&mut self, txid: TxId) {
        self.relevant_transactions.insert(txid);
    }

    pub fn sapling_sent_outputs(&self) -> &Vec<SaplingSentOutput> {
        &self.sapling_sent_outputs
    }

    pub fn sapling_sent_outputs_len(&self) -> usize {
        self.sapling_sent_outputs.len()
    }

    pub fn add_sapling_sent_output(&mut self, mut output: SaplingSentOutput) {
        output.set_index(self.sapling_sent_outputs.len());
        self.sapling_sent_outputs.push(output);
    }

    pub fn orchard_sent_outputs(&self) -> &Vec<OrchardSentOutput> {
        &self.orchard_sent_outputs
    }

    pub fn orchard_sent_outputs_len(&self) -> usize {
        self.orchard_sent_outputs.len()
    }

    pub fn add_orchard_sent_output(&mut self, mut output: OrchardSentOutput) {
        output.set_index(self.orchard_sent_outputs.len());
        self.orchard_sent_outputs.push(output);
    }
}

impl Default for Account {
    fn default() -> Self {
        Self::new()
    }
}

#[rustfmt::skip]
impl From<Account> for Envelope {
    fn from(value: Account) -> Self {
        let mut e = Envelope::new(value.index)
            .add_type("Account")
            .add_assertion("name", value.name)
            .add_optional_assertion("zip32_account_id", value.zip32_account_id)
            .add_assertion("relevant_transactions", value.relevant_transactions.sort_by_cbor_encoding()); // Deterministic ordering

        e = value.addresses.iter().fold(e, |e, address| e.add_assertion("address", address.clone()));
        e = value.sapling_sent_outputs.iter().fold(e, |e, output| e.add_assertion("sapling_sent_output", output.clone()));
        e = value.orchard_sent_outputs.iter().fold(e, |e, output| e.add_assertion("orchard_sent_output", output.clone()));

        value.attachments.add_to_envelope(e)
    }
}

#[rustfmt::skip]
impl TryFrom<Envelope> for Account {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self> {
        envelope.check_type_envelope("Account").context("account")?;
        let index = envelope.extract_subject().context("index")?;
        let name = envelope.extract_object_for_predicate("name").context("name")?;
        let zip32_account_id = envelope.extract_optional_object_for_predicate("zip32_account_id").context("zip32_account_id")?;
        let relevant_transactions = envelope.extract_object_for_predicate("relevant_transactions").context("relevant_transactions")?;

        let addresses = envelope_indexed_objects_for_predicate(&envelope, "address").context("addresses")?;
        let sapling_sent_outputs = envelope_indexed_objects_for_predicate(&envelope, "sapling_sent_output").context("sapling_sent_outputs")?;
        let orchard_sent_outputs = envelope_indexed_objects_for_predicate(&envelope, "orchard_sent_output").context("orchard_sent_outputs")?;

        let attachments = Attachments::try_from_envelope(&envelope).context("attachments")?;

        Ok(Self {
            index,
            name,
            zip32_account_id,
            addresses,
            relevant_transactions,
            sapling_sent_outputs,
            orchard_sent_outputs,
            attachments,
        })
    }
}

#[cfg(test)]
impl crate::RandomInstance for Account {
    fn random() -> Self {
        use crate::SetIndexes;

        Self {
            index: 0,
            name: String::random(),
            zip32_account_id: u32::opt_random(),
            addresses: Vec::random().set_indexes(),
            relevant_transactions: HashSet::random(),
            sapling_sent_outputs: Vec::random().set_indexes(),
            orchard_sent_outputs: Vec::random().set_indexes(),
            attachments: Attachments::random(),
        }
    }
}

test_envelope_roundtrip!(Account);
