use anyhow::Result;

use crate::{parse, parser::prelude::*};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct IntID(u32);

impl IntID {
    pub const fn new(id: u32) -> Self {
        IntID(id)
    }
}

impl std::fmt::Display for IntID {
    // Always display as hex with `0x` prefix
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:08x}", self.0)
    }
}

impl std::fmt::Debug for IntID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Parse for IntID {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self(parse!(p, "IntID")?))
    }
}
impl From<u32> for IntID {
    fn from(id: u32) -> Self {
        IntID(id)
    }
}

impl From<IntID> for u32 {
    fn from(id: IntID) -> Self {
        id.0
    }
}
