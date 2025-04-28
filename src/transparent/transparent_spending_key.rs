use crate::{blob, blob_envelope};

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
