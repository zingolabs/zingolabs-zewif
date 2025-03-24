use anyhow::Result;

use crate::{parse, parser::prelude::*};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BranchId {
    /// The consensus rules at the launch of Zcash.
    Sprout,
    /// The consensus rules deployed by [`NetworkUpgrade::Overwinter`].
    Overwinter,
    /// The consensus rules deployed by [`NetworkUpgrade::Sapling`].
    Sapling,
    /// The consensus rules deployed by [`NetworkUpgrade::Blossom`].
    Blossom,
    /// The consensus rules deployed by [`NetworkUpgrade::Heartwood`].
    Heartwood,
    /// The consensus rules deployed by [`NetworkUpgrade::Canopy`].
    Canopy,
    /// The consensus rules deployed by [`NetworkUpgrade::Nu5`].
    Nu5,
    /// The consensus rules deployed by [`NetworkUpgrade::Nu6`].
    Nu6,
    /// Candidates for future consensus rules; this branch will never
    /// activate on mainnet.
    ZFuture,
}

impl TryFrom<u32> for BranchId {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BranchId::Sprout),
            0x5ba8_1b19 => Ok(BranchId::Overwinter),
            0x76b8_09bb => Ok(BranchId::Sapling),
            0x2bb4_0e60 => Ok(BranchId::Blossom),
            0xf5b9_230b => Ok(BranchId::Heartwood),
            0xe9ff_75a6 => Ok(BranchId::Canopy),
            0xc2d6_d0b4 => Ok(BranchId::Nu5),
            0xc8e7_1055 => Ok(BranchId::Nu6),
            0xffff_ffff => Ok(BranchId::ZFuture),
            _ => Err("Unknown consensus branch ID"),
        }
    }
}

impl From<BranchId> for u32 {
    fn from(consensus_branch_id: BranchId) -> u32 {
        match consensus_branch_id {
            BranchId::Sprout => 0,
            BranchId::Overwinter => 0x5ba8_1b19,
            BranchId::Sapling => 0x76b8_09bb,
            BranchId::Blossom => 0x2bb4_0e60,
            BranchId::Heartwood => 0xf5b9_230b,
            BranchId::Canopy => 0xe9ff_75a6,
            BranchId::Nu5 => 0xc2d6_d0b4,
            BranchId::Nu6 => 0xc8e7_1055,
            BranchId::ZFuture => 0xffff_ffff,
        }
    }
}

impl Parse for BranchId {
    fn parse(p: &mut Parser) -> Result<Self> {
        let consensus_branch_id = parse!(p, u32, "consensus branch ID")?;
        BranchId::try_from(consensus_branch_id)
            .map_err(|_| anyhow::anyhow!("Unknown consensus branch ID: {}", consensus_branch_id))
    }
}
