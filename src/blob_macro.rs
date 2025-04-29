/// Creates a new type wrapping a fixed-size byte array with common methods and trait implementations.
///
/// The `blob!` macro generates a new type that wraps a [`Blob<N>`](crate::Blob) of the specified size,
/// automatically implementing common methods and traits. This provides a convenient way to
/// create domain-specific types for fixed-size binary data with minimal boilerplate.
///
/// # Usage
///
/// ```
/// # use zewif::blob;
/// #
/// blob!(TxId, 32, "A transaction identifier as a 32-byte hash");
/// ```
///
/// # Generated Functionality
///
/// The generated type includes methods for creation, conversion, and inspection,
/// as well as implementations for common traits like `Parse`, `Debug`, `Clone`,
/// and various conversion traits to and from byte collections.
///
/// The macro adds type safety and domain-specific semantics to otherwise
/// generic byte array data, particularly for cryptographic values used in the
/// Zcash protocol.
#[macro_export]
macro_rules! blob {
    ($name:ident, $size:expr, $doc:expr) => {
        #[doc = $doc]
        pub struct $name($crate::Blob<$size>);

        impl $name {
            /// Creates a new instance from a fixed-size byte array.
            ///
            /// This is the primary constructor when you have an exact-sized array available.
            pub fn new(data: [u8; $size]) -> Self {
                Self($crate::Blob::new(data))
            }

            /// Returns the length of this blob in bytes.
            ///
            /// This will always return `$size` for this type.
            pub fn len(&self) -> usize {
                self.0.len()
            }

            /// Returns `true` if this blob contains no bytes.
            ///
            /// This will always return `false` for this type (unless `$size` is 0).
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            /// Converts this blob to a `Vec<u8>`, creating a copy of the data.
            pub fn to_vec(&self) -> Vec<u8> {
                self.0.to_vec()
            }

            /// Creates an instance from a slice of bytes.
            ///
            /// # Errors
            /// Returns an error if the slice's length doesn't match the expected size ($size).
            pub fn from_slice(data: &[u8]) -> ::anyhow::Result<Self> {
                Ok(Self($crate::Blob::from_slice(data)?))
            }

            /// Creates an instance from a `Vec<u8>`.
            ///
            /// # Errors
            /// Returns an error if the vector's length doesn't match the expected size ($size).
            pub fn from_vec(data: Vec<u8>) -> ::anyhow::Result<Self> {
                Ok(Self($crate::Blob::from_vec(data)?))
            }

            /// Creates an instance from a hexadecimal string.
            ///
            /// # Panics
            /// Panics if the hex string cannot be decoded or if the decoded bytes
            /// don't match the expected length ($size).
            pub fn from_hex(hex: &str) -> Result<Self, $crate::HexParseError> {
                Ok(Self($crate::Blob::from_hex(hex)?))
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl Eq for $name {}

        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state)
            }
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}({:?})", stringify!($name), self.0)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}({:?})", stringify!($name), self.0)
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self($crate::Blob::default())
            }
        }

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &[u8] {
                self.0.as_ref()
            }
        }

        impl From<$name> for Vec<u8> {
            fn from(blob: $name) -> Vec<u8> {
                blob.to_vec()
            }
        }

        impl From<&$name> for Vec<u8> {
            fn from(blob: &$name) -> Vec<u8> {
                blob.to_vec()
            }
        }

        impl From<Vec<u8>> for $name {
            fn from(data: Vec<u8>) -> Self {
                Self::from_vec(data).unwrap()
            }
        }

        impl From<&[u8]> for $name {
            fn from(data: &[u8]) -> Self {
                Self::from_slice(data).unwrap()
            }
        }

        impl From<$name> for $crate::Blob<$size> {
            fn from(obj: $name) -> $crate::Blob<$size> {
                obj.0
            }
        }

        impl $crate::parser::Parse for $name {
            /// Parses this type from a binary data stream.
            ///
            /// This implementation allows the type to be used with the `parse!` macro.
            fn parse(parser: &mut $crate::parser::Parser) -> ::anyhow::Result<Self>
            where
                Self: Sized,
            {
                let bytes = ::anyhow::Context::with_context(parser.next($size), || {
                    format!("Parsing {}", stringify!($name))
                })?;
                Ok(Self($crate::Blob::from(bytes)))
            }
        }
    };
}
