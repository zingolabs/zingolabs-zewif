use crate::impl_attachable;

use super::SaplingWitness;
use super::super::{Anchor, Attachments, Position, Blob, Data, u256};

pub type SaplingEncCiphertext = Blob<580>;

/// Data specific to Sapling outputs.
/// *this is currently not sufficient to make all outputs recoverable*
/// Additional things that are needed:
/// * recipient address (unified address that was specified as the recipient by the user)
/// * value of the output
/// * other information in https://github.com/zcash/librustzcash/blob/54fd075449218e27e4cbf5cb7108cd89f458acc0/zcash_client_sqlite/src/wallet/db.rs#L455 may not be recoverable from the chain
#[derive(Debug, Clone, Default)]
pub struct SaplingOutputDescription {
    output_index: u32,
    /// The note commitment.
    commitment: u256,
    /// Ephemeral key for the encrypted note.
    ephemeral_key: u256,
    /// Encrypted ciphertext containing the note details.
    enc_ciphertext: SaplingEncCiphertext,
    /// An optional memo field.
    memo: Option<Data>,
    /// This and the witness are recorded at export as of
    /// an anchor depth 20 blocks back from the chain tip, or the oldest possible witness at a lesser depth.
    note_commitment_tree_position: Position,
    /// Witness
    witness: Option<(Anchor, SaplingWitness)>,
    attachments: Attachments,
}

impl_attachable!(SaplingOutputDescription);

impl SaplingOutputDescription {
    /// Create a new sapling output description.
    pub fn new() -> Self {
        Self {
            output_index: 0,
            commitment: u256::default(),
            ephemeral_key: u256::default(),
            enc_ciphertext: SaplingEncCiphertext::default(),
            memo: None,
            note_commitment_tree_position: Position::default(),
            witness: None,
            attachments: Attachments::new(),
        }
    }

    // Getters
    pub fn output_index(&self) -> u32 {
        self.output_index
    }

    pub fn commitment(&self) -> &u256 {
        &self.commitment
    }

    pub fn ephemeral_key(&self) -> &u256 {
        &self.ephemeral_key
    }

    pub fn enc_ciphertext(&self) -> &SaplingEncCiphertext {
        &self.enc_ciphertext
    }

    pub fn memo(&self) -> Option<&Data> {
        self.memo.as_ref()
    }

    pub fn note_commitment_tree_position(&self) -> &Position {
        &self.note_commitment_tree_position
    }

    pub fn witness(&self) -> Option<&(Anchor, SaplingWitness)> {
        self.witness.as_ref()
    }

    // Setters
    pub fn set_output_index(&mut self, output_index: u32) {
        self.output_index = output_index;
    }

    pub fn set_commitment(&mut self, commitment: u256) {
        self.commitment = commitment;
    }

    pub fn set_ephemeral_key(&mut self, ephemeral_key: u256) {
        self.ephemeral_key = ephemeral_key;
    }

    pub fn set_enc_ciphertext(&mut self, enc_ciphertext: SaplingEncCiphertext) {
        self.enc_ciphertext = enc_ciphertext;
    }

    pub fn set_memo(&mut self, memo: Option<Data>) {
        self.memo = memo;
    }

    pub fn set_note_commitment_tree_position(&mut self, position: Position) {
        self.note_commitment_tree_position = position;
    }

    pub fn set_witness(&mut self, witness: Option<(Anchor, SaplingWitness)>) {
        self.witness = witness;
    }
}
