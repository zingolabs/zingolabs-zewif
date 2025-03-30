use super::{Blob, sapling::SaplingSpendingKey};

/// A spending key that provides full control over funds, enabling transaction creation
/// and authorization for a specific address in a Zcash wallet.
///
/// `SpendingKey` is a protocol-agnostic wrapper for various Zcash spending key types.
/// It contains the cryptographic material necessary to spend funds and view all transaction
/// details associated with an address, making it the most sensitive component of a wallet.
///
/// # Zcash Concept Relation
/// In Zcash, spending keys represent the highest level of control over funds:
///
/// - **Sapling spending keys**: Used for Sapling shielded addresses (zs-prefixed)
/// - **Raw keys**: Used for backward compatibility or other protocols
///
/// The key hierarchy in Zcash allows deriving less-privileged keys from spending keys:
/// ```text
/// Spending Key → Full Viewing Key → Incoming Viewing Key
/// (full control)   (can view all)    (can only view incoming)
/// ```
///
/// # Data Preservation
/// During wallet migration, spending keys are preserved exactly as they exist in the
/// source wallet to maintain complete control over funds. Each variant preserves the
/// appropriate protocol-specific key material.
///
/// # Examples
/// ```
/// use zewif::{SpendingKey, Blob, sapling::SaplingSpendingKey, u256};
///
/// // Create a Sapling spending key
/// let ask = u256::default();
/// let nsk = u256::default();
/// let ovk = u256::default();
/// let sapling_key = SaplingSpendingKey::new(ask, nsk, ovk);
/// let spending_key = SpendingKey::Sapling(sapling_key);
///
/// // Create a raw spending key
/// let raw_key_data = Blob::<32>::default();
/// let raw_spending_key = SpendingKey::Raw(raw_key_data);
/// ```
#[derive(Clone, Debug)]
pub enum SpendingKey {
    /// Sapling protocol spending key with full cryptographic components
    Sapling(SaplingSpendingKey),
    /// Raw key data format for backward compatibility or other protocols
    Raw(Blob<32>),
}

impl SpendingKey {
    /// Creates a new Sapling spending key from a SaplingSpendingKey instance.
    ///
    /// This method wraps a protocol-specific Sapling spending key in the generic
    /// SpendingKey enum.
    ///
    /// # Examples
    /// ```
    /// use zewif::{SpendingKey, sapling::SaplingSpendingKey, u256};
    ///
    /// // Create a Sapling spending key
    /// let ask = u256::default();
    /// let nsk = u256::default();
    /// let ovk = u256::default();
    /// let sapling_key = SaplingSpendingKey::new(ask, nsk, ovk);
    ///
    /// // Wrap it in the generic SpendingKey enum
    /// let spending_key = SpendingKey::new_sapling(sapling_key);
    /// ```
    pub fn new_sapling(key: SaplingSpendingKey) -> Self {
        SpendingKey::Sapling(key)
    }

    /// Creates a raw spending key (for backward compatibility).
    ///
    /// This method creates a spending key that contains raw key material without
    /// additional protocol-specific structure. It's primarily used for backward
    /// compatibility with legacy wallet formats or for protocols that don't have
    /// specialized key structures.
    ///
    /// # Examples
    /// ```
    /// use zewif::{SpendingKey, Blob};
    ///
    /// // Create a raw key from 32 bytes of data
    /// let key_data = Blob::<32>::default();
    /// let raw_key = SpendingKey::new_raw(key_data);
    /// ```
    pub fn new_raw(key_data: Blob<32>) -> Self {
        SpendingKey::Raw(key_data)
    }
}
