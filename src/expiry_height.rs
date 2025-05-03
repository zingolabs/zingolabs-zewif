use bc_envelope::prelude::*;

/// The block height at which a Zcash transaction expires if not yet mined.
///
/// `ExpiryHeight` represents a consensus rule in Zcash that allows transactions to
/// specify a block height after which they should no longer be included in blocks.
/// This feature prevents transactions from remaining in the mempool indefinitely and
/// enables time-bound validity for transactions.
///
/// # Zcash Concept Relation
/// In Zcash's transaction model:
///
/// - **Time-Bound Validity**: Expiry heights allow transactions to be valid only for a
///   specific window of blocks, providing time-limited transaction proposals
///
/// - **Mempool Management**: Expired transactions are automatically removed from node mempools,
///   helping to manage mempool size and prevent lingering transactions
///
/// - **Special Value 0**: A value of 0 is interpreted as "no expiry", meaning the transaction
///   doesn't expire and remains valid indefinitely (until mined or explicitly dropped)
///
/// # Data Preservation
/// During wallet migration, the following aspects of expiry heights are preserved:
///
/// - **Exact Height Value**: The precise block height at which a transaction expires
/// - **No-Expiry Semantics**: The special case of 0 meaning "no expiry"
///
/// # Examples
/// ```
/// # use zewif::ExpiryHeight;
/// // Create an expiry height for 20 blocks in the future
/// let current_height = 1_000_000;
/// let expiry = ExpiryHeight::from(current_height + 20);
///
/// // Check if it's a valid expiry (non-zero)
/// let opt_expiry = expiry.as_option();
/// assert!(opt_expiry.is_some());
///
/// // Create a no-expiry value
/// let no_expiry = ExpiryHeight::from(0u32);
/// assert!(no_expiry.as_option().is_none()); // Converts to None
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ExpiryHeight(u32);

impl ExpiryHeight {
    /// Converts the expiry height to an Option, returning None if the height is 0.
    ///
    /// In Zcash, an expiry height of 0 is interpreted as "no expiry" - the transaction
    /// never expires. This method provides a convenient way to represent this semantic
    /// by returning None for a zero height, which can simplify logic in calling code.
    ///
    /// # Returns
    /// - `None` if the expiry height is 0 (no expiry)
    /// - `Some(ExpiryHeight)` for any non-zero height
    ///
    /// # Examples
    /// ```
    /// # use zewif::ExpiryHeight;
    /// // No expiry (height 0)
    /// let no_expiry = ExpiryHeight::from(0u32);
    /// assert!(no_expiry.as_option().is_none());
    ///
    /// // Specific expiry height
    /// let with_expiry = ExpiryHeight::from(1_050_000u32);
    /// assert!(with_expiry.as_option().is_some());
    /// ```
    pub fn as_option(self) -> Option<Self> {
        if self.0 == 0 { None } else { Some(self) }
    }
}

impl From<u32> for ExpiryHeight {
    fn from(expiry_height: u32) -> Self {
        ExpiryHeight(expiry_height)
    }
}

impl From<ExpiryHeight> for u32 {
    fn from(expiry_height: ExpiryHeight) -> Self {
        expiry_height.0
    }
}

impl From<ExpiryHeight> for CBOR {
    fn from(expiry_height: ExpiryHeight) -> Self {
        CBOR::from(expiry_height.0)
    }
}

impl TryFrom<CBOR> for ExpiryHeight {
    type Error = dcbor::Error;

    fn try_from(cbor: CBOR) -> dcbor::Result<Self> {
        // let expiry_height: u32 = cbor.try_into()?;
        // Ok(ExpiryHeight::from(expiry_height))
        cbor.try_into().map(ExpiryHeight)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_cbor_roundtrip;

    use super::ExpiryHeight;

    impl crate::RandomInstance for ExpiryHeight {
        fn random() -> Self {
            let mut rng = rand::thread_rng();
            let expiry_height = rand::Rng::gen_range(&mut rng, 0..=u32::MAX);
            ExpiryHeight::from(expiry_height)
        }
    }

    test_cbor_roundtrip!(ExpiryHeight);
}
