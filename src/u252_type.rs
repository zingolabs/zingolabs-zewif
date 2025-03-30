use anyhow::{Result, bail};

use crate::parse;
use super::parser::prelude::*;
use super::Blob32;

/// A 252-bit unsigned integer represented as a 32-byte array with the top 4 bits set to zero.
///
/// This type is specifically used in Zcash's Orchard protocol for note commitments
/// and other cryptographic values that require 252 bits of precision while maintaining
/// compatibility with 32-byte data structures.
///
/// # Zcash Concept Relation
/// In Zcash's Orchard protocol, certain cryptographic primitives operate on the
/// prime field with modulus 2^252 + 27742317777372353535851937790883648493. This 
/// requires values that fit within 252 bits.
///
/// This type enforces that constraint by validating that the top 4 bits are zero,
/// ensuring mathematical correctness while maintaining compatibility with 32-byte
/// storage.
///
/// # Data Preservation
/// The `u252` type preserves Orchard note commitments and other 252-bit values 
/// during wallet migrations, while enforcing the constraint that the value actually
/// fits within 252 bits.
///
/// # Examples
/// ```
/// use zewif::{u252, Blob32};
/// use anyhow::Result;
///
/// # fn example() -> Result<()> {
/// // Create a blob with the top 4 bits set to zero
/// let mut data = [0u8; 32];
/// data[0] = 0x0F; // Only using the bottom 4 bits of the first byte
/// let blob = Blob32::new(data);
///
/// // Convert to u252 (will succeed because top 4 bits are zero)
/// let value = u252::from_blob(blob)?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[allow(non_camel_case_types)]
pub struct u252([u8; 32]);

impl u252 {
    /// Creates a new `u252` value from a 32-byte `Blob32`.
    ///
    /// This method validates that the most significant 4 bits are zero,
    /// ensuring the value fits within 252 bits as required by Zcash's Orchard protocol.
    ///
    /// # Examples
    /// ```
    /// use zewif::{Blob32, u252};
    /// use anyhow::Result;
    ///
    /// # fn example() -> Result<()> {
    /// // Valid u252 (MSB has top 4 bits = 0)
    /// let valid_bytes = [0x0F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 
    ///                    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    ///                    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 
    ///                    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    /// let valid_blob = Blob32::new(valid_bytes);
    /// let value = u252::from_blob(valid_blob)?;
    ///
    /// // This would fail: top 4 bits are not zero
    /// let invalid_bytes = [0x10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ///                      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    /// let invalid_blob = Blob32::new(invalid_bytes);
    /// let result = u252::from_blob(invalid_blob);
    /// assert!(result.is_err());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Returns an error if the most significant 4 bits are not zero.
    pub fn from_blob(blob: Blob32) -> Result<Self> {
        let blob: [u8; 32] = blob.into();
        if (blob[0] & 0xf0) != 0 {
            bail!("First four bits of u252 must be zero");
        }
        Ok(Self(blob))
    }
}

impl AsRef<[u8]> for u252 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; 32]> for u252 {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl std::fmt::Debug for u252 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut bytes = self.0;
        bytes.reverse();
        write!(f, "u252({})", hex::encode(bytes))
    }
}

impl std::fmt::Display for u252 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut bytes = self.0;
        bytes.reverse();
        write!(f, "{}", hex::encode(bytes))
    }
}

/// Implementation of the `Parse` trait to enable binary parsing.
///
/// This allows `u252` to be directly parsed from a binary stream using the
/// `parse!` macro, which is commonly used when reading Zcash Orchard protocol elements.
/// The implementation validates that the parsed value fits within 252 bits.
///
/// # Examples
/// ```no_run
/// # use zewif::u252;
/// # use zewif::parser::Parser;
/// # use zewif::parse;
/// # use anyhow::Result;
/// #
/// # fn example(parser: &mut Parser) -> Result<()> {
/// // Parse a 252-bit value from a binary stream
/// let note_commitment = parse!(parser, u252, "Orchard note commitment")?;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
/// Returns an error if:
/// - The parser does not have 32 bytes remaining
/// - The parsed value has non-zero bits in the most significant 4 bits
impl Parse for u252 {
    fn parse(p: &mut Parser) -> Result<Self> {
        let blob = parse!(p, "u252")?;
        Self::from_blob(blob)
    }
}
