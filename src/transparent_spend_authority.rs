use super::SpendingKey;

/// The cryptographic authorization needed to spend funds from a transparent Zcash address.
///
/// `TransparentSpendAuthority` represents the spending capability for transparent
/// addresses (those starting with 't'). It distinguishes between directly stored keys
/// and keys that are derived from another source, such as an HD wallet seed.
///
/// # Zcash Concept Relation
/// In Zcash's transparent address system (inherited from Bitcoin):
///
/// - Spending requires proving ownership through cryptographic signatures
/// - This typically involves a private key corresponding to a public key hash (P2PKH)
///   or a script hash (P2SH)
/// - In hierarchical deterministic (HD) wallets, transparent keys are often derived
///   from a master seed using BIP-44 paths
///
/// # Data Preservation
/// During wallet migration, the `TransparentSpendAuthority` preserves:
///
/// - Directly stored spending keys that exist in the source wallet
/// - Information about keys that are derived from HD wallet seeds
///
/// This ensures that spending capability is maintained after migration while
/// preserving the wallet's key management structure.
///
/// # Examples
/// ```
/// use zewif::{TransparentSpendAuthority, SpendingKey, Blob};
///
/// // Direct spending key
/// let raw_key_data = Blob::<32>::default();
/// let spending_key = SpendingKey::new_raw(raw_key_data);
/// let spend_authority = TransparentSpendAuthority::SpendingKey(spending_key);
///
/// // Derived key (from HD wallet seed)
/// let derived_authority = TransparentSpendAuthority::Derived;
/// ```
#[derive(Debug, Clone)]
pub enum TransparentSpendAuthority {
    /// Direct spending key stored in the wallet
    SpendingKey(SpendingKey),
    
    /// Spending key derived from another source (e.g., HD wallet seed)
    /// The actual derivation information is typically stored with the address
    Derived,
}
