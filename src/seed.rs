use crate::blob;
use crate::blob_envelope;
use crate::test_envelope_roundtrip;

blob!(Seed, 32, "A pre-BIP-39 seed, 32 bytes long");

blob_envelope!(Seed);
