use super::parser::prelude::*;
use anyhow::Result;

pub type TxId = zcash_primitives::transaction::TxId;

impl Parse for TxId {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(TxId::read(p)?)
    }
}
