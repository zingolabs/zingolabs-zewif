use bc_envelope::prelude::CBOR;

use crate::{Blob, blob, blob_envelope, test_envelope_roundtrip};

// A hierarchical deterministic (HD) Sapling spending key with derivation information.
//
// `SaplingExtendedFullViewingKey` extends the core spending key functionality by adding the
// necessary components for hierarchical deterministic (HD) key derivation according to [ZIP 32].
// This enables the creation of structured wallet hierarchies with parent-child key relationships.
//
// This key is encoded as defined in https://zips.z.cash/zip-0032#sapling-extended-full-viewing-keys
//
// [ZIP 32]: https://zips.z.cash/zip-0032
blob!(
    SaplingExtendedFullViewingKey,
    73,
    "A Sapling Extended Full Viewing Key, encoded as specified in ZIP 32"
);

blob_envelope!(SaplingExtendedFullViewingKey);

impl From<SaplingExtendedFullViewingKey> for CBOR {
    fn from(value: SaplingExtendedFullViewingKey) -> Self {
        CBOR::to_byte_string(value.0)
    }
}

impl From<&SaplingExtendedFullViewingKey> for CBOR {
    fn from(value: &SaplingExtendedFullViewingKey) -> Self {
        CBOR::to_byte_string(value.0.clone())
    }
}

impl TryFrom<CBOR> for SaplingExtendedFullViewingKey {
    type Error = anyhow::Error;

    fn try_from(cbor: CBOR) -> Result<Self, Self::Error> {
        let bytes = cbor.try_into_byte_string()?;
        Blob::<73>::from_slice(&bytes[..]).map(SaplingExtendedFullViewingKey)
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_cbor_roundtrip, test_envelope_roundtrip};

    use super::SaplingExtendedFullViewingKey;

    test_cbor_roundtrip!(SaplingExtendedFullViewingKey);
    test_envelope_roundtrip!(SaplingExtendedFullViewingKey);
}
