use super::{sapling::ShieldedAddress, TransparentAddress, UnifiedAddress};
use crate::test_envelope_roundtrip;
use bc_envelope::prelude::*;

/// A protocol-specific Zcash address representation without additional metadata.
///
/// `ProtocolAddress` is an enum that distinguishes between the different address
/// protocols supported in Zcash. It provides a type-safe way to handle the varied
/// address formats while enabling common operations across all address types.
///
/// # Zcash Concept Relation
/// Zcash has evolved through multiple address formats, each with different privacy
/// and functionality characteristics:
///
/// - **Transparent addresses (t-prefixed)**: Function like Bitcoin addresses,
///   exposing all transaction details on the blockchain.
///
/// - **Shielded addresses (z-prefixed)**: Use zero-knowledge proofs to encrypt
///   transaction details. Originally included Sprout (legacy), now primarily
///   Sapling (zs-prefixed) and Orchard (zo-prefixed) protocols.
///
/// - **Unified addresses (u-prefixed)**: Introduced in NU5, these bundle multiple
///   receiver types into a single address, allowing the sender's wallet to automatically
///   choose the most private protocol supported by both parties.
///
/// # Data Preservation
/// During wallet migration, the complete address details from each protocol are preserved:
///
/// - **For transparent addresses**: Base58Check-encoded address strings and associated keys
/// - **For shielded addresses**: Encoded address strings, diversifiers, and viewing keys
/// - **For unified addresses**: All component addresses, diversifier indices, and metadata
///
/// # Examples
/// ```
/// # use zewif::{ProtocolAddress, TransparentAddress, sapling::ShieldedAddress, UnifiedAddress};
/// // Create a transparent address
/// let t_addr = TransparentAddress::new("t1example");
/// let t_protocol = ProtocolAddress::Transparent(t_addr);
/// assert!(t_protocol.as_transparent().is_some());
///
/// // Create a shielded address
/// let s_addr = ShieldedAddress::new("zs1example".to_string());
/// let s_protocol = ProtocolAddress::Shielded(s_addr);
/// assert!(s_protocol.as_shielded().is_some());
///
/// // Create a unified address
/// let u_addr = UnifiedAddress::new("u1example".to_string());
/// let u_protocol = ProtocolAddress::Unified(Box::new(u_addr));
/// assert!(u_protocol.as_unified().is_some());
///
/// // All protocol addresses can be converted to strings
/// assert!(t_protocol.as_string().starts_with("t1"));
/// assert!(s_protocol.as_string().starts_with("zs1"));
/// assert!(u_protocol.as_string().starts_with("u1"));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum ProtocolAddress {
    /// An exposed transparent (T-address) similar to Bitcoin's.
    Transparent(TransparentAddress),

    /// A shielded address (Z-address). This can include Sapling, Sprout, or Orchard formats.
    Shielded(ShieldedAddress),

    /// A unified address (U-address) that contains multiple receiver types.
    /// Uses Box to reduce the total size of the enum since UnifiedAddress is larger.
    Unified(Box<UnifiedAddress>),
}

impl ProtocolAddress {
    /// Returns the address as a string in its canonical format.
    ///
    /// This method returns the string representation of the address, regardless
    /// of which protocol it uses. This is useful for display or storage where
    /// only the address string is needed.
    ///
    /// # Returns
    /// A string representation of the address.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{ProtocolAddress, TransparentAddress, sapling::ShieldedAddress, UnifiedAddress};
    /// #
    /// // Transparent address
    /// let t_addr = TransparentAddress::new("t1example");
    /// let protocol = ProtocolAddress::Transparent(t_addr);
    /// assert_eq!(protocol.as_string(), "t1example");
    ///
    /// // Shielded address
    /// let s_addr = ShieldedAddress::new("zs1example".to_string());
    /// let protocol = ProtocolAddress::Shielded(s_addr);
    /// assert_eq!(protocol.as_string(), "zs1example");
    /// ```
    pub fn as_string(&self) -> String {
        match self {
            ProtocolAddress::Transparent(addr) => addr.address().to_string(),
            ProtocolAddress::Shielded(addr) => addr.address().to_string(),
            ProtocolAddress::Unified(addr) => addr.address().to_string(),
        }
    }

    /// Returns the underlying shielded address, if available.
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
    /// # use zewif::{ProtocolAddress, sapling::ShieldedAddress, UnifiedAddress};
    /// #
    /// // Direct shielded address
    /// let s_addr = ShieldedAddress::new("zs1example".to_string());
    /// let protocol = ProtocolAddress::Shielded(s_addr);
    /// assert!(protocol.as_shielded().is_some());
    ///
    /// // Unified address with sapling component
    /// let mut u_addr = UnifiedAddress::new("u1example".to_string());
    /// let s_component = ShieldedAddress::new("zs1sapling".to_string());
    /// u_addr.set_sapling_component(s_component);
    ///
    /// let protocol = ProtocolAddress::Unified(Box::new(u_addr));
    /// assert!(protocol.as_shielded().is_some());
    /// ```
    pub fn as_shielded(&self) -> Option<&ShieldedAddress> {
        match self {
            ProtocolAddress::Shielded(addr) => Some(addr),
            ProtocolAddress::Unified(addr) => addr.sapling_component(),
            _ => None,
        }
    }

    /// Returns the underlying transparent address, if available.
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
    /// # use zewif::{ProtocolAddress, TransparentAddress, UnifiedAddress};
    /// #
    /// // Direct transparent address
    /// let t_addr = TransparentAddress::new("t1example");
    /// let protocol = ProtocolAddress::Transparent(t_addr);
    /// assert!(protocol.as_transparent().is_some());
    ///
    /// // Unified address with transparent component
    /// let mut u_addr = UnifiedAddress::new("u1example".to_string());
    /// let t_component = TransparentAddress::new("t1transparent");
    /// u_addr.set_transparent_component(t_component);
    ///
    /// let protocol = ProtocolAddress::Unified(Box::new(u_addr));
    /// assert!(protocol.as_transparent().is_some());
    /// ```
    pub fn as_transparent(&self) -> Option<&TransparentAddress> {
        match self {
            ProtocolAddress::Transparent(addr) => Some(addr),
            ProtocolAddress::Unified(addr) => addr.transparent_component(),
            _ => None,
        }
    }

    /// Returns the underlying unified address, if this is a unified address.
    ///
    /// # Returns
    /// `Some(&UnifiedAddress)` if this is a unified address, `None` otherwise.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{ProtocolAddress, UnifiedAddress, TransparentAddress};
    /// #
    /// // Unified address
    /// let u_addr = UnifiedAddress::new("u1example".to_string());
    /// let protocol = ProtocolAddress::Unified(Box::new(u_addr));
    /// assert!(protocol.as_unified().is_some());
    ///
    /// // Non-unified address
    /// let t_addr = TransparentAddress::new("t1example");
    /// let protocol = ProtocolAddress::Transparent(t_addr);
    /// assert!(protocol.as_unified().is_none());
    /// ```
    pub fn as_unified(&self) -> Option<&UnifiedAddress> {
        match self {
            ProtocolAddress::Unified(addr) => Some(&**addr),
            _ => None,
        }
    }
}

impl From<ProtocolAddress> for Envelope {
    fn from(value: ProtocolAddress) -> Self {
        match value {
            ProtocolAddress::Transparent(addr) => addr.into(),
            ProtocolAddress::Shielded(addr) => addr.into(),
            ProtocolAddress::Unified(addr) => (*addr).into(),
        }
    }
}

impl TryFrom<Envelope> for ProtocolAddress {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self, Self::Error> {
        if envelope.has_type_envelope("TransparentAddress") {
            Ok(ProtocolAddress::Transparent(envelope.try_into()?))
        } else if envelope.has_type_envelope("ShieldedAddress") {
            Ok(ProtocolAddress::Shielded(envelope.try_into()?))
        } else if envelope.has_type_envelope("UnifiedAddress") {
            Ok(ProtocolAddress::Unified(Box::new(envelope.try_into()?)))
        } else {
            Err(anyhow::anyhow!("Invalid ProtocolAddress type"))
        }
    }
}

#[cfg(test)]
impl crate::RandomInstance for ProtocolAddress {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let choice = rand::Rng::gen_range(&mut rng, 0..3);
        match choice {
            0 => ProtocolAddress::Transparent(TransparentAddress::random()),
            1 => ProtocolAddress::Shielded(ShieldedAddress::random()),
            _ => ProtocolAddress::Unified(Box::new(UnifiedAddress::random())),
        }
    }
}

test_envelope_roundtrip!(ProtocolAddress);
