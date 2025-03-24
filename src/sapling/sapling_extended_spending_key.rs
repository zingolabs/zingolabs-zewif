use anyhow::Result;

use crate::parse;
use super::super::parser::prelude::*;
use super::super::u256;

use super::SaplingExpandedSpendingKey;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SaplingExtendedSpendingKey {
    pub depth: u8,
    pub parent_fvk_tag: u32,
    pub child_index: u32,
    pub chain_code: u256,
    pub expsk: SaplingExpandedSpendingKey,
    pub dk: u256,
}

impl Parse for SaplingExtendedSpendingKey {
    fn parse(p: &mut Parser) -> Result<Self> {
        let depth = parse!(p, "depth")?;
        let parent_fvk_tag = parse!(p, "parent_fvk_tag")?;
        let child_index = parse!(p, "child_index")?;
        let chain_code = parse!(p, "chain_code")?;
        let expsk = parse!(p, "expsk")?;
        let dk = parse!(p, "dk")?;
        Ok(SaplingExtendedSpendingKey {
            depth,
            parent_fvk_tag,
            child_index,
            chain_code,
            expsk,
            dk,
        })
    }
}
