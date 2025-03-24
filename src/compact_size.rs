use anyhow::{Result, bail};

use crate::{parse, parser::prelude::*};

pub fn parse_compact_size(p: &mut Parser) -> Result<usize> {
    match parse!(p, u8, "compact size")? {
        0xfd => {
            let n = parse!(p, u16, "compact size")?;
            if n < 253 {
                bail!("Compact size with 0xfd prefix must be >= 253, got {}", n);
            }
            Ok(n as usize)
        }
        0xfe => {
            let n = parse!(p, u32, "compact size")?;
            if n < 0x10000 {
                bail!(
                    "Compact size with 0xfe prefix must be >= 0x10000, got {}",
                    n
                );
            }
            Ok(n as usize)
        }
        0xff => {
            let n = parse!(p, u64, "compact size")?;
            if n < 0x100000000 {
                bail!(
                    "Compact size with 0xff prefix must be >= 0x100000000, got {}",
                    n
                );
            }
            Ok(n as usize)
        }
        size => Ok(size as usize),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompactSize(usize);

impl std::fmt::Display for CompactSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Parse for CompactSize {
    fn parse(p: &mut Parser) -> Result<Self> {
        parse_compact_size(p).map(CompactSize)
    }
}

impl std::ops::Deref for CompactSize {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
