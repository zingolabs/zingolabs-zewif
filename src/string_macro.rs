#[macro_export]
macro_rules! string {
    ($name:ident) => {
        pub struct $name(String);

        impl Clone for $name {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl Eq for $name {}

        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}({:?})", stringify!($name), self.0)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self(String::new())
            }
        }

        impl $crate::parser::Parse for $name {
            fn parse(p: &mut $crate::parser::Parser) -> ::anyhow::Result<Self> {
                Ok(Self($crate::parse!(p, "string")?))
            }
        }

        impl From<$name> for String {
            fn from(s: $name) -> Self {
                s.0
            }
        }

        impl From<&$name> for String {
            fn from(s: &$name) -> Self {
                s.0.clone()
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self(s)
            }
        }
    };
}
