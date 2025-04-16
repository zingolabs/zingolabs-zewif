use super::{BlockHeight, Data, SecondsSinceEpoch, TxId, u256};
use super::{
    JoinSplitDescription, OrchardActionDescription, TxIn, TxOut,
    sapling::{SaplingOutputDescription, SaplingSpendDescription},
};
use crate::{
    Indexed, TransactionStatus, envelope_optional_indexed_objects_for_predicate,
    test_envelope_roundtrip,
};
use anyhow::{Context, Result};
use bc_envelope::prelude::*;

/// A Zcash transaction that can combine transparent and multiple shielded protocol components.
///
/// `Transaction` represents a complete Zcash transaction, which can include components from
/// transparent Bitcoin-style inputs/outputs as well as from any of the three Zcash shielded
/// protocols: Sprout, Sapling, and Orchard. The transaction structure preserves both blockchain
/// data (block height, timestamp) and the detailed components needed to represent the
/// transaction in a wallet format.
///
/// # Zcash Concept Relation
///
/// In Zcash:
///
/// - **Unified Transaction Format**: Transactions can seamlessly combine transparent and
///   different shielded protocols in a single operation
/// - **Multi-Protocol Support**: Zcash has evolved through multiple shielded protocols,
///   and transactions can include components from any of them:
///   - Transparent (Bitcoin-style public inputs/outputs)
///   - Sprout (original shielded protocol using JoinSplits)
///   - Sapling (improved shielded protocol with separate spends/outputs)
///   - Orchard (latest shielded protocol using unified actions)
/// - **Transaction Lifecycle**: Transactions go through stages (Pending, Confirmed, Failed, Abandoned)
///   that represent their status on the blockchain
///
/// # Data Preservation
///
/// During wallet migration, the following transaction data must be preserved:
///
/// - **Identity**: The unique transaction ID (txid)
/// - **Blockchain Context**: Block height, timestamp, block hash when available
/// - **Status Information**: Whether the transaction is pending, confirmed, failed, or abandoned
/// - **Raw Transaction**: Optional full binary transaction data
/// - **Protocol-Specific Components**:
///   - Transparent inputs and outputs
///   - Sapling spends and outputs
///   - Orchard actions
///   - Sprout JoinSplits
///
/// # Examples
/// ```no_run
/// # use zewif::{Transaction, TxId, TransactionStatus, BlockHeight};
/// // Create a new transaction with a transaction ID (in practice, a real ID)
/// let txid = TxId::from_bytes([0u8; 32]);
/// let mut tx = Transaction::new(txid);
///
/// // Set transaction metadata
/// tx.set_status(TransactionStatus::Confirmed);
/// tx.set_mined_height(BlockHeight::from(1000000));
///
/// // In a real application, you would add transaction inputs and outputs
/// // tx.add_input(...);
/// // tx.add_output(...);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    /// The transaction id.
    txid: TxId,
    /// The raw transaction data, if known.
    raw: Option<Data>,
    /// The height at which the transaction was mined, if known.
    /// It is possible that if a rollback occurred just after the zeWIF
    /// export, the transaction could have been unmined, and possibly
    /// remined at a different height.
    mined_height: Option<BlockHeight>,
    /// The timestamp of the transaction, if known.
    timestamp: Option<SecondsSinceEpoch>,
    /// The status of the transaction.
    status: Option<TransactionStatus>,
    /// The hash of the block containing the transaction, if known.
    block_hash: Option<u256>,

    // Design issue: do we want to parse out all of this? All wallets will
    // necessarily have code to parse a transaction. The only information that
    // is not redundant with the raw transaction encoding is the *decrypted*
    // note plaintexts (and it might be sufficient to just indicate which output
    // indices are expected to be decryptable with which keys). I don't see the
    // point of duplicating the raw data in a different format (that still needs
    // to be parsed!) -- Daira-Emma
    //
    // Not all wallets (including `zcashd`) will have the raw transaction
    // encoding, and since `zewif` is focused on data migration, not
    // transformation, we need to allow for preserving either the raw
    // transaction or the parsed data, or both. Wallet exporters will need to be
    // able to handle both cases. -- Wolf
    /// Optional data for transparent inputs
    inputs: Option<Vec<TxIn>>,
    /// Optional data for transparent outputs
    outputs: Option<Vec<TxOut>>,
    /// Optional data for Sapling spends
    sapling_spends: Option<Vec<SaplingSpendDescription>>,
    /// Optional data for Sapling outputs
    sapling_outputs: Option<Vec<SaplingOutputDescription>>,
    /// Optional data for Orchard actions
    orchard_actions: Option<Vec<OrchardActionDescription>>,
    /// Optional data for Sprout JoinSplit descriptions
    sprout_joinsplits: Option<Vec<JoinSplitDescription>>,
    // Additional metadata such as confirmations or timestamp may be added here.
    attachments: Attachments,
}

bc_envelope::impl_attachable!(Transaction);

impl Transaction {
    pub fn new(txid: TxId) -> Self {
        Self {
            txid,
            raw: None,
            mined_height: None,
            timestamp: None,
            status: None,
            block_hash: None,
            inputs: None,
            outputs: None,
            sapling_spends: None,
            sapling_outputs: None,
            orchard_actions: None,
            sprout_joinsplits: None,
            attachments: Attachments::new(),
        }
    }

    pub fn txid(&self) -> TxId {
        self.txid
    }

    pub fn set_txid(&mut self, txid: TxId) {
        self.txid = txid;
    }

    pub fn raw(&self) -> Option<&Data> {
        self.raw.as_ref()
    }

    pub fn set_raw(&mut self, raw: Data) {
        self.raw = Some(raw);
    }

    pub fn mined_height(&self) -> Option<&BlockHeight> {
        self.mined_height.as_ref()
    }

    pub fn set_mined_height(&mut self, height: BlockHeight) {
        self.mined_height = Some(height);
    }

    pub fn timestamp(&self) -> Option<&SecondsSinceEpoch> {
        self.timestamp.as_ref()
    }

    pub fn set_timestamp(&mut self, timestamp: SecondsSinceEpoch) {
        self.timestamp = Some(timestamp);
    }

    pub fn status(&self) -> Option<&TransactionStatus> {
        self.status.as_ref()
    }

    pub fn set_status(&mut self, status: TransactionStatus) {
        self.status = Some(status);
    }

    pub fn block_hash(&self) -> Option<&u256> {
        self.block_hash.as_ref()
    }

    pub fn set_block_hash(&mut self, hash: u256) {
        self.block_hash = Some(hash);
    }

    pub fn inputs(&self) -> Option<&Vec<TxIn>> {
        self.inputs.as_ref()
    }

    pub fn inputs_len(&self) -> usize {
        self.inputs.as_ref().map_or(0, |v| v.len())
    }

    pub fn add_input(&mut self, mut input: TxIn) {
        input.set_index(self.inputs_len());
        self.inputs.get_or_insert_with(Vec::new).push(input);
    }

    pub fn outputs(&self) -> Option<&Vec<TxOut>> {
        self.outputs.as_ref()
    }

    pub fn outputs_len(&self) -> usize {
        self.outputs.as_ref().map_or(0, |v| v.len())
    }

    pub fn add_output(&mut self, mut output: TxOut) {
        output.set_index(self.outputs_len());
        self.outputs.get_or_insert_with(Vec::new).push(output);
    }

    pub fn sapling_spends(&self) -> Option<&Vec<SaplingSpendDescription>> {
        self.sapling_spends.as_ref()
    }

    pub fn sapling_spends_len(&self) -> usize {
        self.sapling_spends.as_ref().map_or(0, |v| v.len())
    }

    pub fn add_sapling_spend(&mut self, mut spend: SaplingSpendDescription) {
        spend.set_index(self.sapling_spends_len());
        self.sapling_spends.get_or_insert_with(Vec::new).push(spend);
    }

    pub fn sapling_outputs(&self) -> Option<&Vec<SaplingOutputDescription>> {
        self.sapling_outputs.as_ref()
    }

    pub fn sapling_outputs_len(&self) -> usize {
        self.sapling_outputs.as_ref().map_or(0, |v| v.len())
    }

    pub fn add_sapling_output(&mut self, mut output: SaplingOutputDescription) {
        output.set_index(self.sapling_outputs_len());
        self.sapling_outputs
            .get_or_insert_with(Vec::new)
            .push(output);
    }

    pub fn orchard_actions(&self) -> Option<&Vec<OrchardActionDescription>> {
        self.orchard_actions.as_ref()
    }

    pub fn orchard_actions_len(&self) -> usize {
        self.orchard_actions.as_ref().map_or(0, |v| v.len())
    }

    pub fn add_orchard_action(&mut self, mut action: OrchardActionDescription) {
        action.set_index(self.orchard_actions_len());
        self.orchard_actions
            .get_or_insert_with(Vec::new)
            .push(action);
    }

    pub fn sprout_joinsplits(&self) -> Option<&Vec<JoinSplitDescription>> {
        self.sprout_joinsplits.as_ref()
    }

    pub fn sprout_joinsplits_len(&self) -> usize {
        self.sprout_joinsplits.as_ref().map_or(0, |v| v.len())
    }

    pub fn add_sprout_joinsplit(&mut self, mut joinsplit: JoinSplitDescription) {
        joinsplit.set_index(self.sprout_joinsplits_len());
        self.sprout_joinsplits
            .get_or_insert_with(Vec::new)
            .push(joinsplit);
    }

    // Mutable accessors for position updating

    /// Get mutable access to sapling outputs
    pub fn sapling_outputs_mut(&mut self) -> Option<&mut Vec<SaplingOutputDescription>> {
        self.sapling_outputs.as_mut()
    }

    /// Get mutable access to orchard actions
    pub fn orchard_actions_mut(&mut self) -> Option<&mut Vec<OrchardActionDescription>> {
        self.orchard_actions.as_mut()
    }

    /// Get mutable access to sprout joinsplits
    pub fn sprout_joinsplits_mut(&mut self) -> Option<&mut Vec<JoinSplitDescription>> {
        self.sprout_joinsplits.as_mut()
    }

    /// Get mutable access to inputs
    pub fn inputs_mut(&mut self) -> Option<&mut Vec<TxIn>> {
        self.inputs.as_mut()
    }

    /// Get mutable access to outputs
    pub fn outputs_mut(&mut self) -> Option<&mut Vec<TxOut>> {
        self.outputs.as_mut()
    }
}

#[rustfmt::skip]
impl From<Transaction> for Envelope {
    fn from(value: Transaction) -> Self {
        let mut e = Envelope::new(value.txid)
            .add_type("Transaction")
            .add_optional_assertion("raw", value.raw)
            .add_optional_assertion("mined_height", value.mined_height)
            .add_optional_assertion("timestamp", value.timestamp)
            .add_optional_assertion("status", value.status)
            .add_optional_assertion("block_hash", value.block_hash);

        e = value.inputs.into_iter().flatten()
            .fold(e, |e, input| e.add_assertion("input", input));

        e = value.outputs.into_iter().flatten()
            .fold(e, |e, output| e.add_assertion("output", output));

        e = value.sapling_spends.into_iter().flatten()
            .fold(e, |e, spend| e.add_assertion("sapling_spend", spend));

        e = value.sapling_outputs.into_iter().flatten()
            .fold(e, |e, output| e.add_assertion("sapling_output", output));

        e = value.orchard_actions.into_iter().flatten()
            .fold(e, |e, action| e.add_assertion("orchard_action", action));

        e = value.sprout_joinsplits.into_iter().flatten()
            .fold(e, |e, joinsplit| e.add_assertion("sprout_joinsplit", joinsplit));

        value.attachments.add_to_envelope(e)
    }
}

#[rustfmt::skip]
impl TryFrom<Envelope> for Transaction {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self, Self::Error> {
        envelope.check_type_envelope("Transaction")?;
        let txid = envelope.extract_subject().context("txid")?;
        let raw = envelope.try_optional_object_for_predicate("raw").context("raw")?;
        let mined_height = envelope.try_optional_object_for_predicate("mined_height").context("mined_height")?;
        let timestamp = envelope.try_optional_object_for_predicate("timestamp").context("timestamp")?;
        let status = envelope.try_optional_object_for_predicate("status").context("status")?;
        let block_hash = envelope.try_optional_object_for_predicate("block_hash").context("block_hash")?;

        let inputs = envelope_optional_indexed_objects_for_predicate(&envelope, "input").context("inputs")?;
        let outputs = envelope_optional_indexed_objects_for_predicate(&envelope, "output").context("outputs")?;
        let sapling_spends = envelope_optional_indexed_objects_for_predicate(&envelope, "sapling_spend").context("sapling_spends")?;
        let sapling_outputs = envelope_optional_indexed_objects_for_predicate(&envelope, "sapling_output").context("sapling_outputs")?;
        let orchard_actions = envelope_optional_indexed_objects_for_predicate(&envelope, "orchard_action").context("orchard_actions")?;
        let sprout_joinsplits = envelope_optional_indexed_objects_for_predicate(&envelope, "sprout_joinsplit").context("sprout_joinsplits")?;
        let attachments = Attachments::try_from_envelope(&envelope).context("attachments")?;

        Ok(Self {
            txid,
            raw,
            mined_height,
            timestamp,
            status,
            block_hash,
            inputs,
            outputs,
            sapling_spends,
            sapling_outputs,
            orchard_actions,
            sprout_joinsplits,
            attachments,
        })
    }
}

#[cfg(test)]
impl crate::RandomInstance for Transaction {
    fn random() -> Self {
        use crate::SetIndexes;

        Self {
            txid: TxId::random(),
            raw: Data::opt_random(),
            mined_height: BlockHeight::opt_random(),
            timestamp: SecondsSinceEpoch::opt_random(),
            status: TransactionStatus::opt_random(),
            block_hash: u256::opt_random(),
            inputs: Vec::<TxIn>::opt_random().set_indexes(),
            outputs: Vec::<TxOut>::opt_random().set_indexes(),
            sapling_spends: Vec::<SaplingSpendDescription>::opt_random().set_indexes(),
            sapling_outputs: Vec::<SaplingOutputDescription>::opt_random().set_indexes(),
            orchard_actions: Vec::<OrchardActionDescription>::opt_random().set_indexes(),
            sprout_joinsplits: Vec::<JoinSplitDescription>::opt_random().set_indexes(),
            attachments: Attachments::random(),
        }
    }
}

test_envelope_roundtrip!(Transaction);
