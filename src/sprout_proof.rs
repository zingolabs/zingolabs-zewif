use anyhow::Result;

use super::{parse, parser::prelude::*};
use super::{GrothProof, PHGRProof};

#[derive(Debug, Clone, PartialEq)]
pub enum SproutProof {
    PHGRProof(PHGRProof),
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
