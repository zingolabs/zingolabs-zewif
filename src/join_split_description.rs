use crate::impl_attachable;
use super::{Anchor, Attachments, SproutProof, u256};

/// A structure for Sprout shielded transactions that can convert between transparent and shielded value.
///
/// `JoinSplitDescription` represents a cryptographic construction in the legacy Sprout protocol
/// that allows value to flow between the transparent and shielded parts of the Zcash blockchain.
/// It implements a mechanism similar to a "shielding" or "deshielding" operation, where each
/// JoinSplit can take up to two input notes and create up to two output notes, with value
/// potentially flowing in or out of the shielded pool.
///
/// # Zcash Concept Relation
/// In Zcash's original Sprout protocol (active from 2016-2018):
///
/// - **JoinSplits**: Allow conversion between transparent and shielded value
/// - **Dual Input/Output**: Each JoinSplit can consume two old notes and create two new notes
/// - **Note Commitments**: Cryptographic commitments to the new notes created
/// - **Nullifiers**: Unique identifiers that prevent double-spending of input notes
/// - **Anchor**: Merkle tree root that fixes the blockchain state for the proof
/// - **zkProof**: Zero-knowledge proof that validates the operation without revealing details
///
/// JoinSplits were the primary mechanism in the Sprout protocol for maintaining privacy,
/// allowing users to shield coins (move from transparent to shielded) or deshield coins
/// (move from shielded to transparent).
///
/// # Data Preservation
/// During wallet migration, the following components must be preserved:
///
/// - **Anchor**: The Merkle tree root at the time of the transaction
/// - **Nullifiers**: Identifiers for spent notes, preventing double-spending
/// - **Commitments**: Cryptographic commitments to the created notes
/// - **Proof Type**: Whether PHGR (original) or Groth16 (later) was used
/// - **Proof Data**: The cryptographic proof validating the transaction
///
/// # Examples
/// ```
/// use zewif::{JoinSplitDescription, Anchor, SproutProof, PHGRProof, Blob, u256};
///
/// // Create components for a JoinSplit description
/// let anchor = Anchor::default(); // In practice, this would be a real Merkle root
/// let nullifiers = [u256::default(), u256::default()]; // Identifiers for spent notes
/// let commitments = [u256::default(), u256::default()]; // Commitments to new notes
///
/// // Create a PHGR proof for the JoinSplit
/// let g1_point = Blob::new([0u8; 33]);
/// let phgr = PHGRProof::with_fields(
///     g1_point.clone(), g1_point.clone(), g1_point.clone(), g1_point.clone(),
///     g1_point.clone(), g1_point.clone(), g1_point.clone(), g1_point.clone()
/// );
/// let zkproof = SproutProof::PHGRProof(phgr);
///
/// // Create the JoinSplit description
/// let joinsplit = JoinSplitDescription::new(
///     anchor,
///     nullifiers,
///     commitments,
///     zkproof
/// );
/// ```
#[derive(Debug, Clone)]
pub struct JoinSplitDescription {
    /// Merkle tree root (anchor) used in the zero-knowledge proof
    anchor: Anchor,
    
    /// Nullifiers for the two input notes being spent (if any)
    nullifiers: [u256; 2],
    
    /// Commitments to the two output notes being created (if any)
    commitments: [u256; 2],
    
    /// Zero-knowledge proof validating the JoinSplit
    zkproof: SproutProof,
    
    /// Additional metadata attachments for this JoinSplit
    attachments: Attachments,
}

impl_attachable!(JoinSplitDescription);

impl JoinSplitDescription {
    /// Creates a new JoinSplit description with the specified components.
    ///
    /// # Arguments
    /// * `anchor` - The Merkle tree root used in the zero-knowledge proof
    /// * `nullifiers` - Array of two nullifiers for the input notes being spent
    /// * `commitments` - Array of two commitments for the output notes being created
    /// * `zkproof` - The zero-knowledge proof validating this JoinSplit
    ///
    /// # Returns
    /// A new `JoinSplitDescription` with the specified components and default attachments
    ///
    /// # Examples
    /// ```
    /// use zewif::{JoinSplitDescription, Anchor, SproutProof, GrothProof, u256};
    ///
    /// // Create components for a JoinSplit description
    /// let anchor = Anchor::default();
    /// let nullifiers = [u256::default(), u256::default()];
    /// let commitments = [u256::default(), u256::default()];
    /// 
    /// // Create a Groth proof for the JoinSplit
    /// let groth_bytes = [0u8; 192];
    /// let zkproof = SproutProof::GrothProof(GrothProof::new(groth_bytes));
    ///
    /// // Create the JoinSplit description
    /// let joinsplit = JoinSplitDescription::new(
    ///     anchor,
    ///     nullifiers,
    ///     commitments,
    ///     zkproof
    /// );
    /// ```
    pub fn new(
        anchor: Anchor,
        nullifiers: [u256; 2],
        commitments: [u256; 2],
        zkproof: SproutProof,
    ) -> Self {
        Self {
            anchor,
            nullifiers,
            commitments,
            zkproof,
            attachments: Attachments::default(),
        }
    }

    /// Returns the Merkle tree root (anchor) used in this JoinSplit.
    ///
    /// The anchor is a commitment to the state of the note commitment tree
    /// at a specific point in time, used to validate that input notes existed
    /// in the blockchain state.
    ///
    /// # Returns
    /// The Merkle tree root (anchor) value
    pub fn anchor(&self) -> Anchor {
        self.anchor
    }

    /// Returns the nullifiers for the input notes being spent.
    ///
    /// Nullifiers are unique identifiers that prevent double-spending of notes.
    /// When a note is spent, its nullifier is published on the blockchain.
    ///
    /// # Returns
    /// An array of two nullifiers for the input notes
    pub fn nullifiers(&self) -> [u256; 2] {
        self.nullifiers
    }

    /// Returns the commitments to the output notes being created.
    ///
    /// Note commitments are cryptographic commitments that appear on the blockchain
    /// representing the new notes created by this JoinSplit, without revealing their details.
    ///
    /// # Returns
    /// An array of two commitments for the output notes
    pub fn commitments(&self) -> [u256; 2] {
        self.commitments
    }

    /// Returns a reference to the zero-knowledge proof.
    ///
    /// The zkproof validates that the JoinSplit operation is legitimate without
    /// revealing the contents of the notes or the values being transferred.
    ///
    /// # Returns
    /// A reference to the SproutProof (either PHGR or Groth16)
    pub fn zkproof(&self) -> &SproutProof {
        &self.zkproof
    }

    /// Returns a reference to the additional metadata attachments.
    ///
    /// Attachments can contain additional data associated with this JoinSplit
    /// that isn't part of the core protocol.
    ///
    /// # Returns
    /// A reference to the attachments collection
    pub fn attachments(&self) -> &Attachments {
        &self.attachments
    }
}
