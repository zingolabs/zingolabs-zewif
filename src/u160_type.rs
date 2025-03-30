use anyhow::{Context, Result};

use crate::parse;
use super::parser::prelude::*;
use super::Blob20;

/// A 160-bit unsigned integer represented as a 20-byte array.
///
/// This type is used in Zcash primarily for transparent addresses (P2PKH, P2SH),
/// which follow Bitcoin's addressing scheme based on 160-bit hashes.
///
/// # Zcash Concept Relation
/// In Zcash's transparent addressing:
/// - P2PKH (Pay to Public Key Hash) addresses contain a 20-byte RIPEMD-160 hash of a public key
/// - P2SH (Pay to Script Hash) addresses contain a 20-byte RIPEMD-160 hash of a script
///
/// These 160-bit values provide a balance of security and space efficiency for
/// transparent addresses, matching Bitcoin's addressing scheme.
///
/// # Data Preservation
/// The `u160` type preserves the exact 20-byte representation of transparent address
/// hashes during wallet migrations, maintaining compatibility with the Bitcoin-derived
/// portions of the Zcash protocol.
///
/// # Examples
/// ```
/// use zewif::{u160, Blob20};
/// use anyhow::Result;
///
/// # fn example() -> Result<()> {
/// // Create a u160 from a byte slice (e.g., for a P2PKH address hash)
/// let address_bytes = [
///     0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa,
///     0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00, 0x11, 0x22, 0x33, 0x44
/// ];
/// let address_hash = u160::from_slice(&address_bytes)?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[allow(non_camel_case_types)]
pub struct u160([u8; 20]);

impl u160 {
    /// Creates a new `u160` value from a 20-byte `Blob20`.
    ///
    /// This method provides a convenient way to create a `u160` from a `Blob20`
    /// without error checking, since `Blob20` already guarantees the correct size.
    ///
    /// # Examples
    /// ```
    /// use zewif::{u160, Blob20};
    ///
    /// // Create a u160 from a Blob20
    /// let data = [0u8; 20];
    /// let blob = Blob20::new(data);
    /// let value = u160::from_blob(blob);
    /// ```
    pub fn from_blob(blob: Blob20) -> Self {
        Self(blob.into())
    }

    /// Creates a new `u160` value from a byte slice.
    ///
    /// This method validates that the slice is exactly 20 bytes long,
    /// which is required for a 160-bit value.
    ///
    /// # Examples
    /// ```
    /// use zewif::u160;
    /// use anyhow::Result;
    ///
    /// # fn example() -> Result<()> {
    /// // Valid slice (exactly 20 bytes)
    /// let valid_bytes = [0u8; 20];
    /// let value = u160::from_slice(&valid_bytes)?;
    ///
    /// // This would fail: incorrect length
    /// let invalid_bytes = [0u8; 19];
    /// let result = u160::from_slice(&invalid_bytes);
    /// assert!(result.is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Returns an error if the byte slice is not exactly 20 bytes long.
    pub fn from_slice(bytes: &[u8]) -> Result<Self> {
        let blob = Blob20::from_slice(bytes).context("Creating U160 from slice")?;
        Ok(Self(blob.into()))
    }
}

impl AsRef<[u8]> for u160 {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<[u8; 20]> for u160 {
    fn as_ref(&self) -> &[u8; 20] {
        &self.0
    }
}

impl std::fmt::Debug for u160 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut bytes = self.0;
        bytes.reverse();
        write!(f, "u160({})", hex::encode(bytes))
    }
}

impl std::fmt::Display for u160 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut bytes = self.0;
        bytes.reverse();
        write!(f, "{}", hex::encode(bytes))
    }
}

/// Implementation of the `Parse` trait to enable binary parsing.
///
/// This allows `u160` to be directly parsed from a binary stream using the
/// `parse!` macro, which is used when reading transparent address data in
/// Zcash transactions.
///
/// # Examples
/// ```no_run
/// # use zewif::u160;
/// # use zewif::parser::Parser;
/// # use zewif::parse;
/// # use anyhow::Result;
/// #
/// # fn example(parser: &mut Parser) -> Result<()> {
/// // Parse a 20-byte address hash from a binary stream
/// let address_hash = parse!(parser, u160, "P2PKH address hash")?;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
/// Returns an error if the parser does not have 20 bytes remaining.
impl Parse for u160 {
    fn parse(p: &mut Parser) -> Result<Self> {
        let blob = parse!(p, Blob20, "u160")?;
        Ok(Self(blob.into()))
    }
}
