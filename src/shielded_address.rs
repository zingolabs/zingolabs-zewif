use crate::NoQuotesDebugOption;

use super::Data;

use super::{sapling::SaplingIncomingViewingKey, SpendingKey};

/// Details specific to shielded addresses.
#[derive(Clone)]
pub struct ShieldedAddress {
    /// The actual address string (could encode Sapling, Orchard, etc.).
    address: String, // Unique
    incoming_viewing_key: Option<SaplingIncomingViewingKey>,
    /// Optional spending key for this address
    spending_key: Option<SpendingKey>,
    /// Optional diversifier or other Zcash-specific metadata.
    diversifier: Option<Data>,
}

impl std::fmt::Debug for ShieldedAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShieldedAddress")
            .field("address", &self.address)
            .field("incoming_viewing_key", &NoQuotesDebugOption(&self.incoming_viewing_key))
            .field("spending_key", &self.spending_key)
            .field("diversifier", &self.diversifier)
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
        }
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn set_address(&mut self, address: String) {
        self.address = address;
    }

    pub fn incoming_viewing_key(&self) -> Option<&SaplingIncomingViewingKey> {
        self.incoming_viewing_key.as_ref()
    }

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
}
