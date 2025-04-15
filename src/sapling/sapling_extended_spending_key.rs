use anyhow::{Context, Result};
use bc_envelope::prelude::*;

use crate::{parse, test_envelope_roundtrip};
use super::super::parser::prelude::*;
use super::super::u256;

use super::SaplingExpandedSpendingKey;

/// A hierarchical deterministic (HD) Sapling spending key with derivation information.
///
/// `SaplingExtendedSpendingKey` extends the core spending key functionality by adding the
/// necessary components for hierarchical deterministic (HD) key derivation according to
/// ZIP-32 (Zcash's equivalent of BIP-32). This enables the creation of structured wallet
/// hierarchies with parent-child key relationships.
///
/// # Zcash Concept Relation
/// In Zcash's HD wallet structure (ZIP-32):
///
/// - **Hierarchical Keys**: Form a tree structure where child keys can be derived from parent keys
/// - **Key Derivation Paths**: e.g., m/44'/133'/0'/0/5 indicates specific positions in the hierarchy
/// - **Sapling-Specific**: ZIP-32 extends BIP-32 with Sapling-specific cryptography
///
/// The key components include:
/// - The basic spending key components (ask, nsk, ovk) in the `expsk` field
/// - HD derivation components (depth, parent tag, child index, chain code)
/// - A diversifier key (dk) for generating multiple addresses from a single key
///
/// # Data Preservation
/// During wallet migration, all components must be preserved exactly to maintain:
///
/// - Spending capability for all derived addresses
/// - The ability to derive new child keys
/// - Proper wallet structure and hierarchy
/// - The capability to generate multiple diversified addresses
///
/// # Examples
/// ```
/// # use zewif::{sapling::{SaplingExtendedSpendingKey, SaplingExpandedSpendingKey}, u256};
/// // Create the expanded spending key components
/// let ask = u256::default();
/// let nsk = u256::default();
/// let ovk = u256::default();
/// let expsk = SaplingExpandedSpendingKey { ask, nsk, ovk };
///
/// // Create HD wallet components
/// let depth = 3; // Depth in the hierarchy
/// let parent_tag = 0x12345678; // Fingerprint of parent key
/// let child_index = 5; // Index of this key at its depth
/// let chain_code = u256::default(); // For derivation
/// let dk = u256::default(); // For diversified address generation
///
/// // Create the extended spending key
/// let extended_sk = SaplingExtendedSpendingKey {
///     depth,
///     parent_fvk_tag: parent_tag,
///     child_index,
///     chain_code,
///     expsk,
///     dk,
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SaplingExtendedSpendingKey {
    /// Depth in the HD hierarchy (0 for master keys)
    pub depth: u8,
    /// Fingerprint of the parent full viewing key (0 for master keys)
    pub parent_fvk_tag: u32,
    /// Index of this key among its siblings (normal or hardened)
    pub child_index: u32,
    /// Entropy for child key derivation
    pub chain_code: u256,
    /// The core spending key components
    pub expsk: SaplingExpandedSpendingKey,
    /// Diversifier key for generating multiple addresses
    pub dk: u256,
}

/// Implementation of the Parse trait for binary deserialization
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

impl From<SaplingExtendedSpendingKey> for Envelope {
    fn from(value: SaplingExtendedSpendingKey) -> Self {
        Envelope::new(value.expsk)
            .add_type("SaplingExtendedSpendingKey")
            .add_assertion("depth", value.depth)
            .add_assertion("parent_fvk_tag", value.parent_fvk_tag)
            .add_assertion("child_index", value.child_index)
            .add_assertion("chain_code", value.chain_code)
            .add_assertion("dk", value.dk)
    }
}

impl TryFrom<Envelope> for SaplingExtendedSpendingKey {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self, Self::Error> {
        envelope.check_type_envelope("SaplingExtendedSpendingKey").context("SaplingExtendedSpendingKey")?;
        let expsk = envelope.try_as().context("expsk")?;
        let depth = envelope.extract_object_for_predicate("depth").context("depth")?;
        let parent_fvk_tag = envelope.extract_object_for_predicate("parent_fvk_tag").context("parent_fvk_tag")?;
        let child_index = envelope.extract_object_for_predicate("child_index").context("child_index")?;
        let chain_code = envelope.extract_object_for_predicate("chain_code").context("chain_code")?;
        let dk = envelope.extract_object_for_predicate("dk").context("dk")?;
        Ok(SaplingExtendedSpendingKey {
            expsk,
            depth,
            parent_fvk_tag,
            child_index,
            chain_code,
            dk,
        })
    }
}

#[cfg(test)]
impl crate::RandomInstance for SaplingExtendedSpendingKey {
    fn random() -> Self {
        Self {
            depth: u8::random(),
            parent_fvk_tag: u32::random(),
            child_index: u32::random(),
            chain_code: u256::random(),
            expsk: SaplingExpandedSpendingKey::random(),
            dk: u256::random(),
        }
    }
}

test_envelope_roundtrip!(SaplingExtendedSpendingKey);
