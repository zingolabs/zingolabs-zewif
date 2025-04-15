use anyhow::Context;
use bc_envelope::prelude::*;

use crate::{test_envelope_roundtrip, Indexed};

use super::super::{Blob, Data, Position, u256};
use super::SaplingAnchorWitness;

/// A fixed-size blob type for Sapling encrypted ciphertexts (580 bytes).
pub type SaplingEncCiphertext = Blob<580>;

/// A description of a received output in a Sapling shielded transaction.
///
/// `SaplingOutputDescription` represents a received note in the Sapling shielded pool,
/// containing both the blockchain-published components (commitments, ciphertexts) and
/// wallet-specific data needed to later spend the note (witness data, position information).
///
/// # Zcash Concept Relation
/// In Zcash's Sapling protocol:
///
/// - **Notes**: Encrypted value containers, similar to UTXOs in transparent transactions
/// - **Note Commitments**: Public values on the blockchain that commit to the note's contents
///   without revealing them
/// - **Encrypted Ciphertexts**: Contain the encrypted details of the note (value, memo, etc.)
/// - **Ephemeral Keys**: One-time keys used to encrypt notes for their intended recipients
/// - **Witnesses**: Cryptographic proofs that a note commitment exists in the note commitment
///   tree, needed when spending the note
///
/// # Data Preservation
/// During wallet migration, the following components must be preserved:
///
/// - **Note Commitment**: The note's representation on the blockchain
/// - **Encrypted Data**: The ciphertext containing the note's details
/// - **Position Information**: Where the note exists in the commitment tree
/// - **Witness Data**: Cryptographic proof of the note's inclusion in the tree
/// - **Memo Data**: Optional encrypted message data attached to the note
///
/// # Examples
/// ```
/// # use zewif::{sapling::{SaplingOutputDescription, SaplingEncCiphertext}, Position, u256, Data, Indexed};
/// // Create a new output description
/// let mut output = SaplingOutputDescription::new();
///
/// // Set the output index in the transaction
/// output.set_index(0);
///
/// // Set the note commitment that appears on the blockchain
/// let commitment = u256::default(); // In practice, this would be the actual commitment
/// output.set_commitment(commitment);
///
/// // Set the ephemeral key used for encryption
/// let ephemeral_key = u256::default(); // In practice, this would be the actual key
/// output.set_ephemeral_key(ephemeral_key);
///
/// // Set the position in the note commitment tree
/// let position = Position::default(); // In practice, this would be the actual position
/// output.set_note_commitment_tree_position(position);
///
/// // The ciphertext containing encrypted note details
/// let ciphertext = SaplingEncCiphertext::default();
/// output.set_enc_ciphertext(ciphertext);
///
/// // Optional memo field attached to the note
/// let memo_data = Data::from_vec(vec![0u8; 32]); // In practice, this would be actual memo content
/// output.set_memo(Some(memo_data));
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SaplingOutputDescription {
    /// Index of this output in the transaction's list of Sapling outputs
    index: usize,
    /// The note commitment visible on the blockchain
    commitment: u256,
    /// Ephemeral key for the encrypted note
    ephemeral_key: u256,
    /// Encrypted ciphertext containing the note details
    enc_ciphertext: SaplingEncCiphertext,
    /// An optional memo field attached to the note
    memo: Option<Data>,
    /// Position of this note commitment in the note commitment tree
    note_commitment_tree_position: Position,
    /// Witness data proving inclusion in the note commitment tree
    witness: Option<SaplingAnchorWitness>,
    /// Additional metadata attachments for this output
    attachments: Attachments,
}

bc_envelope::impl_attachable!(SaplingOutputDescription);

impl Indexed for SaplingOutputDescription {
    fn index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}

impl SaplingOutputDescription {
    /// Creates a new empty Sapling output description.
    ///
    /// This constructor initializes a `SaplingOutputDescription` with default values
    /// for all fields. In practical use, these values would be set using the setter
    /// methods before the object is used.
    ///
    /// # Returns
    /// A new `SaplingOutputDescription` instance with default values.
    ///
    /// # Examples
    /// ```
    /// # use zewif::sapling::SaplingOutputDescription;
    /// let output = SaplingOutputDescription::new();
    /// ```
    pub fn new() -> Self {
        Self {
            index: 0,
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

    /// Returns a reference to the note commitment.
    ///
    /// The note commitment is a cryptographic commitment to the note's contents that
    /// appears on the blockchain without revealing the note details. It's used for
    /// proving ownership and preventing double-spending.
    pub fn commitment(&self) -> &u256 {
        &self.commitment
    }

    /// Returns a reference to the ephemeral key.
    ///
    /// The ephemeral key is a one-time key used to encrypt the note for its intended
    /// recipient. It's published on the blockchain alongside the note commitment.
    pub fn ephemeral_key(&self) -> &u256 {
        &self.ephemeral_key
    }

    /// Returns a reference to the encrypted ciphertext.
    ///
    /// The encrypted ciphertext contains the encrypted note details, including the
    /// value, recipient, and other metadata. Only the note's owner can decrypt it.
    pub fn enc_ciphertext(&self) -> &SaplingEncCiphertext {
        &self.enc_ciphertext
    }

    /// Returns a reference to the optional memo field, if present.
    ///
    /// The memo field allows senders to attach encrypted messages to shielded
    /// transactions. This field is optional - not all notes have memos.
    pub fn memo(&self) -> Option<&Data> {
        self.memo.as_ref()
    }

    /// Returns a reference to the note commitment tree position.
    ///
    /// The position records where this note commitment exists in the global note
    /// commitment tree. This information is necessary for creating witnesses when
    /// the note is later spent.
    pub fn note_commitment_tree_position(&self) -> &Position {
        &self.note_commitment_tree_position
    }

    /// Returns a reference to the witness data, if present.
    ///
    /// The witness is a cryptographic proof that the note commitment exists in the
    /// note commitment tree at a particular position. It's required when spending
    /// the note in a future transaction.
    pub fn witness(&self) -> Option<&SaplingAnchorWitness> {
        self.witness.as_ref()
    }

    // Setters

    /// Sets the note commitment.
    pub fn set_commitment(&mut self, commitment: u256) {
        self.commitment = commitment;
    }

    /// Sets the ephemeral key.
    pub fn set_ephemeral_key(&mut self, ephemeral_key: u256) {
        self.ephemeral_key = ephemeral_key;
    }

    /// Sets the encrypted ciphertext.
    pub fn set_enc_ciphertext(&mut self, enc_ciphertext: SaplingEncCiphertext) {
        self.enc_ciphertext = enc_ciphertext;
    }

    /// Sets the optional memo field.
    pub fn set_memo(&mut self, memo: Option<Data>) {
        self.memo = memo;
    }

    /// Sets the note commitment tree position.
    pub fn set_note_commitment_tree_position(&mut self, position: Position) {
        self.note_commitment_tree_position = position;
    }

    /// Sets the witness data.
    pub fn set_witness(&mut self, witness: Option<SaplingAnchorWitness>) {
        self.witness = witness;
    }
}

impl From<SaplingOutputDescription> for Envelope {
    fn from(value: SaplingOutputDescription) -> Self {
        let e = Envelope::new(value.index)
            .add_type("SaplingOutputDescription")
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

impl TryFrom<Envelope> for SaplingOutputDescription {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self, Self::Error> {
        envelope.check_type_envelope("SaplingOutputDescription").context("SaplingOutputDescription")?;
        let index = envelope.extract_subject().context("index")?;
        let commitment = envelope.try_object_for_predicate("commitment").context("commitment")?;
        let ephemeral_key = envelope.try_object_for_predicate("ephemeral_key").context("ephemeral_key")?;
        let enc_ciphertext = envelope.try_object_for_predicate("enc_ciphertext").context("enc_ciphertext")?;
        let memo = envelope.extract_optional_object_for_predicate("memo").context("memo")?;
        let note_commitment_tree_position =
            envelope.extract_object_for_predicate("note_commitment_tree_position").context("note_commitment_tree_position")?;
        let witness = envelope.try_optional_object_for_predicate("witness").context("witness")?;
        let attachments = Attachments::try_from_envelope(&envelope)?;
        Ok(SaplingOutputDescription {
            index,
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

#[cfg(test)]
impl crate::RandomInstance for SaplingOutputDescription {
    fn random() -> Self {
        Self {
            index: 0,
            commitment: u256::random(),
            ephemeral_key: u256::random(),
            enc_ciphertext: SaplingEncCiphertext::random(),
            memo: Data::opt_random_with_size(64),
            note_commitment_tree_position: Position::random(),
            witness: SaplingAnchorWitness::opt_random(),
            attachments: Attachments::random(),
        }
    }
}

test_envelope_roundtrip!(SaplingOutputDescription);
