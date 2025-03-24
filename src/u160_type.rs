use anyhow::{Context, Result};

use crate::parse;
use super::parser::prelude::*;
use super::Blob20;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[allow(non_camel_case_types)]
pub struct u160([u8; 20]);

impl u160 {
    pub fn from_blob(blob: Blob20) -> Self {
        Self(blob.into())
    }

    pub fn from_slice(bytes: &[u8]) -> Result<Self> {
        let blob = Blob20::from_slice(bytes).context("Creating U160 from slice")?;
        Ok(Self(blob.into()))
    }
}

impl AsRef<[u8]> for u160 {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<[u8; 20]> for u160 {
    fn as_ref(&self) -> &[u8; 20] {
        &self.0
    }
}

impl std::fmt::Debug for u160 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut bytes = self.0;
        bytes.reverse();
        write!(f, "u160({})", hex::encode(bytes))
    }
}

impl std::fmt::Display for u160 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut bytes = self.0;
        bytes.reverse();
        write!(f, "{}", hex::encode(bytes))
    }
}

impl Parse for u160 {
    fn parse(p: &mut Parser) -> Result<Self> {
        let blob = parse!(p, Blob20, "u160")?;
        Ok(Self(blob.into())) // Update to use as_slice() here
    }
}
