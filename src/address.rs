use crate::{DebugOption, Indexed, test_envelope_roundtrip};
use anyhow::Context;
use bc_envelope::prelude::*;

use super::{ProtocolAddress, ShieldedAddress, TransparentAddress, UnifiedAddress};

/// A high-level address representation with metadata in a Zcash wallet.
///
/// `Address` serves as the primary container for all Zcash addresses, wrapping
/// the protocol-specific address details with additional wallet-level metadata
/// such as a user-assigned name, purpose descriptor, and arbitrary attachments.
/// This structure bridges the raw cryptographic address formats with the
/// user-facing wallet experience.
///
/// # Zcash Concept Relation
/// In Zcash wallets, users typically assign labels or metadata to their addresses
/// for easier identification. `Address` preserves these user-defined attributes
/// alongside the underlying cryptographic address details. It supports all Zcash
/// address protocols:
///
/// - **Transparent addresses**: Bitcoin-compatible addresses (t-prefixed)
/// - **Shielded addresses**: Privacy-focused Sapling/Orchard addresses (z-prefixed)
/// - **Unified addresses**: Multi-protocol addresses (u-prefixed)
///
/// # Data Preservation
/// During wallet migration, the following components are preserved:
///
/// - **Address Data**: The complete protocol-specific address details
/// - **User Labels**: Custom names assigned to addresses by users
/// - **Purpose Strings**: Descriptions of the address's intended use
/// - **Attachments**: Any additional metadata associated with the address
///
/// # Examples
/// ```
/// # use zewif::{Address, ProtocolAddress, TransparentAddress};
/// #
/// // Create a transparent address
/// let t_addr = TransparentAddress::new("t1exampleaddress");
/// let protocol_addr = ProtocolAddress::Transparent(t_addr);
///
/// // Wrap it in an Address with metadata
/// let mut address = Address::new(protocol_addr);
/// address.set_name("Donation Address".to_string());
/// address.set_purpose("Receiving public donations".to_string());
///
/// // Access the address string
/// assert!(address.as_string().starts_with("t1"));
/// assert_eq!(address.name(), "Donation Address");
/// ```
#[derive(Clone, PartialEq)]
pub struct Address {
    /// The index of this address in the wallet
    index: usize,

    /// The underlying protocol-specific address
    address: ProtocolAddress,

    /// User-assigned name/label for this address
    name: String,

    /// Optional description of this address's purpose
    purpose: Option<String>,

    /// Additional metadata attached to this address
    attachments: Attachments,
}

impl Indexed for Address {
    fn index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}

impl std::fmt::Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Address")
            .field("address", &self.address)
            .field("name", &self.name)
            .field("purpose", &DebugOption(&self.purpose))
            .field("attachments", &self.attachments)
            .finish()
    }
}

bc_envelope::impl_attachable!(Address);

impl Address {
    /// Creates a new `Address` with the specified protocol address.
    ///
    /// This constructor creates an `Address` with default empty metadata
    /// (blank name, no purpose) and the provided protocol-specific address.
    ///
    /// # Arguments
    /// * `address` - The protocol-specific address implementation
    ///
    /// # Examples
    /// ```
    /// # use zewif::{Address, ProtocolAddress, TransparentAddress};
    /// #
    /// let t_addr = TransparentAddress::new("t1example");
    /// let protocol_addr = ProtocolAddress::Transparent(t_addr);
    /// let address = Address::new(protocol_addr);
    /// ```
    pub fn new(address: ProtocolAddress) -> Self {
        Self {
            index: 0,
            address,
            name: String::default(),
            purpose: None,
            attachments: Attachments::new(),
        }
    }

    /// Returns the user-assigned name for this address.
    ///
    /// # Returns
    /// The name string assigned to this address, or an empty string if no name has been set.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{Address, ProtocolAddress, TransparentAddress};
    /// #
    /// let t_addr = TransparentAddress::new("t1example");
    /// let protocol_addr = ProtocolAddress::Transparent(t_addr);
    /// let mut address = Address::new(protocol_addr);
    ///
    /// address.set_name("Personal Savings".to_string());
    /// assert_eq!(address.name(), "Personal Savings");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the purpose descriptor for this address, if available.
    ///
    /// # Returns
    /// `Some(&str)` containing the purpose string if set, or `None` if no purpose was assigned.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{Address, ProtocolAddress, TransparentAddress};
    /// #
    /// let mut address = Address::new(ProtocolAddress::Transparent(
    ///     TransparentAddress::new("t1example")
    /// ));
    ///
    /// // Initially there is no purpose
    /// assert!(address.purpose().is_none());
    ///
    /// // Set a purpose and verify it was stored
    /// address.set_purpose("Business expenses".to_string());
    /// assert_eq!(address.purpose(), Some("Business expenses"));
    /// ```
    pub fn purpose(&self) -> Option<&str> {
        self.purpose.as_deref()
    }

    /// Sets the purpose descriptor for this address.
    ///
    /// # Arguments
    /// * `purpose` - A string describing the intended use of this address
    ///
    /// # Examples
    /// ```
    /// # use zewif::{Address, ProtocolAddress, TransparentAddress};
    /// #
    /// let mut address = Address::new(ProtocolAddress::Transparent(
    ///     TransparentAddress::new("t1example")
    /// ));
    ///
    /// address.set_purpose("Donations".to_string());
    /// ```
    pub fn set_purpose(&mut self, purpose: String) {
        self.purpose = Some(purpose);
    }

    /// Returns the address as a string in its canonical format.
    ///
    /// # Returns
    /// A string representation of the address.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{Address, ProtocolAddress, TransparentAddress};
    /// #
    /// let address = Address::new(ProtocolAddress::Transparent(
    ///     TransparentAddress::new("t1exampleaddress")
    /// ));
    ///
    /// let addr_string = address.as_string();
    /// assert_eq!(addr_string, "t1exampleaddress");
    /// ```
    pub fn as_string(&self) -> String {
        self.address.as_string()
    }

    /// Returns a reference to the protocol-specific address.
    ///
    /// # Returns
    /// A reference to the `ProtocolAddress` contained within this address.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{Address, ProtocolAddress, TransparentAddress};
    /// #
    /// let t_addr = TransparentAddress::new("t1example");
    /// let protocol_addr = ProtocolAddress::Transparent(t_addr);
    /// let address = Address::new(protocol_addr);
    ///
    /// let protocol = address.address();
    /// assert!(matches!(protocol, ProtocolAddress::Transparent(_)));
    /// ```
    pub fn address(&self) -> &ProtocolAddress {
        &self.address
    }

    /// Returns a mutable reference to the protocol-specific address.
    ///
    /// # Returns
    /// A mutable reference to the `ProtocolAddress` contained within this address.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{Address, ProtocolAddress, TransparentAddress, ShieldedAddress};
    /// #
    /// let mut address = Address::new(ProtocolAddress::Transparent(
    ///     TransparentAddress::new("t1example")
    /// ));
    ///
    /// // Change the address type to shielded
    /// if let ProtocolAddress::Transparent(_) = address.address() {
    ///     *address.address_mut() = ProtocolAddress::Shielded(
    ///         ShieldedAddress::new("zs1example".to_string())
    ///     );
    /// }
    ///
    /// assert!(matches!(address.address(), ProtocolAddress::Shielded(_)));
    /// ```
    pub fn address_mut(&mut self) -> &mut ProtocolAddress {
        &mut self.address
    }

    /// Sets the name for this address.
    ///
    /// # Arguments
    /// * `name` - The user-defined name or label to assign to this address
    ///
    /// # Examples
    /// ```
    /// # use zewif::{Address, ProtocolAddress, TransparentAddress};
    /// #
    /// let mut address = Address::new(ProtocolAddress::Transparent(
    ///     TransparentAddress::new("t1example")
    /// ));
    ///
    /// address.set_name("Cold Storage".to_string());
    /// assert_eq!(address.name(), "Cold Storage");
    /// ```
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Replaces the protocol-specific address.
    ///
    /// # Arguments
    /// * `address` - The new protocol address to store
    ///
    /// # Examples
    /// ```
    /// # use zewif::{Address, ProtocolAddress, TransparentAddress};
    /// #
    /// let mut address = Address::new(ProtocolAddress::Transparent(
    ///     TransparentAddress::new("t1old")
    /// ));
    ///
    /// // Replace with a new address
    /// let new_addr = TransparentAddress::new("t1new");
    /// address.set_address(ProtocolAddress::Transparent(new_addr));
    ///
    /// assert_eq!(address.as_string(), "t1new");
    /// ```
    pub fn set_address(&mut self, address: ProtocolAddress) {
        self.address = address;
    }

    /// Returns true if this is a shielded address.
    ///
    /// # Returns
    /// `true` if the address is a shielded address (z-address), `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{Address, ProtocolAddress, ShieldedAddress, TransparentAddress};
    /// #
    /// // Create a shielded address
    /// let s_addr = ShieldedAddress::new("zs1example".to_string());
    /// let address = Address::new(ProtocolAddress::Shielded(s_addr));
    /// assert!(address.is_shielded());
    ///
    /// // Create a transparent address
    /// let t_addr = TransparentAddress::new("t1example");
    /// let address = Address::new(ProtocolAddress::Transparent(t_addr));
    /// assert!(!address.is_shielded());
    /// ```
    pub fn is_shielded(&self) -> bool {
        matches!(self.address, ProtocolAddress::Shielded(_))
    }

    /// Returns true if this is a transparent address.
    ///
    /// # Returns
    /// `true` if the address is a transparent address (t-address), `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{Address, ProtocolAddress, ShieldedAddress, TransparentAddress};
    /// #
    /// // Create a transparent address
    /// let t_addr = TransparentAddress::new("t1example");
    /// let address = Address::new(ProtocolAddress::Transparent(t_addr));
    /// assert!(address.is_transparent());
    ///
    /// // Create a shielded address
    /// let s_addr = ShieldedAddress::new("zs1example".to_string());
    /// let address = Address::new(ProtocolAddress::Shielded(s_addr));
    /// assert!(!address.is_transparent());
    /// ```
    pub fn is_transparent(&self) -> bool {
        matches!(self.address, ProtocolAddress::Transparent(_))
    }

    /// Returns true if this is a unified address.
    ///
    /// # Returns
    /// `true` if the address is a unified address (u-address), `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{Address, ProtocolAddress, UnifiedAddress, TransparentAddress};
    /// #
    /// // Create a unified address
    /// let u_addr = UnifiedAddress::new("u1example".to_string());
    /// let address = Address::new(ProtocolAddress::Unified(Box::new(u_addr)));
    /// assert!(address.is_unified());
    ///
    /// // Create a transparent address
    /// let t_addr = TransparentAddress::new("t1example");
    /// let address = Address::new(ProtocolAddress::Transparent(t_addr));
    /// assert!(!address.is_unified());
    /// ```
    pub fn is_unified(&self) -> bool {
        matches!(self.address, ProtocolAddress::Unified(_))
    }

    /// Returns the shielded address component, if available.
    ///
    /// This method returns the shielded address in one of two cases:
    /// 1. When this is directly a shielded address
    /// 2. When this is a unified address with a sapling component
    ///
    /// # Returns
    /// `Some(&ShieldedAddress)` if a shielded component is present, `None` otherwise.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{Address, ProtocolAddress, ShieldedAddress, UnifiedAddress};
    /// #
    /// // Direct shielded address
    /// let s_addr = ShieldedAddress::new("zs1example".to_string());
    /// let address = Address::new(ProtocolAddress::Shielded(s_addr));
    /// assert!(address.as_shielded().is_some());
    ///
    /// // A unified address with sapling component
    /// let mut u_addr = UnifiedAddress::new("u1example".to_string());
    /// let s_component = ShieldedAddress::new("zs1sapling".to_string());
    /// u_addr.set_sapling_component(s_component);
    ///
    /// let address = Address::new(ProtocolAddress::Unified(Box::new(u_addr)));
    /// assert!(address.as_shielded().is_some());
    /// ```
    pub fn as_shielded(&self) -> Option<&ShieldedAddress> {
        match &self.address {
            ProtocolAddress::Shielded(addr) => Some(addr),
            ProtocolAddress::Unified(addr) => addr.sapling_component(),
            _ => None,
        }
    }

    /// Returns the transparent address component, if available.
    ///
    /// This method returns the transparent address in one of two cases:
    /// 1. When this is directly a transparent address
    /// 2. When this is a unified address with a transparent component
    ///
    /// # Returns
    /// `Some(&TransparentAddress)` if a transparent component is present, `None` otherwise.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{Address, ProtocolAddress, TransparentAddress, UnifiedAddress};
    /// #
    /// // Direct transparent address
    /// let t_addr = TransparentAddress::new("t1example");
    /// let address = Address::new(ProtocolAddress::Transparent(t_addr));
    /// assert!(address.as_transparent().is_some());
    ///
    /// // A unified address with transparent component
    /// let mut u_addr = UnifiedAddress::new("u1example".to_string());
    /// let t_component = TransparentAddress::new("t1transparent");
    /// u_addr.set_transparent_component(t_component);
    ///
    /// let address = Address::new(ProtocolAddress::Unified(Box::new(u_addr)));
    /// assert!(address.as_transparent().is_some());
    /// ```
    pub fn as_transparent(&self) -> Option<&TransparentAddress> {
        match &self.address {
            ProtocolAddress::Transparent(addr) => Some(addr),
            ProtocolAddress::Unified(addr) => addr.transparent_component(),
            _ => None,
        }
    }

    /// Returns the unified address, if this is a unified address.
    ///
    /// # Returns
    /// `Some(&UnifiedAddress)` if this is a unified address, `None` otherwise.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{Address, ProtocolAddress, UnifiedAddress, TransparentAddress};
    /// #
    /// // Create a unified address
    /// let u_addr = UnifiedAddress::new("u1example".to_string());
    /// let address = Address::new(ProtocolAddress::Unified(Box::new(u_addr)));
    /// assert!(address.as_unified().is_some());
    ///
    /// // Create a non-unified address
    /// let t_addr = TransparentAddress::new("t1example");
    /// let address = Address::new(ProtocolAddress::Transparent(t_addr));
    /// assert!(address.as_unified().is_none());
    /// ```
    pub fn as_unified(&self) -> Option<&UnifiedAddress> {
        match &self.address {
            ProtocolAddress::Unified(addr) => Some(addr),
            _ => None,
        }
    }
}

impl From<Address> for Envelope {
    fn from(value: Address) -> Self {
        let envelope = Envelope::new(value.index)
            .add_type("Address")
            .add_assertion("address", value.address)
            .add_assertion("name", value.name)
            .add_optional_assertion("purpose", value.purpose);
        value.attachments.add_to_envelope(envelope)
    }
}

impl TryFrom<Envelope> for Address {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self, Self::Error> {
        envelope.check_type_envelope("Address").context("Address")?;
        let index = envelope.extract_subject().context("index")?;
        let address = envelope.try_object_for_predicate("address").context("address")?;
        let name = envelope.try_object_for_predicate("name").context("name")?;
        let purpose = envelope.try_optional_object_for_predicate("purpose").context("purpose")?;
        let attachments = Attachments::try_from_envelope(&envelope).context("attachments")?;
        Ok(Address { index, address, name, purpose, attachments })
    }
}

#[cfg(test)]
impl crate::RandomInstance for Address {
    fn random() -> Self {
        Self {
            index: 0,
            name: String::random(),
            purpose: String::opt_random(),
            address: ProtocolAddress::random(),
            attachments: Attachments::random(),
        }
    }
}

test_envelope_roundtrip!(Address);
