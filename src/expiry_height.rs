use anyhow::Result;

use crate::{parse, parser::prelude::*};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ExpiryHeight(u32);

impl ExpiryHeight {
    pub fn as_option(self) -> Option<Self> {
        if self.0 == 0 { None } else { Some(self) }
    }
}

impl Parse for ExpiryHeight {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(ExpiryHeight::from(parse!(p, u32, "expiry_height")?))
    }
}

impl From<u32> for ExpiryHeight {
    fn from(expiry_height: u32) -> Self {
        ExpiryHeight(expiry_height)
    }
}

impl From<ExpiryHeight> for u32 {
    fn from(expiry_height: ExpiryHeight) -> Self {
        expiry_height.0
    }
}
