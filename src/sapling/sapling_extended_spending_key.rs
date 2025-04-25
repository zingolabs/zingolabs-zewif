use bc_envelope::prelude::CBOR;

use crate::{Blob, blob, blob_envelope, test_envelope_roundtrip};

// A hierarchical deterministic (HD) Sapling spending key with derivation information.
//
// `SaplingExtendedSpendingKey` extends the core spending key functionality by adding the necessary
// components for hierarchical deterministic (HD) key derivation according to [ZIP 32]. This
// enables the creation of structured wallet hierarchies with parent-child key relationships.
//
// This key is encoded as defined in https://zips.z.cash/zip-0032#sapling-extended-spending-keys
//
// [ZIP 32]: https://zips.z.cash/zip-0032
blob!(
    SaplingExtendedSpendingKey,
    169,
    "A Sapling Extended Spending Key, encoded as specified in ZIP 32"
);

blob_envelope!(SaplingExtendedSpendingKey);

impl From<SaplingExtendedSpendingKey> for CBOR {
    fn from(value: SaplingExtendedSpendingKey) -> Self {
        CBOR::to_byte_string(value.0)
    }
}

impl From<&SaplingExtendedSpendingKey> for CBOR {
    fn from(value: &SaplingExtendedSpendingKey) -> Self {
        CBOR::to_byte_string(value.0.clone())
    }
}

impl TryFrom<CBOR> for SaplingExtendedSpendingKey {
    type Error = anyhow::Error;

    fn try_from(cbor: CBOR) -> Result<Self, Self::Error> {
        let bytes = cbor.try_into_byte_string()?;
        Blob::<169>::from_slice(&bytes[..]).map(SaplingExtendedSpendingKey)
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_cbor_roundtrip, test_envelope_roundtrip};

    use super::SaplingExtendedSpendingKey;

    test_cbor_roundtrip!(SaplingExtendedSpendingKey);
    test_envelope_roundtrip!(SaplingExtendedSpendingKey);
}
