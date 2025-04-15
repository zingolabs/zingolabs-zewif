use super::u256;

/// A commitment tree root used for proving membership of shielded notes.
///
/// `Anchor` is a cryptographic commitment that represents the root of a note commitment
/// tree at a specific point in time. It is used in shielded transaction validation to
/// prove that a note being spent was part of the blockchain state without revealing which
/// specific note is being spent.
///
/// # Zcash Concept Relation
/// In Zcash's shielded protocols (Sapling and Orchard), each spent note must reference
/// an anchor to create a zero-knowledge proof showing:
///
/// 1. The note exists in the blockchain as of a particular state (the anchor)
/// 2. The spender has the authority to spend this note
/// 3. The note has not been previously spent
///
/// The anchors are critical for maintaining privacy while preventing double-spending.
///
/// # Data Preservation
/// `Anchor` preserves the exact 256-bit root hash values from wallet data, which are
/// needed when reconstructing or validating shielded transactions during wallet migration.
///
/// # Technical Implementation
/// Technically, `Anchor` is an alias for the [`u256`](crate::u256) type, which represents
/// a 256-bit unsigned integer used for cryptographic values.
///
/// # Examples
/// ```
/// # use zewif::{Anchor, u256};
/// #
/// // Create an anchor from a u256 value
/// let anchor_value = u256::default(); // Usually this would be a real tree root
/// let anchor: Anchor = anchor_value;
/// ```
pub type Anchor = u256;
