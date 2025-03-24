use crate::{parse, parser::prelude::*};

pub type BlockHeight = zcash_primitives::consensus::BlockHeight;

impl Parse for BlockHeight {
    fn parse(p: &mut Parser) -> anyhow::Result<Self> {
        let height = parse!(p, u32, "BlockHeight")?;
        Ok(BlockHeight::from(height))
    }
}
