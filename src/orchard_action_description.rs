use crate::{Indexed, OrchardAnchorWitness, Position, test_envelope_roundtrip};
use anyhow::Context;

use super::{Blob, Data, u256};
use bc_envelope::prelude::*;

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
/// # use zewif::{OrchardActionDescription, Position, u256, Blob, Data, Indexed};
/// // Create a new Orchard action description
/// let mut action = OrchardActionDescription::new();
///
/// // Set action properties
/// let index = 0;
/// let nullifier = u256::default(); // In practice, a real nullifier
/// let commitment = u256::default(); // In practice, a real commitment
///
/// action.set_index(index);
/// action.set_nullifier(nullifier);
/// action.set_commitment(commitment);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct OrchardActionDescription {
    index: usize,
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
    witness: Option<OrchardAnchorWitness>,
    attachments: Attachments,
}

bc_envelope::impl_attachable!(OrchardActionDescription);

impl Indexed for OrchardActionDescription {
    fn index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}

impl OrchardActionDescription {
    pub fn new() -> Self {
        Self {
            index: 0,
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

    pub fn witness(&self) -> Option<&OrchardAnchorWitness> {
        self.witness.as_ref()
    }

    pub fn set_witness(&mut self, witness: Option<OrchardAnchorWitness>) {
        self.witness = witness;
    }
}

impl Default for OrchardActionDescription {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
impl crate::RandomInstance for OrchardActionDescription {
    fn random() -> Self {
        Self {
            index: 0,
            anchor: u256::random(),
            nullifier: u256::random(),
            zkproof: Data::random(),
            commitment: u256::random(),
            ephemeral_key: u256::random(),
            enc_ciphertext: Blob::random(),
            memo: Data::opt_random(),
            note_commitment_tree_position: Position::random(),
            witness: OrchardAnchorWitness::opt_random(),
            attachments: Attachments::random(),
        }
    }
}

impl From<OrchardActionDescription> for Envelope {
    fn from(value: OrchardActionDescription) -> Self {
        let e = Envelope::new(value.index)
            .add_type("OrchardActionDescription")
            .add_assertion("anchor", value.anchor)
            .add_assertion("nullifier", value.nullifier)
            .add_assertion("zkproof", value.zkproof)
            .add_assertion("commitment", value.commitment)
            .add_assertion("ephemeral_key", value.ephemeral_key)
            .add_assertion("enc_ciphertext", value.enc_ciphertext)
            .add_optional_assertion("memo", value.memo)
            .add_assertion(
                "note_commitment_tree_position",
                value.note_commitment_tree_position,
            )
            .add_optional_assertion("witness", value.witness);

        value.attachments.add_to_envelope(e)
    }
}

impl TryFrom<Envelope> for OrchardActionDescription {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self, Self::Error> {
        envelope.check_type_envelope("OrchardActionDescription").context("OrchardActionDescription")?;
        let index = envelope.extract_subject().context("index")?;
        let anchor = envelope.extract_object_for_predicate("anchor").context("anchor")?;
        let nullifier = envelope.extract_object_for_predicate("nullifier").context("nullifier")?;
        let zkproof = envelope.extract_object_for_predicate("zkproof").context("zkproof")?;
        let commitment = envelope.extract_object_for_predicate("commitment").context("commitment")?;
        let ephemeral_key = envelope.extract_object_for_predicate("ephemeral_key").context("ephemeral_key")?;
        let enc_ciphertext = envelope.extract_object_for_predicate("enc_ciphertext").context("enc_ciphertext")?;
        let memo = envelope.try_optional_object_for_predicate("memo").context("memo")?;
        let note_commitment_tree_position =
            envelope.extract_object_for_predicate("note_commitment_tree_position").context("note_commitment_tree_position")?;
        let witness = envelope.try_optional_object_for_predicate("witness").context("witness")?;

        let attachments = Attachments::try_from_envelope(&envelope)?;

        Ok(OrchardActionDescription {
            index,
            anchor,
            nullifier,
            zkproof,
            commitment,
            ephemeral_key,
            enc_ciphertext,
            memo,
            note_commitment_tree_position,
            witness,
            attachments,
        })
    }
}

test_envelope_roundtrip!(OrchardActionDescription, 10, true);
