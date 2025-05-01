use crate::{blob, blob_envelope};

blob!(
    SeedFingerprint,
    32,
    "The fingerprint of an HD seed, as defined in ZIP 32"
);
impl Copy for SeedFingerprint {}

blob_envelope!(SeedFingerprint);

