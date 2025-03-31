use crate::blob;

/// Size of a complete Groth16 proof in bytes (192)
pub const GROTH_PROOF_SIZE: usize = 48 + 96 + 48;

// The GrothProof type represents a Groth16 zk-SNARK proof used in Zcash's Sapling and Orchard
// shielded transactions. It's a 192-byte proof consisting of three elliptic curve points:
// Point A (48 bytes), Point B (96 bytes), and Point C (48 bytes).
//
// In Zcash, these proofs cryptographically verify transaction validity without revealing
// private information like addresses or amounts.

blob!(GrothProof, GROTH_PROOF_SIZE, "A Groth16 zk-SNARK proof used in Zcash's Sapling and Orchard shielded transactions.");
