//! A Sapling Incoming Viewing Key (IVK), which allows detection and decryption of
//! incoming transactions to a Sapling shielded address.
//!
//! `SaplingIncomingViewingKey` is a 32-byte key that enables a wallet to detect when
//! funds have been sent to its associated Sapling address and to decrypt the incoming
//! transaction details, without granting the ability to spend those funds.
//!
//! # Zcash Concept Relation
//! In Zcash's Sapling protocol, the privacy features rely on a system of keys with
//! different capabilities:
//!
//! - **Full Viewing Keys** can detect both incoming and outgoing transactions
//! - **Incoming Viewing Keys** (derived from full viewing keys) can only detect
//!   incoming transactions
//! - **Spending Keys** provide full control, including spending capability
//!
//! IVKs enable "watch-only" wallet functionality, where users can monitor their
//! funds without risking theft if the wallet is compromised.
//!
//! # Data Preservation
//! During wallet migration, incoming viewing keys are preserved exactly as they
//! exist in the source wallet, maintaining the ability to detect and view incoming
//! transactions in the migrated wallet.
//!
//! # Examples
//! ```
//! use zewif::sapling::SaplingIncomingViewingKey;
//! use zewif::Blob;
//!
//! // Wrap the raw bytes of an encoded Sapling incoming viewing key.
//! let raw_bytes = [0u8; 32]; // In practice, this would be actual key material
//! let ivk = SaplingIncomingViewingKey::new(raw_bytes);
//!
//! // The key can be converted to a blob for storage or transmission
//! let as_blob: Blob<32> = ivk.into();
//! ```

use bc_envelope::prelude::CBOR;

use crate::{Blob, blob, blob_envelope, test_envelope_roundtrip};

blob!(
    IncomingViewingKey,
    32,
    "A Sapling Incoming Viewing Key (IVK) for detecting incoming transactions."
);

blob_envelope!(IncomingViewingKey);

impl From<IncomingViewingKey> for CBOR {
    fn from(value: IncomingViewingKey) -> Self {
        CBOR::to_byte_string(value.0)
    }
}

impl From<&IncomingViewingKey> for CBOR {
    fn from(value: &IncomingViewingKey) -> Self {
        CBOR::to_byte_string(value.0.clone())
    }
}

impl TryFrom<CBOR> for IncomingViewingKey {
    type Error = anyhow::Error;

    fn try_from(cbor: CBOR) -> Result<Self, Self::Error> {
        let bytes = cbor.try_into_byte_string()?;
        Blob::<32>::from_slice(&bytes[..]).map(IncomingViewingKey)
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_cbor_roundtrip, test_envelope_roundtrip};

    use super::IncomingViewingKey;

    test_cbor_roundtrip!(IncomingViewingKey);
    test_envelope_roundtrip!(IncomingViewingKey);
}
