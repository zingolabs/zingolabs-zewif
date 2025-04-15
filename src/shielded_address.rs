use super::Data;
use super::{SpendingKey, sapling::SaplingIncomingViewingKey};
use crate::{NoQuotesDebugOption, test_envelope_roundtrip};
use anyhow::Context;
use bc_envelope::prelude::*;

/// A privacy-enhancing Zcash address that shields transaction details on the blockchain.
///
/// `ShieldedAddress` represents addresses in Zcash's privacy-focused protocols (Sapling or Orchard)
/// that encrypt transaction details, including sender, receiver, and amounts. These addresses
/// provide significantly stronger privacy guarantees than transparent addresses.
///
/// # Zcash Concept Relation
/// In Zcash, shielded addresses come in multiple protocol variants:
///
/// - **Sapling addresses**: Start with "zs" prefix, introduced in the Sapling network upgrade
/// - **Orchard addresses**: Start with "zo" prefix, introduced in the NU5 network upgrade
///
/// Shielded addresses rely on zero-knowledge proofs to validate transactions without
/// revealing transaction details publicly. Each address can have associated keys:
///
/// - **Spending Keys**: Allow full control (viewing and spending)
/// - **Viewing Keys**: Allow monitoring transactions without spending capability
/// - **Diversifiers**: Enable generating multiple unique addresses from the same key material
///
/// # Data Preservation
/// During wallet migration, the following components are preserved:
///
/// - **Address strings**: The canonical string representation (e.g., "zs1...")
/// - **Incoming Viewing Keys (IVKs)**: Preserved for transaction monitoring capability
/// - **Spending Keys**: Preserved for spending capability when available
/// - **Diversifiers**: Preserved to maintain address derivation capability
/// - **HD paths**: Preserved to maintain hierarchical wallet structure
///
/// Note: Full Viewing Keys (FVKs) are not stored separately because they can be derived
/// from spending keys when needed, and source wallets typically don't store them separately.
///
/// # Examples
/// ```
/// # use zewif::{ShieldedAddress, SpendingKey, sapling::SaplingIncomingViewingKey, Blob, Data};
/// // Create a new Sapling shielded address
/// let mut address = ShieldedAddress::new("zs1exampleaddress".to_string());
///
/// // Associate an incoming viewing key (for monitoring transactions)
/// let ivk_data = [0u8; 32]; // In practice, this would be actual key material
/// let ivk = SaplingIncomingViewingKey::new(ivk_data);
/// address.set_incoming_viewing_key(ivk);
///
/// // For addresses with spending capability, add a spending key
/// let raw_key_data = Blob::<32>::default();
/// let spending_key = SpendingKey::new_raw(raw_key_data);
/// address.set_spending_key(spending_key);
///
/// // Set the diversifier if available
/// let diversifier_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
/// let diversifier = Data::from(diversifier_data);
/// address.set_diversifier(diversifier);
///
/// // Set HD derivation path information
/// address.set_hd_derivation_path("m/32'/1'/0'/0/5".to_string());
/// ```
#[derive(Clone, PartialEq)]
pub struct ShieldedAddress {
    /// The actual address string (could encode Sapling, Orchard, etc.).
    /// This is used as a unique identifier within the wallet.
    address: String, // Unique

    /// Optional Incoming Viewing Key (IVK) for this address.
    ///
    /// When present, this 32-byte key allows the wallet to detect and view incoming transactions
    /// to this address without granting spending capability. This is particularly important for
    /// "watch-only" wallet functionality where spending keys aren't available.
    incoming_viewing_key: Option<SaplingIncomingViewingKey>,

    /// Optional spending key for this address.
    ///
    /// When present, this key allows spending funds sent to this address. During migration,
    /// spending keys are preserved exactly as they exist in the source wallet.
    spending_key: Option<SpendingKey>,

    /// Optional diversifier or other Zcash-specific metadata.
    ///
    /// The diversifier is used in creating multiple distinct addresses from a single viewing key.
    /// It allows wallets to generate multiple unique shielded addresses that all share the same
    /// spending authority.
    diversifier: Option<Data>,

    /// HD derivation path if this address was derived using HD wallet techniques.
    ///
    /// This stores the path used to derive this address in a hierarchical deterministic wallet.
    /// Preserving this information allows wallets to reconstruct their address hierarchy.
    hd_derivation_path: Option<String>,
}

impl std::fmt::Debug for ShieldedAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShieldedAddress")
            .field("address", &self.address)
            .field(
                "incoming_viewing_key",
                &NoQuotesDebugOption(&self.incoming_viewing_key),
            )
            .field("spending_key", &self.spending_key)
            .field("diversifier", &self.diversifier)
            .field("hd_derivation_path", &self.hd_derivation_path)
            .finish()
    }
}

impl ShieldedAddress {
    pub fn new(address: String) -> Self {
        ShieldedAddress {
            address,
            incoming_viewing_key: None,
            spending_key: None,
            diversifier: None,
            hd_derivation_path: None,
        }
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn set_address(&mut self, address: String) {
        self.address = address;
    }

    /// Returns the Incoming Viewing Key (IVK) associated with this address, if available.
    ///
    /// The IVK enables viewing incoming transactions without granting spending capability.
    /// During wallet migration, IVKs are preserved exactly as they exist in the source wallet.
    ///
    /// # Returns
    /// - `Some(&SaplingIncomingViewingKey)` if an IVK is associated with this address
    /// - `None` if no IVK is available (common for addresses without viewing capability)
    pub fn incoming_viewing_key(&self) -> Option<&SaplingIncomingViewingKey> {
        self.incoming_viewing_key.as_ref()
    }

    /// Associates an Incoming Viewing Key (IVK) with this shielded address.
    ///
    /// This method is primarily used during wallet migration to preserve viewing capability
    /// for existing addresses. An IVK enables the wallet to scan the blockchain for incoming
    /// transactions without revealing transaction details publicly.
    ///
    /// # Parameters
    /// - `ivk`: The 32-byte Sapling Incoming Viewing Key to associate with this address
    pub fn set_incoming_viewing_key(&mut self, ivk: SaplingIncomingViewingKey) {
        self.incoming_viewing_key = Some(ivk);
    }

    pub fn spending_key(&self) -> Option<&SpendingKey> {
        self.spending_key.as_ref()
    }

    pub fn set_spending_key(&mut self, key: SpendingKey) {
        self.spending_key = Some(key);
    }

    pub fn diversifier(&self) -> Option<&Data> {
        self.diversifier.as_ref()
    }

    pub fn set_diversifier(&mut self, diversifier: Data) {
        self.diversifier = Some(diversifier);
    }

    /// Get the HD derivation path for this address, if available
    pub fn hd_derivation_path(&self) -> Option<&str> {
        self.hd_derivation_path.as_deref()
    }

    /// Set the HD derivation path for this address
    pub fn set_hd_derivation_path(&mut self, path: String) {
        self.hd_derivation_path = Some(path);
    }
}

impl From<ShieldedAddress> for Envelope {
    fn from(value: ShieldedAddress) -> Self {
        Envelope::new(value.address)
            .add_type("ShieldedAddress")
            .add_optional_assertion("incoming_viewing_key", value.incoming_viewing_key)
            .add_optional_assertion("spending_key", value.spending_key)
            .add_optional_assertion("diversifier", value.diversifier)
            .add_optional_assertion("hd_derivation_path", value.hd_derivation_path)
    }
}

impl TryFrom<Envelope> for ShieldedAddress {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self, Self::Error> {
        envelope.check_type_envelope("ShieldedAddress").context("ShieldedAddress")?;
        let address = envelope.extract_subject().context("address")?;
        let incoming_viewing_key = envelope.try_optional_object_for_predicate("incoming_viewing_key").context("incoming_viewing_key")?;
        let spending_key = envelope.try_optional_object_for_predicate("spending_key").context("spending_key")?;
        let diversifier = envelope.try_optional_object_for_predicate("diversifier").context("diversifier")?;
        let hd_derivation_path = envelope.try_optional_object_for_predicate("hd_derivation_path").context("hd_derivation_path")?;
        Ok(ShieldedAddress {
            address,
            incoming_viewing_key,
            spending_key,
            diversifier,
            hd_derivation_path,
        })
    }
}

#[cfg(test)]
impl crate::RandomInstance for ShieldedAddress {
    fn random() -> Self {
        Self {
            address: String::random(),
            incoming_viewing_key: SaplingIncomingViewingKey::opt_random(),
            spending_key: SpendingKey::opt_random(),
            diversifier: Data::opt_random(),
            hd_derivation_path: String::opt_random(),
        }
    }
}

test_envelope_roundtrip!(ShieldedAddress);
