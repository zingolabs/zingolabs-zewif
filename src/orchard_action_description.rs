use crate::impl_attachable;
use super::{Blob, Data, u256};

use super::{Anchor, Attachments, IncrementalWitness, Position};

/// The depth of the Orchard Merkle tree, set to 32 levels.
///
/// This constant defines the maximum depth of the Orchard note commitment tree,
/// which allows for 2^32 (over 4 billion) note commitments to be included.
const ORCHARD_INCREMENTAL_MERKLE_TREE_DEPTH: usize = 32;

/// A type alias for the Sinsemilla hash used in Orchard Merkle trees.
///
/// Sinsemilla hashes are cryptographic hash functions used for note commitments 
/// and in the Merkle tree structure for the Orchard protocol. They provide efficient
/// hashing with homomorphic properties used in zero-knowledge proofs.
pub type SinsemillaHash = u256;

/// A cryptographic witness proving that an Orchard note commitment exists in the note commitment tree.
///
/// This type specializes the generic `IncrementalWitness` for the Orchard protocol parameters.
pub type OrchardWitness = IncrementalWitness<ORCHARD_INCREMENTAL_MERKLE_TREE_DEPTH, SinsemillaHash>;

/// A description of an action in the Orchard shielded pool.
///
/// `OrchardActionDescription` represents a combined spend and output in the Orchard
/// shielded protocol. Unlike earlier protocols that separated spends and outputs,
/// Orchard uses a unified "action" approach where a single privacy-preserving operation
/// can consume existing notes and create new ones.
///
/// # Zcash Concept Relation
///
/// In Zcash's Orchard protocol:
///
/// - **Actions**: The fundamental building blocks of Orchard transactions
/// - **Unified Design**: Each action combines aspects of both spending and output creation
/// - **Note Commitments**: Cryptographic commitments to shielded values and metadata
/// - **Nullifiers**: Unique identifiers published when spending notes to prevent double-spending
/// - **Witnesses**: Cryptographic proofs demonstrating note existence in the commitment tree
///
/// Orchard was introduced in the NU5 network upgrade and represents the most
/// advanced shielded protocol in Zcash, with improvements over both Sprout and Sapling.
///
/// # Data Preservation
///
/// During wallet migration, the following Orchard-specific data must be preserved:
///
/// - **Nullifiers**: Essential to prevent double-spending of notes
/// - **Commitments**: Required to identify and validate notes
/// - **Encrypted Ciphertexts**: Contain encrypted note details
/// - **Witness Data**: Required for spending notes (proving they exist in the tree)
/// - **Proof Data**: Zero-knowledge proofs verifying transaction validity
///
/// Without this data, Orchard notes couldn't be properly identified or spent.
///
/// # Examples
/// ```
/// use zewif::{OrchardActionDescription, Position, u256, Blob, Data};
///
/// // Create a new Orchard action description
/// let mut action = OrchardActionDescription::new();
///
/// // Set action properties
/// let action_index = 0;
/// let nullifier = u256::default(); // In practice, a real nullifier
/// let commitment = u256::default(); // In practice, a real commitment
///
/// action.set_action_index(action_index);
/// action.set_nullifier(nullifier);
/// action.set_commitment(commitment);
/// ```
#[derive(Debug, Clone)]
pub struct OrchardActionDescription {
    action_index: u32,
    /// The anchor of the current commitment tree.
    anchor: u256,
    /// A nullifier to ensure the note is spent only once.
    nullifier: u256,
    /// A zero-knowledge proof that the spend is valid.
    zkproof: Data,
    /// Additional fields (e.g., spending key components) may be required.
    /// The note commitment.
    commitment: u256,
    /// Ephemeral key for the encrypted note.
    ephemeral_key: u256,
    /// Encrypted ciphertext containing the note details.
    enc_ciphertext: Blob<580>,
    /// An optional memo field.
    memo: Option<Data>,
    /// This and the witness are recorded at export as of
    /// an anchor depth 20 blocks back from the chain tip, or the oldest possible witness at a lesser depth.
    note_commitment_tree_position: Position,
    /// Witness
    witness: Option<(Anchor, OrchardWitness)>,
    attachments: Attachments,
}

impl_attachable!(OrchardActionDescription);

impl OrchardActionDescription {
    pub fn new() -> Self {
        Self {
            action_index: 0,
            anchor: u256::default(),
            nullifier: u256::default(),
            zkproof: Data::default(),
            commitment: u256::default(),
            ephemeral_key: u256::default(),
            enc_ciphertext: Blob::default(),
            memo: None,
            note_commitment_tree_position: Position::default(),
            witness: None,
            attachments: Attachments::new(),
        }
    }

    pub fn action_index(&self) -> u32 {
        self.action_index
    }

    pub fn set_action_index(&mut self, action_index: u32) {
        self.action_index = action_index;
    }

    pub fn anchor(&self) -> &u256 {
        &self.anchor
    }

    pub fn set_anchor(&mut self, anchor: u256) {
        self.anchor = anchor;
    }

    pub fn nullifier(&self) -> &u256 {
        &self.nullifier
    }

    pub fn set_nullifier(&mut self, nullifier: u256) {
        self.nullifier = nullifier;
    }

    pub fn zkproof(&self) -> &Data {
        &self.zkproof
    }

    pub fn set_zkproof(&mut self, zkproof: Data) {
        self.zkproof = zkproof;
    }

    pub fn commitment(&self) -> &u256 {
        &self.commitment
    }

    pub fn set_commitment(&mut self, commitment: u256) {
        self.commitment = commitment;
    }

    pub fn ephemeral_key(&self) -> &u256 {
        &self.ephemeral_key
    }

    pub fn set_ephemeral_key(&mut self, ephemeral_key: u256) {
        self.ephemeral_key = ephemeral_key;
    }

    pub fn enc_ciphertext(&self) -> &Blob<580> {
        &self.enc_ciphertext
    }

    pub fn set_enc_ciphertext(&mut self, enc_ciphertext: Blob<580>) {
        self.enc_ciphertext = enc_ciphertext;
    }

    pub fn memo(&self) -> Option<&Data> {
        self.memo.as_ref()
    }

    pub fn set_memo(&mut self, memo: Option<Data>) {
        self.memo = memo;
    }

    pub fn note_commitment_tree_position(&self) -> &Position {
        &self.note_commitment_tree_position
    }

    pub fn set_note_commitment_tree_position(&mut self, note_commitment_tree_position: Position) {
        self.note_commitment_tree_position = note_commitment_tree_position;
    }

    pub fn witness(&self) -> Option<&(Anchor, OrchardWitness)> {
        self.witness.as_ref()
    }

    pub fn set_witness(&mut self, witness: Option<(Anchor, OrchardWitness)>) {
        self.witness = witness;
    }
}

impl Default for OrchardActionDescription {
    fn default() -> Self {
        Self::new()
    }
}
