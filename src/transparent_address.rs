use super::{DerivationInfo, TransparentSpendAuthority};

/// A Bitcoin-compatible transparent address in the Zcash network.
///
/// `TransparentAddress` represents addresses in Zcash that begin with 't' and
/// function similarly to Bitcoin addresses. These addresses offer no privacy
/// features - all transaction data is visible on the blockchain.
///
/// # Zcash Concept Relation
/// Zcash supports transparent addresses for backward compatibility with Bitcoin
/// infrastructure. Two main types exist:
///
/// - **P2PKH** (Pay to Public Key Hash): Standard addresses that begin with 't1'
/// - **P2SH** (Pay to Script Hash): Script-based addresses that begin with 't3'
///
/// Transparent addresses make transaction data publicly visible, including:
/// - Sender address
/// - Receiver address
/// - Transaction amount
/// - Transaction time
///
/// # Data Preservation
/// During wallet migration, the following components are preserved:
///
/// - **Address string**: The canonical string representation (e.g., "t1...")
/// - **Spending authority**: Private key information needed to spend funds
/// - **Derivation information**: HD wallet path data for derived addresses
///
/// # Examples
/// ```
/// use zewif::{TransparentAddress, TransparentSpendAuthority, DerivationInfo, NonHardenedChildIndex};
///
/// // Create a new transparent address
/// let mut address = TransparentAddress::new("t1exampleaddress");
///
/// // Set the spending authority (usually a derived key for HD wallets)
/// let spend_authority = TransparentSpendAuthority::Derived;
/// address.set_spend_authority(spend_authority);
///
/// // For HD wallets, set the derivation information
/// let change = NonHardenedChildIndex::from(0u32); // external chain
/// let address_index = NonHardenedChildIndex::from(3u32); // 4th address in chain
/// let derivation_info = DerivationInfo::new(change, address_index);
/// address.set_derivation_info(derivation_info);
///
/// // Access the address string
/// assert_eq!(address.address(), "t1exampleaddress");
/// ```
#[derive(Debug, Clone)]
pub struct TransparentAddress {
    /// The transparent address string (starting with 't')
    /// This is used as a unique identifier within the wallet
    address: String, // Unique
    
    /// Optional spending authority for this address
    /// When present, this contains the information needed to spend funds
    spend_authority: Option<TransparentSpendAuthority>,
    
    /// Optional HD wallet derivation information
    /// When present, this contains the path information for HD wallets
    derivation_info: Option<DerivationInfo>,
}

impl TransparentAddress {
    /// Creates a new transparent address with the given address string.
    ///
    /// This constructor creates a basic transparent address with just the
    /// address string. Spending authority and derivation information can
    /// be added later if available.
    ///
    /// # Arguments
    /// * `address` - The transparent address string (e.g., "t1...")
    ///
    /// # Examples
    /// ```
    /// use zewif::TransparentAddress;
    ///
    /// let address = TransparentAddress::new("t1exampleaddress");
    /// assert_eq!(address.address(), "t1exampleaddress");
    /// ```
    pub fn new(address: impl Into<String>) -> Self {
        TransparentAddress {
            address: address.into(),
            spend_authority: None,
            derivation_info: None,
        }
    }

    /// Returns the transparent address string.
    ///
    /// # Returns
    /// The canonical string representation of this transparent address.
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Returns the spending authority for this address, if available.
    ///
    /// The spending authority contains the information needed to spend
    /// funds from this address, either as a direct key or as a reference
    /// to a derived key.
    ///
    /// # Returns
    /// - `Some(&TransparentSpendAuthority)` if spending capability is available
    /// - `None` if this is a watch-only address without spending capability
    pub fn spend_authority(&self) -> Option<&TransparentSpendAuthority> {
        self.spend_authority.as_ref()
    }

    /// Sets the spending authority for this address.
    ///
    /// This method associates spending capability with the address, allowing
    /// funds to be spent from it. The authority can be either a direct key
    /// or a reference to a derived key from an HD wallet.
    ///
    /// # Arguments
    /// * `spend_authority` - The spending authority to associate with this address
    pub fn set_spend_authority(&mut self, spend_authority: TransparentSpendAuthority) {
        self.spend_authority = Some(spend_authority);
    }

    /// Returns the HD wallet derivation information for this address, if available.
    ///
    /// For addresses derived from an HD wallet seed, this provides the path
    /// information necessary to regenerate the address.
    ///
    /// # Returns
    /// - `Some(&DerivationInfo)` if this address has derivation information
    /// - `None` if this is not an HD wallet derived address or the information is unavailable
    pub fn derivation_info(&self) -> Option<&DerivationInfo> {
        self.derivation_info.as_ref()
    }

    /// Sets the HD wallet derivation information for this address.
    ///
    /// This method associates HD path information with the address, which is useful
    /// for addresses derived from a hierarchical deterministic wallet.
    ///
    /// # Arguments
    /// * `derivation_info` - The derivation path information to associate with this address
    pub fn set_derivation_info(&mut self, derivation_info: DerivationInfo) {
        self.derivation_info = Some(derivation_info);
    }
}
