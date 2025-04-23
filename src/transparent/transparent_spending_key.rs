use bc_envelope::prelude::CBOR;

use crate::{Blob, blob, blob_envelope, test_envelope_roundtrip};

// A Zcash transparent spending key with derivation information.
//
// `TransparentSpendingKey` extends the core spending key functionality by adding the necessary
// components for hierarchical deterministic (HD) key derivation according to [BIP 44]. This
// enables the creation of structured wallet hierarchies with parent-child key relationships.
//
// [BIP 44]: https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki
blob!(
    TransparentSpendingKey,
    32,
    "A Zcash transparent private key"
);

blob_envelope!(TransparentSpendingKey);

impl From<TransparentSpendingKey> for CBOR {
    fn from(value: TransparentSpendingKey) -> Self {
        CBOR::to_byte_string(value.0)
    }
}

impl From<&TransparentSpendingKey> for CBOR {
    fn from(value: &TransparentSpendingKey) -> Self {
        CBOR::to_byte_string(value.0.clone())
    }
}

impl TryFrom<CBOR> for TransparentSpendingKey {
    type Error = anyhow::Error;

    fn try_from(cbor: CBOR) -> Result<Self, Self::Error> {
        let bytes = cbor.try_into_byte_string()?;
        Blob::<32>::from_slice(&bytes[..]).map(TransparentSpendingKey)
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_cbor_roundtrip, test_envelope_roundtrip};

    use super::TransparentSpendingKey;

    test_cbor_roundtrip!(TransparentSpendingKey);
    test_envelope_roundtrip!(TransparentSpendingKey);
}
