use bc_envelope::prelude::*;
use anyhow::{Result, Context};

use crate::test_envelope_roundtrip;

use super::{parse, parser::prelude::*};
use super::Blob;

/// Compressed representation of a G1 elliptic curve point (33 bytes)
/// A byte for point format (0x02 or 0x03) followed by 32 bytes for the Fq field element
pub type CompressedG1 = Blob<33>;

/// A zero-knowledge proof using the PHGR (Pinocchio Hash Generator and Reducer) proving system.
///
/// `PHGRProof` represents a zero-knowledge proof from the original Sprout protocol in Zcash,
/// based on the Pinocchio zk-SNARK system. This proof consists of eight elliptic curve points
/// in compressed form, collectively providing cryptographic verification of shielded transaction
/// validity without revealing private details.
///
/// # Zcash Concept Relation
/// In the original Zcash implementation (prior to Sapling):
///
/// - **Sprout Protocol**: Used PHGR proofs as the first zk-SNARK implementation in Zcash
/// - **Trusted Setup**: Required the original "powers of tau" ceremony to create the common
///   reference string needed for these proofs
/// - **Structure**: Each PHGR proof contains eight G1 points representing various cryptographic
///   elements of the proof
///
/// These proofs verify that:
/// - A JoinSplit operation properly converts between transparent and shielded value
/// - The spender has authority over the notes being spent
/// - The cryptographic constraints of the system are satisfied
///
/// While Zcash later transitioned to Groth16 proofs for efficiency, PHGR proofs remain
/// in the blockchain from early Sprout transactions.
///
/// # Data Preservation
/// During wallet migration, the following aspects of PHGR proofs are preserved:
///
/// - **All Eight Points**: The complete set of elliptic curve points in their compressed form
/// - **Point Order**: The specific ordering of points that determines the proof's validity
///
/// # Examples
/// ```
/// # use zewif::{PHGRProof, Blob};
/// // Create a compressed G1 point (33 bytes)
/// let g1_point = Blob::new([0u8; 33]);
///
/// // Construct a PHGR proof with all its required components
/// let proof = PHGRProof::with_fields(
///     g1_point.clone(), g1_point.clone(), g1_point.clone(), g1_point.clone(),
///     g1_point.clone(), g1_point.clone(), g1_point.clone(), g1_point.clone()
/// );
///
/// // Convert to raw bytes
/// let proof_bytes = proof.to_bytes();
/// assert_eq!(proof_bytes.len(), 8 * 33); // 264 bytes total
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct PHGRProof {
    /// First proof component (A point)
    g_a: CompressedG1,
    /// A-prime proof component
    g_a_prime: CompressedG1,
    /// B point in the proof
    g_b: CompressedG1,
    /// B-prime point in the proof
    g_b_prime: CompressedG1,
    /// C point in the proof
    g_c: CompressedG1,
    /// C-prime point in the proof
    g_c_prime: CompressedG1,
    /// K point in the proof
    g_k: CompressedG1,
    /// H point in the proof
    g_h: CompressedG1,
}

impl PHGRProof {
    #[allow(clippy::too_many_arguments)]
    pub fn with_fields(
        g_a: CompressedG1,
        g_a_prime: CompressedG1,
        g_b: CompressedG1,
        g_b_prime: CompressedG1,
        g_c: CompressedG1,
        g_c_prime: CompressedG1,
        g_k: CompressedG1,
        g_h: CompressedG1,
    ) -> Self {
        Self {
            g_a,
            g_a_prime,
            g_b,
            g_b_prime,
            g_c,
            g_c_prime,
            g_k,
            g_h,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(8 * 33);
        result.extend_from_slice(self.g_a.as_slice());
        result.extend_from_slice(self.g_a_prime.as_slice());
        result.extend_from_slice(self.g_b.as_slice());
        result.extend_from_slice(self.g_b_prime.as_slice());
        result.extend_from_slice(self.g_c.as_slice());
        result.extend_from_slice(self.g_c_prime.as_slice());
        result.extend_from_slice(self.g_k.as_slice());
        result.extend_from_slice(self.g_h.as_slice());
        result
    }

    pub fn g_a(&self) -> &CompressedG1 {
        &self.g_a
    }

    pub fn g_a_prime(&self) -> &CompressedG1 {
        &self.g_a_prime
    }

    pub fn g_b(&self) -> &CompressedG1 {
        &self.g_b
    }

    pub fn g_b_prime(&self) -> &CompressedG1 {
        &self.g_b_prime
    }

    pub fn g_c(&self) -> &CompressedG1 {
        &self.g_c
    }

    pub fn g_c_prime(&self) -> &CompressedG1 {
        &self.g_c_prime
    }

    pub fn g_k(&self) -> &CompressedG1 {
        &self.g_k
    }

    pub fn g_h(&self) -> &CompressedG1 {
        &self.g_h
    }
}

impl Parse for PHGRProof {
    fn parse(p: &mut Parser) -> Result<Self> {
        let g_a = parse!(p, "g_a")?;
        let g_a_prime = parse!(p, "g_a_prime")?;
        let g_b = parse!(p, "g_b")?;
        let g_b_prime = parse!(p, "g_b_prime")?;
        let g_c = parse!(p, "g_c")?;
        let g_c_prime = parse!(p, "g_c_prime")?;
        let g_k = parse!(p, "g_k")?;
        let g_h = parse!(p, "g_h")?;
        Ok(Self::with_fields(
            g_a, g_a_prime, g_b, g_b_prime, g_c, g_c_prime, g_k, g_h,
        ))
    }
}

impl From<PHGRProof> for Envelope {
    fn from(value: PHGRProof) -> Self {
        Envelope::new(CBOR::to_byte_string(value.to_bytes()))
            .add_type("PHGRProof")
    }
}

impl TryFrom<Envelope> for PHGRProof {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self, Self::Error> {
        envelope.check_type_envelope("PHGRProof").context("PHGRProf")?;
        let bytes = envelope.subject().try_byte_string().context("bytes")?;
        let proof = parse!(buf = &bytes, PHGRProof, "PHGRProof")?;
        Ok(proof)
    }
}

#[cfg(test)]
impl crate::RandomInstance for PHGRProof {
    fn random() -> Self {
        Self {
            g_a: CompressedG1::random(),
            g_a_prime: CompressedG1::random(),
            g_b: CompressedG1::random(),
            g_b_prime: CompressedG1::random(),
            g_c: CompressedG1::random(),
            g_c_prime: CompressedG1::random(),
            g_k: CompressedG1::random(),
            g_h: CompressedG1::random(),
        }
    }
}

test_envelope_roundtrip!(PHGRProof);
