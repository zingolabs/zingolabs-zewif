use anyhow::Result;

use crate::{parse, parser::prelude::*};

/// Identifies the consensus rules in effect for a particular block or transaction.
///
/// `BranchId` represents the different network upgrades in Zcash's history, each of which
/// introduced new consensus rules and protocol features. The branch ID is used for
/// transaction signature validation and to determine which consensus rules apply
/// at a given block height.
///
/// # Zcash Concept Relation
/// Zcash evolves through planned network upgrades that activate at specific block heights.
/// Each upgrade has a unique ID (branch ID) used to:
///
/// - **Replay protection**: Prevents transactions from one network upgrade being valid on another
/// - **Versioning**: Indicates which transaction format and validation rules apply
/// - **Activation**: Determines which features are available based on block height
///
/// Unlike most blockchains that use simple version numbers, Zcash uses unique 32-bit values 
/// for each upgrade to ensure strong transaction replay protection between different
/// network upgrade eras.
///
/// # Data Preservation
/// During wallet migration, preserving branch IDs is critical for:
/// - Transaction validation, especially for partially signed transactions
/// - Correctly applying activation rules to wallet data
/// - Understanding which privacy and consensus features were available when transactions occurred
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BranchId {
    /// The consensus rules at the launch of Zcash.
    Sprout,
    /// The consensus rules for the Overwinter network upgrade.
    Overwinter,
    /// The consensus rules for the Sapling network upgrade, which introduced
    /// the Sapling shielded pool.
    Sapling,
    /// The consensus rules for the Blossom network upgrade.
    Blossom,
    /// The consensus rules for the Heartwood network upgrade.
    Heartwood,
    /// The consensus rules for the Canopy network upgrade.
    Canopy,
    /// The consensus rules for the NU5 (Network Upgrade 5) which introduced
    /// the Orchard shielded pool and Halo 2 proving system.
    Nu5,
    /// The consensus rules for the NU6 (Network Upgrade 6).
    Nu6,
    /// Candidates for future consensus rules; this branch will never
    /// activate on mainnet.
    ZFuture,
}

/// Converts a 32-bit numeric branch ID into the corresponding enum variant
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

/// Converts a BranchId enum variant to its raw 32-bit numeric value
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

/// Parses a BranchId from a binary data stream
impl Parse for BranchId {
    fn parse(p: &mut Parser) -> Result<Self> {
        let consensus_branch_id = parse!(p, u32, "consensus branch ID")?;
        BranchId::try_from(consensus_branch_id)
            .map_err(|_| anyhow::anyhow!("Unknown consensus branch ID: {}", consensus_branch_id))
    }
}
