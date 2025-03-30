use anyhow::Result;

use crate::{parse, parser::prelude::*};

/// A type-safe wrapper for 32-bit integer identifiers in the Zcash wallet ecosystem.
///
/// `IntID` encapsulates a 32-bit unsigned integer value that can be used as an identifier
/// within the Zcash Wallet Interchange Format (ZeWIF). It provides type safety, consistent
/// hexadecimal formatting, and binary parsing capabilities, making it suitable for
/// standardized identification across different wallet implementations.
///
/// # Zcash Concept Relation
/// In the Zcash Wallet Interchange Format specification, standardized identifiers
/// are essential for maintaining consistent references during wallet migration.
/// While not currently used in the core code, `IntID` is designed to provide a
/// foundation for:
///
/// - **Component Referencing**: Providing stable identifiers for wallet structures
/// - **Serialization**: Supporting consistent binary representation in the ZeWIF format
/// - **Derivation Indices**: Potentially representing HD wallet derivation indices
///
/// Unlike cryptographic identifiers (like TxId), these simple numeric IDs are intended for
/// internal referencing within the wallet data structures.
///
/// # Data Preservation
/// In future wallet migrations using ZeWIF, the following aspects of integer identifiers
/// would be preserved:
///
/// - **Raw Value**: The exact 32-bit unsigned integer value
/// - **Binary Format**: Consistent serialization for interchange purposes
/// - **Display Format**: Standardized hexadecimal representation with `0x` prefix
///
/// # Implementation Details
/// `IntID` implements:
///
/// - **Display**: Formats as hexadecimal with `0x` prefix and leading zeros
/// - **Parse**: Supports binary parsing within ZeWIF data streams
/// - **Conversion**: Seamless conversion to/from raw u32 values
///
/// # Examples
/// ```
/// use zewif::IntID;
///
/// // Create an ID from a u32 value
/// let id = IntID::new(42);
/// 
/// // IDs are displayed in hexadecimal format with 0x prefix and leading zeros
/// assert_eq!(format!("{}", id), "0x0000002a");
/// 
/// // Convert back to raw u32 when needed
/// let raw_id: u32 = id.into();
/// assert_eq!(raw_id, 42);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct IntID(u32);

impl IntID {
    /// Creates a new ID from a u32 value.
    ///
    /// # Arguments
    /// * `id` - The raw integer value to wrap in an `IntID`
    ///
    /// # Examples
    /// ```
    /// use zewif::IntID;
    ///
    /// let id = IntID::new(123);
    /// ```
    pub const fn new(id: u32) -> Self {
        IntID(id)
    }

    /// Returns the raw u32 value of this ID.
    ///
    /// This method provides direct access to the underlying integer value
    /// when needed for calculations or comparisons.
    ///
    /// # Returns
    /// The raw u32 value contained in this ID
    ///
    /// # Examples
    /// ```
    /// use zewif::IntID;
    ///
    /// let id = IntID::new(123);
    /// assert_eq!(id.value(), 123);
    /// ```
    pub const fn value(&self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for IntID {
    // Always display as hex with `0x` prefix
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:08x}", self.0)
    }
}

impl std::fmt::Debug for IntID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Parse for IntID {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self(parse!(p, "IntID")?))
    }
}
impl From<u32> for IntID {
    fn from(id: u32) -> Self {
        IntID(id)
    }
}

impl From<IntID> for u32 {
    fn from(id: IntID) -> Self {
        id.0
    }
}
