use anyhow::{Error, Result, bail};

use crate::parse;
use super::parser::prelude::*;
use super::Blob32;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[allow(non_camel_case_types)]
pub struct u256([u8; 32]);

impl u256 {
    pub fn from_hex(hex: &str) -> Self {
        let blob = Blob32::from_hex(hex);
        Self(blob.into())
    }
}

impl TryFrom<&[u8]> for u256 {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != 32 {
            bail!("Invalid data length: expected 32, got {}", bytes.len());
        }
        let mut a = [0u8; 32];
        a.copy_from_slice(bytes);
        Ok(Self(a))
    }
}

impl TryFrom<&[u8; 32]> for u256 {
    type Error = Error;

    fn try_from(bytes: &[u8; 32]) -> Result<Self, Self::Error> {
        Ok(Self(*bytes))
    }
}

impl TryFrom<&Vec<u8>> for u256 {
    type Error = Error;

    fn try_from(bytes: &Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl AsRef<[u8]> for u256 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; 32]> for u256 {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl std::fmt::Debug for u256 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut bytes = self.0;
        bytes.reverse();
        write!(f, "u256({})", hex::encode(bytes))
    }
}

impl std::fmt::Display for u256 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut bytes = self.0;
        bytes.reverse();
        write!(f, "{}", hex::encode(bytes))
    }
}

impl Parse for u256 {
    fn parse(p: &mut Parser) -> Result<Self> {
        let bytes = parse!(p, "u256")?;
        Ok(Self(bytes))
    }
}
