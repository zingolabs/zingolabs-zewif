use super::{Blob32, parser::prelude::*};
use crate::{HexParseError, parse, test_cbor_roundtrip, test_envelope_roundtrip};
use anyhow::{Error, Result, bail};
use bc_envelope::prelude::*;

pub const U256_SIZE: usize = 32;

/// A 256-bit unsigned integer represented as a 32-byte array in little-endian byte order.
///
/// This type is used throughout ZCash data structures to represent hashes, block hashes,
/// transaction IDs, and other cryptographic values that require 256 bits of precision.
///
/// # Zcash Concept Relation
/// In Zcash, many protocol elements use 256-bit values:
/// - Block hashes
/// - Transaction IDs (txids)
/// - Nullifiers
/// - Merkle tree nodes
/// - Various cryptographic commitments
///
/// The 256-bit size provides the cryptographic strength needed for secure hash representations
/// while maintaining compatibility with common cryptographic primitives like SHA-256.
///
/// # Data Preservation
/// The `u256` type preserves the exact 32-byte representation of 256-bit values found
/// in the Zcash protocol, ensuring cryptographic integrity during wallet migrations.
///
/// # Examples
/// ```
/// # use zewif::u256;
/// // Create a u256 from a hexadecimal string (common for block hashes)
/// let block_hash = u256::from_hex("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f").unwrap();
///
/// // Display values are shown in reversed byte order (as is conventional in Bitcoin/Zcash)
/// assert_eq!(
///     format!("{}", block_hash),
///     "6fe28c0ab6f1b372c1a6a246ae63f74f931e8365e15a089c68d6190000000000"
/// );
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[allow(non_camel_case_types)]
pub struct u256([u8; U256_SIZE]);

impl u256 {
    /// Creates a new `u256` value from a hexadecimal string.
    ///
    /// This method is particularly useful for creating test values or initializing
    /// from known hash values. The input hex string is interpreted in the standard order
    /// (not reversed), so block hashes and txids should be provided in their canonical form.
    ///
    /// # Examples
    /// ```
    /// # use zewif::u256;
    /// // Bitcoin genesis block hash (note: not reversed in the input)
    /// let genesis_hash = u256::from_hex("000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f");
    /// ```
    pub fn from_hex(hex: &str) -> Result<Self, HexParseError> {
        let blob = Blob32::from_hex(hex)?;
        Ok(Self(blob.into()))
    }
}

impl TryFrom<&[u8]> for u256 {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != U256_SIZE {
            bail!("Invalid data length: expected 32, got {}", bytes.len());
        }
        let mut a = [0u8; U256_SIZE];
        a.copy_from_slice(bytes);
        Ok(Self(a))
    }
}

impl TryFrom<&[u8; U256_SIZE]> for u256 {
    type Error = Error;

    fn try_from(bytes: &[u8; U256_SIZE]) -> Result<Self, Self::Error> {
        Ok(Self(*bytes))
    }
}

impl TryFrom<&Vec<u8>> for u256 {
    type Error = Error;

    fn try_from(bytes: &Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl AsRef<[u8]> for u256 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; U256_SIZE]> for u256 {
    fn as_ref(&self) -> &[u8; U256_SIZE] {
        &self.0
    }
}

impl std::fmt::Debug for u256 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut bytes = self.0;
        bytes.reverse();
        write!(f, "u256({})", hex::encode(bytes))
    }
}

impl std::fmt::Display for u256 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut bytes = self.0;
        bytes.reverse();
        write!(f, "{}", hex::encode(bytes))
    }
}

/// Implementation of the `Parse` trait to enable binary parsing.
///
/// This allows `u256` to be directly parsed from a binary stream using the
/// `parse!` macro, which is commonly used when reading Zcash blocks and transactions.
///
/// # Examples
/// ```no_run
/// # use zewif::u256;
/// # use zewif::parser::Parser;
/// # use zewif::parse;
/// # use anyhow::Result;
/// #
/// # fn example(parser: &mut Parser) -> Result<()> {
/// // Parse a 32-byte hash value from a binary stream
/// let block_hash = parse!(parser, u256, "block hash")?;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
/// Returns an error if the parser does not have 32 bytes remaining.
impl Parse for u256 {
    fn parse(p: &mut Parser) -> Result<Self> {
        let bytes = parse!(p, "u256")?;
        Ok(Self(bytes))
    }
}

impl From<u256> for CBOR {
    fn from(value: u256) -> Self {
        CBOR::to_byte_string(value)
    }
}

impl From<&u256> for CBOR {
    fn from(value: &u256) -> Self {
        CBOR::to_byte_string(value)
    }
}

impl TryFrom<CBOR> for u256 {
    type Error = dcbor::Error;

    fn try_from(cbor: CBOR) -> dcbor::Result<Self> {
        let bytes = cbor.try_into_byte_string()?;
        Ok(Self::try_from(&bytes)?)
    }
}

impl From<u256> for Envelope {
    fn from(value: u256) -> Self {
        Envelope::new(CBOR::from(value))
    }
}

impl TryFrom<Envelope> for u256 {
    type Error = Error;

    fn try_from(envelope: Envelope) -> Result<Self> {
        envelope.extract_subject()
    }
}

#[cfg(test)]
impl crate::RandomInstance for u256 {
    fn random() -> Self {
        let mut rng = bc_rand::thread_rng();
        Self(bc_rand::rng_random_array(&mut rng))
    }
}

test_cbor_roundtrip!(u256);
test_envelope_roundtrip!(u256);
