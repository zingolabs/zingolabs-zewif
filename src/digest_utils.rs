use sha2::{Digest, Sha256};

use super::u256;

/// SHA256 hash.
pub fn sha256(data: impl AsRef<[u8]>) -> u256 {
    let mut hasher = Sha256::new();
    hasher.update(data);
    u256::try_from(hasher.finalize().as_slice()).unwrap()
}

/// Bitcoin double SHA256 hash.
pub fn hash256(data: impl AsRef<[u8]>) -> u256 {
    sha256(sha256(data))
}
