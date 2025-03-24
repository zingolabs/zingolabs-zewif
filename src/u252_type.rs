use anyhow::{Result, bail};

use crate::parse;
use super::parser::prelude::*;
use super::Blob32;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[allow(non_camel_case_types)]
pub struct u252([u8; 32]);

impl u252 {
    pub fn from_blob(blob: Blob32) -> Result<Self> {
        let blob: [u8; 32] = blob.into();
        if (blob[0] & 0xf0) != 0 {
            bail!("First four bits of u252 must be zero");
        }
        Ok(Self(blob))
    }
}

impl AsRef<[u8]> for u252 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; 32]> for u252 {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl std::fmt::Debug for u252 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut bytes = self.0;
        bytes.reverse();
        write!(f, "u252({})", hex::encode(bytes))
    }
}

impl std::fmt::Display for u252 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut bytes = self.0;
        bytes.reverse();
        write!(f, "{}", hex::encode(bytes))
    }
}

impl Parse for u252 {
    fn parse(p: &mut Parser) -> Result<Self> {
        let blob = parse!(p, "u252")?;
        Self::from_blob(blob)
    }
}
