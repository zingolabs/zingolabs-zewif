use anyhow::Result;

use super::{parse, parser::prelude::*};
use super::{GrothProof, PHGRProof};

/// A zero-knowledge proof for the legacy Sprout shielded protocol in Zcash.
///
/// `SproutProof` represents one of two zero-knowledge proof systems used in the Sprout
/// protocol, the first shielded transaction format in Zcash. The proof can be either a
/// PHGR proof (Pinocchio Hash Generator and Reducer) from the original launch, or a Groth16
/// proof introduced in the Sapling upgrade that was retrofitted to Sprout for efficiency.
///
/// # Zcash Concept Relation
/// In Zcash's evolution, the Sprout protocol underwent changes to its proving system:
///
/// - **Original Sprout (2016)**: Used PHGR proofs based on the Pinocchio zk-SNARK system.
///   These were the first zk-SNARKs used in Zcash, requiring the original "powers of tau"
///   trusted setup.
///
/// - **Sprout with Groth16 (2018)**: During the Sapling upgrade, Zcash retrofitted the 
///   more efficient Groth16 proving system to the Sprout protocol, allowing nodes to
///   validate both proof types.
///
/// The blockchain contains both types of proofs for Sprout transactions, depending on when
/// they were created. Sprout was eventually succeeded by the Sapling protocol, but existing
/// Sprout transactions and their proofs remain part of the blockchain.
///
/// # Data Preservation
/// During wallet migration, the following aspects of Sprout proofs are preserved:
///
/// - **Proof Type**: The distinction between PHGR and Groth16 proofs
/// - **Proof Content**: The complete binary representation of the cryptographic proof
///
/// These proofs are used in JoinSplit descriptions, which record the mixing of transparent
/// and shielded values in legacy Sprout transactions.
///
/// # Examples
/// ```
/// use zewif::{SproutProof, PHGRProof, GrothProof, Blob};
///
/// // Example compressed G1 point (33 bytes each)
/// let g1_point = Blob::new([0u8; 33]);
///
/// // Create a PHGR proof with all its components
/// let phgr = PHGRProof::with_fields(
///     g1_point.clone(), g1_point.clone(), g1_point.clone(), g1_point.clone(),
///     g1_point.clone(), g1_point.clone(), g1_point.clone(), g1_point.clone()
/// );
/// let sprout_phgr = SproutProof::PHGRProof(phgr);
///
/// // Or create a Groth proof for Sprout
/// let groth_bytes = [0u8; 192];
/// let groth = GrothProof::new(groth_bytes);
/// let sprout_groth = SproutProof::GrothProof(groth);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum SproutProof {
    /// Original Pinocchio-based PHGR proof used at Zcash launch
    PHGRProof(PHGRProof),
    /// More efficient Groth16 proof retrofitted to Sprout during the Sapling upgrade
    GrothProof(GrothProof),
}

impl ParseWithParam<bool> for SproutProof {
    fn parse(p: &mut Parser, use_groth: bool) -> Result<Self> {
        if use_groth {
            Ok(Self::GrothProof(parse!(p, "groth_proof")?))
        } else {
            Ok(Self::PHGRProof(parse!(p, "phgr_proof")?))
        }
    }
}
