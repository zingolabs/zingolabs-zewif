use crate::{test_envelope_roundtrip, u256};

use crate::IncrementalWitness;

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

#[cfg(test)]
impl crate::RandomInstance for IncrementalWitness<32, u256> {
    fn random() -> Self {
        let tree = crate::IncrementalMerkleTree::random();
        let filled: Vec<SinsemillaHash> = (0..10).map(|_| SinsemillaHash::random()).collect();
        let cursor = crate::IncrementalMerkleTree::opt_random();
        Self::with_fields(tree, filled, cursor)
    }
}

test_envelope_roundtrip!(OrchardWitness);
