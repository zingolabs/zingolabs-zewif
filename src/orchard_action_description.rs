use crate::impl_attachable;
use super::{Blob, Data, u256};

use super::{Anchor, Attachments, IncrementalWitness, Position};

const ORCHARD_INCREMENTAL_MERKLE_TREE_DEPTH: usize = 32;
pub type SinsemillaHash = u256;
pub type OrchardWitness = IncrementalWitness<ORCHARD_INCREMENTAL_MERKLE_TREE_DEPTH, SinsemillaHash>;

/// Data specific to Orchard actions.
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
