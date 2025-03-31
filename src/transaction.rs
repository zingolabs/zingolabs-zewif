use super::{BlockHeight, Data, SecondsSinceEpoch, TxId, u256};
use crate::impl_attachable;

use super::{
    Attachments, JoinSplitDescription, OrchardActionDescription, TxIn, TxOut,
    sapling::{SaplingOutputDescription, SaplingSpendDescription},
};

/// The status of a transaction in the blockchain
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionStatus {
    /// Transaction is in the mempool, not yet confirmed
    Pending,
    /// Transaction is confirmed in a block
    Confirmed,
    /// Transaction failed to be included in a block
    Failed,
    /// Transaction was abandoned
    Abandoned,
}

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
/// use zewif::{Transaction, TxId, TransactionStatus, BlockHeight};
///
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
#[derive(Debug, Clone)]
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
    // necessarily have code to parse a transaction. The only information
    // that is not redundant with the raw transaction encoding is the
    // *decrypted* note plaintexts (and it might be sufficient to just
    // indicate which output indices are expected to be decryptable with
    // which keys). I don't see the point of duplicating the raw data in a
    // different format (that still needs to be parsed!)
    // -- Daira-Emma
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

impl_attachable!(Transaction);

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

    pub fn add_input(&mut self, input: TxIn) {
        self.inputs.get_or_insert_with(Vec::new).push(input);
    }

    pub fn outputs(&self) -> Option<&Vec<TxOut>> {
        self.outputs.as_ref()
    }

    pub fn add_output(&mut self, output: TxOut) {
        self.outputs.get_or_insert_with(Vec::new).push(output);
    }

    pub fn sapling_spends(&self) -> Option<&Vec<SaplingSpendDescription>> {
        self.sapling_spends.as_ref()
    }

    pub fn add_sapling_spend(&mut self, spend: SaplingSpendDescription) {
        self.sapling_spends.get_or_insert_with(Vec::new).push(spend);
    }

    pub fn sapling_outputs(&self) -> Option<&Vec<SaplingOutputDescription>> {
        self.sapling_outputs.as_ref()
    }

    pub fn add_sapling_output(&mut self, output: SaplingOutputDescription) {
        self.sapling_outputs
            .get_or_insert_with(Vec::new)
            .push(output);
    }

    pub fn orchard_actions(&self) -> Option<&Vec<OrchardActionDescription>> {
        self.orchard_actions.as_ref()
    }

    pub fn add_orchard_action(&mut self, action: OrchardActionDescription) {
        self.orchard_actions
            .get_or_insert_with(Vec::new)
            .push(action);
    }

    pub fn sprout_joinsplits(&self) -> Option<&Vec<JoinSplitDescription>> {
        self.sprout_joinsplits.as_ref()
    }

    pub fn add_sprout_joinsplit(&mut self, joinsplit: JoinSplitDescription) {
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
