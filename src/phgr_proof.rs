use anyhow::Result;

use super::{parse, parser::prelude::*};
use super::Blob;

// Initial bool || Fq
pub type CompressedG1 = Blob<33>;

#[derive(Debug, Clone, PartialEq)]
pub struct PHGRProof {
    g_a: CompressedG1,
    g_a_prime: CompressedG1,
    g_b: CompressedG1,
    g_b_prime: CompressedG1,
    g_c: CompressedG1,
    g_c_prime: CompressedG1,
    g_k: CompressedG1,
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
