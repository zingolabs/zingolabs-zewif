use super::super::{NoQuotesDebugOption, u256};
use super::SaplingExpandedSpendingKey;

/// Represents a Sapling spending key which contains cryptographic material
/// necessary to spend funds and view transaction details, along with optional
/// hierarchical deterministic (HD) wallet components.
///
/// # Zcash Concept Relation
/// In Zcash's Sapling protocol, spending keys are the master keys that provide full
/// control over funds. The Sapling spending key consists of:
///
/// - `ask` (spending authorization key): Used to sign transactions
/// - `nsk` (nullifier private key): Used to create nullifiers for spent notes
/// - `ovk` (outgoing viewing key): Used to view outgoing transaction details
///
/// When used in HD wallets (ZIP-32), additional components are used for key derivation:
///
/// - `depth`: Position in the HD tree hierarchy
/// - `parent_fingerprint`: Identifier for the parent key
/// - `child_index`: Index of this key among its siblings
/// - `chain_code`: Entropy used in derivation
/// - `dk` (diversifier key): Used to derive diversifiers for addresses
///
/// # Data Preservation
/// This structure preserves the complete cryptographic material needed to spend funds,
/// including both the core components (ask, nsk, ovk) and the optional HD wallet
/// components needed for key derivation and address generation.
#[derive(Clone)]
pub struct SaplingSpendingKey {
    /// The expanded spending key containing core cryptographic components
    pub expsk: SaplingExpandedSpendingKey,
    /// Depth in the HD hierarchy
    pub depth: Option<u8>,
    /// Parent full viewing key fingerprint
    pub parent_fingerprint: Option<u32>,
    /// Child index in the HD hierarchy
    pub child_index: Option<u32>,
    /// Chain code for HD derivation
    pub chain_code: Option<u256>,
    /// Diversifier key
    pub dk: Option<u256>,
}

impl SaplingSpendingKey {
    /// Creates a new Sapling spending key with just the essential components.
    ///
    /// This constructs a basic Sapling spending key with only the core cryptographic
    /// components (`ask`, `nsk`, `ovk`) and no HD wallet information. This is suitable
    /// for simple non-HD wallets or when the HD information will be added later.
    ///
    /// # Arguments
    /// * `ask` - Spending authorization key for signing transactions
    /// * `nsk` - Nullifier private key for creating nullifiers
    /// * `ovk` - Outgoing viewing key for viewing transaction details
    ///
    /// # Examples
    /// ```
    /// # use zewif::{sapling::SaplingSpendingKey, u256};
    /// #
    /// let ask = u256::default();
    /// let nsk = u256::default();
    /// let ovk = u256::default();
    ///
    /// let sk = SaplingSpendingKey::new(ask, nsk, ovk);
    /// ```
    pub fn new(ask: u256, nsk: u256, ovk: u256) -> Self {
        SaplingSpendingKey {
            expsk: SaplingExpandedSpendingKey { ask, nsk, ovk },
            depth: None,
            parent_fingerprint: None,
            child_index: None,
            chain_code: None,
            dk: None,
        }
    }

    /// Creates a new complete Sapling extended spending key with all HD components.
    ///
    /// This constructs a full Sapling spending key with both the core cryptographic
    /// components and all the hierarchical deterministic (ZIP-32) wallet information.
    /// This is suitable for HD wallets that need full derivation capabilities.
    ///
    /// # Arguments
    /// * `ask` - Spending authorization key for signing transactions
    /// * `nsk` - Nullifier private key for creating nullifiers
    /// * `ovk` - Outgoing viewing key for viewing transaction details
    /// * `depth` - Depth in the HD hierarchy
    /// * `parent_fingerprint` - Fingerprint of the parent key
    /// * `child_index` - Index of this key among its siblings
    /// * `chain_code` - Entropy used in derivation
    /// * `dk` - Diversifier key for address generation
    ///
    /// # Examples
    /// ```
    /// # use zewif::{sapling::SaplingSpendingKey, u256};
    /// #
    /// let ask = u256::default();
    /// let nsk = u256::default();
    /// let ovk = u256::default();
    /// let chain_code = u256::default();
    /// let dk = u256::default();
    ///
    /// let extended_sk = SaplingSpendingKey::new_extended(
    ///     ask, nsk, ovk,
    ///     1, // depth
    ///     0, // parent_fingerprint
    ///     5, // child_index
    ///     chain_code,
    ///     dk
    /// );
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn new_extended(
        ask: u256,
        nsk: u256,
        ovk: u256,
        depth: u8,
        parent_fingerprint: u32,
        child_index: u32,
        chain_code: u256,
        dk: u256,
    ) -> Self {
        SaplingSpendingKey {
            expsk: SaplingExpandedSpendingKey { ask, nsk, ovk },
            depth: Some(depth),
            parent_fingerprint: Some(parent_fingerprint),
            child_index: Some(child_index),
            chain_code: Some(chain_code),
            dk: Some(dk),
        }
    }
}

impl std::fmt::Debug for SaplingSpendingKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SaplingSpendingKey")
            .field("ask", &self.expsk.ask)
            .field("nsk", &self.expsk.nsk)
            .field("ovk", &self.expsk.ovk)
            .field("depth", &NoQuotesDebugOption(&self.depth))
            .field("parent_fingerprint", &NoQuotesDebugOption(&self.parent_fingerprint))
            .field("child_index", &NoQuotesDebugOption(&self.child_index))
            .field("chain_code", &NoQuotesDebugOption(&self.chain_code))
            .field("dk", &NoQuotesDebugOption(&self.dk))
            .finish()
    }
}

// TODO: Add binary serialization/deserialization after establishing proper requirements
// and test vectors. Binary compatibility is critical and needs thorough validation.
