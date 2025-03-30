use crate::impl_attachable;

use super::SaplingWitness;
use super::super::{Anchor, Attachments, Position, Blob, Data, u256};

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
/// use zewif::{sapling::{SaplingOutputDescription, SaplingEncCiphertext}, Position, u256, Data};
///
/// // Create a new output description
/// let mut output = SaplingOutputDescription::new();
///
/// // Set the output index in the transaction
/// output.set_output_index(0);
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
#[derive(Debug, Clone, Default)]
pub struct SaplingOutputDescription {
    /// Index of this output in the transaction's list of Sapling outputs
    output_index: u32,
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
    witness: Option<(Anchor, SaplingWitness)>,
    /// Additional metadata attachments for this output
    attachments: Attachments,
}

impl_attachable!(SaplingOutputDescription);

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
    /// use zewif::sapling::SaplingOutputDescription;
    /// 
    /// let output = SaplingOutputDescription::new();
    /// ```
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
    
    /// Returns the index of this output in the transaction's list of Sapling outputs.
    pub fn output_index(&self) -> u32 {
        self.output_index
    }

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
    pub fn witness(&self) -> Option<&(Anchor, SaplingWitness)> {
        self.witness.as_ref()
    }

    // Setters
    
    /// Sets the index of this output in the transaction's list of Sapling outputs.
    pub fn set_output_index(&mut self, output_index: u32) {
        self.output_index = output_index;
    }

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
    pub fn set_witness(&mut self, witness: Option<(Anchor, SaplingWitness)>) {
        self.witness = witness;
    }
}
