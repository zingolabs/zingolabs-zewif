use crate::blob;

pub const GROTH_PROOF_SIZE: usize = 48 + 96 + 48;

blob!(GrothProof, GROTH_PROOF_SIZE);
