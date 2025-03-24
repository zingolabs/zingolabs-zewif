#[macro_export]
macro_rules! data {
    ($name:ident) => {
        pub struct $name($crate::Data);

        impl $name {
            pub fn new(data: Vec<u8>) -> Self {
                Self($crate::Data::from_vec(data))
            }

            pub fn len(&self) -> usize {
                self.0.len()
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            pub fn to_vec(&self) -> Vec<u8> {
                self.0.to_vec()
            }

            pub fn from_slice(data: &[u8]) -> Self {
                Self($crate::Data::from_slice(data))
            }

            pub fn from_vec(data: Vec<u8>) -> Self {
                Self($crate::Data::from_vec(data))
            }

            pub fn from_hex(hex: &str) -> Self {
                Self($crate::Data::from_hex(hex).unwrap())
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

        impl Default for $name {
            fn default() -> Self {
                Self($crate::Data::default())
            }
        }

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &[u8] {
                self.0.as_ref()
            }
        }

        impl From<$name> for Vec<u8> {
            fn from(data: $name) -> Vec<u8> {
                data.to_vec()
            }
        }

        impl From<&$name> for Vec<u8> {
            fn from(data: &$name) -> Vec<u8> {
                data.to_vec()
            }
        }

        impl From<Vec<u8>> for $name {
            fn from(data: Vec<u8>) -> Self {
                Self::from_vec(data)
            }
        }

        impl From<&[u8]> for $name {
            fn from(data: &[u8]) -> Self {
                Self::from_slice(data)
            }
        }

        impl $crate::Parse for $name {
            fn parse(parser: &mut $crate::Parser) -> ::anyhow::Result<Self> {
                Ok(Self($crate::Data::parse(parser)?))
            }
        }
    };
}
