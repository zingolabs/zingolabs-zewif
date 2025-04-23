use bc_envelope::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::{test_envelope_roundtrip, Blob, Data, ReceiverType, sapling, TransparentAddress};
use anyhow::Context;

/// A multi-protocol Zcash address that can contain components from different Zcash protocols.
///
/// `UnifiedAddress` represents Zcash's next-generation addressing format that allows bundling
/// multiple receiver types (transparent, Sapling, Orchard) into a single encoded string.
/// This enables wallets to generate one address that can receive funds via any supported
/// protocol while hiding the underlying complexity from users.
///
/// # Zcash Concept Relation
/// Unified Addresses (UAs) were introduced in ZIP-316 and deployed with NU5 (Orchard
/// network upgrade). They address several limitations of individual address types:
///
/// - **Simplified User Experience**: Users see a single address (starting with "u1...")
///   rather than dealing with multiple address types
/// - **Forward Compatibility**: New receiver types can be added without breaking existing UAs
/// - **Protocol Agnostic**: Senders don't need to choose which protocol to use; their wallet
///   can select the most appropriate one based on capability
/// - **Progressive Privacy**: Transactions automatically use the best privacy technology
///   available to both sender and receiver
///
/// Each UA contains one or more "receivers" from different protocols (transparent, Sapling,
/// and/or Orchard). When funds are sent to a UA, the sender's wallet selects the most
/// private protocol that both wallets support.
///
/// # Data Preservation
/// During wallet migration, the following components are preserved:
///
/// - **UA string**: The full unified address string when available
/// - **Diversifier index**: Used to deterministically derive addresses from keys
/// - **HD path information**: For addresses derived from hierarchical deterministic wallets
///
/// # Implementation Note
/// In Zcash wallets, unified addresses are not typically stored directly in wallet.dat files.
/// Instead, wallets store the metadata needed to derive UAs at runtime:
/// - Diversifier indices
/// - Information about which receiver types to include
/// - References to the keys used to derive the addresses
///
/// This structure preserves all the metadata that exists in the source wallet,
/// even if the actual unified address string is a placeholder in some cases.
/// This approach aligns with the ZeWIF focus on data preservation rather than
/// operational wallet functionality.
///
/// # Examples
/// ```
/// # use zewif::{UnifiedAddress, TransparentAddress, sapling, ReceiverType, Blob};
/// // Create a new unified address
/// let mut ua = UnifiedAddress::new("u1exampleaddress".to_string());
///
/// // Set a diversifier index (used in address derivation)
/// let diversifier_data = [0u8; 11];
/// let diversifier_index = Blob::new(diversifier_data);
/// ua.set_diversifier_index(diversifier_index);
///
/// // Add a transparent component
/// let t_addr = TransparentAddress::new("t1transparentpart");
/// ua.set_transparent_component(t_addr);
///
/// // Add a sapling component
/// let s_addr = sapling::Address::new("zs1saplingpart".to_string());
/// ua.set_sapling_component(s_addr);
///
/// // Check which components are present
/// assert!(ua.has_transparent_component());
/// assert!(ua.has_sapling_component());
/// assert!(!ua.has_orchard_component());
///
/// // Retrieve the receiver types
/// let receiver_types = ua.receiver_types();
/// assert_eq!(receiver_types.len(), 2);
/// assert!(receiver_types.contains(&ReceiverType::P2PKH));
/// assert!(receiver_types.contains(&ReceiverType::Sapling));
/// ```
#[derive(Clone, PartialEq)]
pub struct UnifiedAddress {
    /// The full unified address string (starting with "u...")
    address: String,

    /// The diversifier index used to derive this unified address
    /// This is typically an 11-byte value used in the address derivation process
    diversifier_index: Option<Blob<11>>,

    /// The types of receivers contained in this unified address
    receiver_types: HashSet<ReceiverType>,

    /// Map of receiver types to their component addresses
    /// This allows access to the individual addresses that make up the unified address
    component_addresses: HashMap<ReceiverType, String>,

    /// Transparent component of this unified address (if present)
    transparent_component: Option<TransparentAddress>,

    /// Sapling component of this unified address (if present)
    sapling_component: Option<sapling::Address>,

    /// Orchard component raw data (if present)
    /// Since we don't have a dedicated OrchardAddress type yet,
    /// we store the raw data for future use
    orchard_component_data: Option<Data>,

    /// HD derivation path if this address was derived using HD wallet techniques
    hd_derivation_path: Option<String>,
}

impl std::fmt::Debug for UnifiedAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnifiedAddress")
            .field("address", &self.address)
            .field("diversifier_index", &self.diversifier_index)
            .field("receiver_types", &self.receiver_types)
            .field("component_addresses", &self.component_addresses)
            .field("transparent_component", &self.transparent_component)
            .field("sapling_component", &self.sapling_component)
            .field("orchard_component_data", &self.orchard_component_data)
            .field("hd_derivation_path", &self.hd_derivation_path)
            .finish()
    }
}

impl UnifiedAddress {
    /// Create a new UnifiedAddress with the given address string
    pub fn new(address: String) -> Self {
        UnifiedAddress {
            address,
            diversifier_index: None,
            receiver_types: HashSet::new(),
            component_addresses: HashMap::new(),
            transparent_component: None,
            sapling_component: None,
            orchard_component_data: None,
            hd_derivation_path: None,
        }
    }

    /// Get the full unified address string
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Set the unified address string
    pub fn set_address(&mut self, address: String) {
        self.address = address;
    }

    /// Get the diversifier index if available
    pub fn diversifier_index(&self) -> Option<&Blob<11>> {
        self.diversifier_index.as_ref()
    }

    /// Set the diversifier index
    pub fn set_diversifier_index(&mut self, diversifier_index: Blob<11>) {
        self.diversifier_index = Some(diversifier_index);
    }

    /// Get the list of receiver types contained in this unified address
    pub fn receiver_types(&self) -> &HashSet<ReceiverType> {
        &self.receiver_types
    }

    /// Add a receiver type to this unified address
    pub fn add_receiver_type(&mut self, receiver_type: ReceiverType) {
        self.receiver_types.insert(receiver_type);
    }

    /// Set the receiver types for this unified address
    pub fn set_receiver_types(&mut self, receiver_types: HashSet<ReceiverType>) {
        self.receiver_types = receiver_types;
    }

    /// Get a component address for a specific receiver type, if present
    pub fn component_address(&self, receiver_type: &ReceiverType) -> Option<&str> {
        self.component_addresses.get(receiver_type).map(|s| s.as_str())
    }

    /// Add a component address for a specific receiver type
    pub fn add_component_address(&mut self, receiver_type: ReceiverType, address: String) {
        self.component_addresses.insert(receiver_type, address);
        self.receiver_types.insert(receiver_type);
    }

    /// Get the transparent component of this unified address, if present
    pub fn transparent_component(&self) -> Option<&TransparentAddress> {
        self.transparent_component.as_ref()
    }

    /// Set the transparent component of this unified address
    pub fn set_transparent_component(&mut self, address: TransparentAddress) {
        let address_str = address.address().to_string();
        self.transparent_component = Some(address);
        self.add_receiver_type(ReceiverType::P2PKH);
        self.component_addresses.insert(ReceiverType::P2PKH, address_str);
    }

    /// Get the sapling component of this unified address, if present
    pub fn sapling_component(&self) -> Option<&sapling::Address> {
        self.sapling_component.as_ref()
    }

    /// Set the sapling component of this unified address
    pub fn set_sapling_component(&mut self, address: sapling::Address) {
        let address_str = address.address().to_string();
        self.sapling_component = Some(address);
        self.add_receiver_type(ReceiverType::Sapling);
        self.component_addresses.insert(ReceiverType::Sapling, address_str);
    }

    /// Get the orchard component data, if present
    pub fn orchard_component_data(&self) -> Option<&Data> {
        self.orchard_component_data.as_ref()
    }

    /// Set the orchard component data
    pub fn set_orchard_component_data(&mut self, data: Data) {
        self.orchard_component_data = Some(data);
        self.add_receiver_type(ReceiverType::Orchard);
        // We don't add to component_addresses since we don't have a string representation
    }

    /// Get the HD derivation path for this address, if available
    pub fn hd_derivation_path(&self) -> Option<&str> {
        self.hd_derivation_path.as_deref()
    }

    /// Set the HD derivation path for this address
    pub fn set_hd_derivation_path(&mut self, path: String) {
        self.hd_derivation_path = Some(path);
    }

    /// Returns true if this unified address has a transparent component
    pub fn has_transparent_component(&self) -> bool {
        self.transparent_component.is_some() ||
        self.receiver_types.contains(&ReceiverType::P2PKH) ||
        self.receiver_types.contains(&ReceiverType::P2SH)
    }

    /// Returns true if this unified address has a sapling component
    pub fn has_sapling_component(&self) -> bool {
        self.sapling_component.is_some() ||
        self.receiver_types.contains(&ReceiverType::Sapling)
    }

    /// Returns true if this unified address has an orchard component
    pub fn has_orchard_component(&self) -> bool {
        self.orchard_component_data.is_some() ||
        self.receiver_types.contains(&ReceiverType::Orchard)
    }
}

impl From<UnifiedAddress> for Envelope {
    fn from(value: UnifiedAddress) -> Self {
        Envelope::new(value.address)
            .add_type("UnifiedAddress")
            .add_optional_assertion("diversifier_index", value.diversifier_index)
            .add_assertion("receiver_types", value.receiver_types.sort_by_cbor_encoding()) // Deterministic ordering
            .add_assertion("component_addresses", value.component_addresses)
            .add_optional_assertion("transparent_component", value.transparent_component)
            .add_optional_assertion("sapling_component", value.sapling_component)
            .add_optional_assertion("orchard_component_data", value.orchard_component_data)
            .add_optional_assertion("hd_derivation_path", value.hd_derivation_path)
    }
}

impl TryFrom<Envelope> for UnifiedAddress {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self, Self::Error> {
        envelope.check_type_envelope("UnifiedAddress").context("UnifiedAddress")?;
        let address = envelope.extract_subject().context("address")?;
        let diversifier_index = envelope.try_optional_object_for_predicate("diversifier_index").context("diversifier_index")?;
        let receiver_types = envelope.extract_object_for_predicate("receiver_types").context("receiver_types")?;
        let component_addresses = envelope.extract_object_for_predicate("component_addresses").context("component_addresses")?;
        let transparent_component = envelope.try_optional_object_for_predicate("transparent_component").context("transparent_component")?;
        let sapling_component = envelope.try_optional_object_for_predicate("sapling_component").context("sapling_component")?;
        let orchard_component_data = envelope.try_optional_object_for_predicate("orchard_component_data").context("orchard_component_data")?;
        let hd_derivation_path = envelope.try_optional_object_for_predicate("hd_derivation_path").context("hd_derivation_path")?;

        Ok(UnifiedAddress {
            address,
            diversifier_index,
            receiver_types,
            component_addresses,
            transparent_component,
            sapling_component,
            orchard_component_data,
            hd_derivation_path,
        })
    }
}

#[cfg(test)]
impl crate::RandomInstance for UnifiedAddress {
    fn random() -> Self {
        Self {
            address: String::random(),
            diversifier_index: Blob::opt_random(),
            receiver_types: HashSet::random(),
            component_addresses: HashMap::random(),
            transparent_component: TransparentAddress::opt_random(),
            sapling_component: sapling::Address::opt_random(),
            orchard_component_data: Data::opt_random(),
            hd_derivation_path: String::opt_random(),
        }
    }
}

test_envelope_roundtrip!(UnifiedAddress);
