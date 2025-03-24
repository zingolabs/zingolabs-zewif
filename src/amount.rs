use anyhow::Result;

use crate::parse;
use super::parser::prelude::*;

pub type Amount = zcash_protocol::value::ZatBalance;

impl Parse for Amount {
    fn parse(p: &mut Parser) -> Result<Self> {
        let zat_balance = parse!(p, i64, "Zat balance")?;
        Amount::try_from(zat_balance)
            .map_err(|_| anyhow::anyhow!("Invalid Zat balance: {}", zat_balance))
    }
}
