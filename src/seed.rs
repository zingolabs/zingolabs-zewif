use crate::{blob, blob_envelope};

blob!(Seed, 32, "A pre-BIP-39 seed, 32 bytes long");

blob_envelope!(Seed);

