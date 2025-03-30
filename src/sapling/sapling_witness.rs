use super::super::{IncrementalWitness, u256};

/// The depth of the Sapling Merkle tree, set to 32 levels.
///
/// This constant defines the maximum depth of the Sapling note commitment tree,
/// which allows for 2^32 (over 4 billion) note commitments to be included.
const SAPLING_INCREMENTAL_MERKLE_TREE_DEPTH: usize = 32;

/// A type alias for the Pedersen hash used in Sapling Merkle trees.
///
/// Pedersen hashes are used for note commitments and in the Merkle tree structure
/// for the Sapling protocol. They provide cryptographic binding while maintaining
/// homomorphic properties useful for zero-knowledge proofs.
pub type PedersenHash = u256;

/// A cryptographic witness proving that a Sapling note commitment exists in the note commitment tree.
///
/// `SaplingWitness` is a specialized form of incremental Merkle tree witness for the
/// Sapling protocol. It proves that a specific note commitment is included in the
/// global Sapling note commitment tree, which is necessary when spending a note.
///
/// # Zcash Concept Relation
/// In Zcash's Sapling protocol:
///
/// - **Note Commitment Tree**: A Merkle tree containing all Sapling note commitments
/// - **Merkle Path**: The path from a leaf (note commitment) to the root of the tree
/// - **Witness**: The authentication path proving a leaf exists in the tree
/// - **Anchors**: Root hashes of the note commitment tree at specific blockchain heights
///
/// When spending a Sapling note, a zero-knowledge proof must demonstrate that the
/// note's commitment exists in the tree at a specific anchor (root hash), without
/// revealing which specific commitment is being spent. The witness provides the
/// necessary path information to create this proof.
///
/// # Data Preservation
/// During wallet migration, complete witness data must be preserved for all unspent
/// notes. This includes:
///
/// - The authentication path (sequence of hashes forming the Merkle path)
/// - The position of the note commitment in the tree
/// - The tree depth used (32 for Sapling)
///
/// Without this witness data, unspent notes cannot be spent as it would be impossible
/// to prove their inclusion in the note commitment tree.
///
/// # Implementation Details
/// This type is an alias for `IncrementalWitness<32, PedersenHash>`, representing a
/// witness for a Merkle tree with 32 levels using Pedersen hashes as the hash function.
/// The witness supports incremental updates as new notes are added to the tree.
pub type SaplingWitness = IncrementalWitness<SAPLING_INCREMENTAL_MERKLE_TREE_DEPTH, PedersenHash>;
