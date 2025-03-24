use super::{Blob, u256};

/// Represents a spending key which contains cryptographic material
/// necessary to spend funds and view transaction details
#[derive(Debug, Clone)]
pub enum SpendingKey {
    /// Sapling spending key with its components
    Sapling {
        /// Spending authorization key - enables transaction signing
        ask: u256,
        /// Nullifier spending key - required for creating nullifiers to prevent double-spending
        nsk: u256,
        /// Outgoing viewing key - allows viewing outgoing transaction details
        ovk: u256,

        // Extended spending key components (ZIP-32 HD wallet)
        /// Depth in the HD hierarchy
        depth: Option<u8>,
        /// Parent fingerprint (parent_fvk_tag)
        parent_fingerprint: Option<u32>,
        /// Child index in the HD hierarchy
        child_index: Option<u32>,
        /// Chain code for HD derivation
        chain_code: Option<u256>,
        /// Diversifier key
        dk: Option<u256>,
    },
    /// Raw key data format for backward compatibility or other protocols
    Raw(Blob<32>),
}

impl SpendingKey {
    /// Create a new Sapling spending key with the essential components
    pub fn new_sapling(ask: u256, nsk: u256, ovk: u256) -> Self {
        SpendingKey::Sapling {
            ask,
            nsk,
            ovk,
            depth: None,
            parent_fingerprint: None,
            child_index: None,
            chain_code: None,
            dk: None,
        }
    }

    /// Create a new complete Sapling extended spending key with all HD components
    #[allow(clippy::too_many_arguments)]
    pub fn new_sapling_extended(
        ask: u256,
        nsk: u256,
        ovk: u256,
        depth: u8,
        parent_fingerprint: u32,
        child_index: u32,
        chain_code: u256,
        dk: u256,
    ) -> Self {
        SpendingKey::Sapling {
            ask,
            nsk,
            ovk,
            depth: Some(depth),
            parent_fingerprint: Some(parent_fingerprint),
            child_index: Some(child_index),
            chain_code: Some(chain_code),
            dk: Some(dk),
        }
    }

    /// Create a raw spending key (for backward compatibility)
    pub fn new_raw(key_data: Blob<32>) -> Self {
        SpendingKey::Raw(key_data)
    }
}
