use super::Blob;

/// Seed material used to generate the keys in the wallet.
/// Proposal as minimal set of sources of truth
#[derive(Clone)]
pub enum SeedMaterial {
    Bip39Mnemonic(String),
    PreBIP39Seed(Blob<32>),
}

impl std::fmt::Debug for SeedMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bip39Mnemonic(phrase) => write!(f, "SeedMaterial::Bip39Mnemonic(\"{}\")", phrase),
            Self::PreBIP39Seed(seed) => write!(f, "SeedMaterial::PreBIP39Seed({:?})", seed),
        }
    }
}

impl std::fmt::Display for SeedMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bip39Mnemonic(phrase) => write!(f, "SeedMaterial::Bip39Mnemonic(\"{}\")", phrase),
            Self::PreBIP39Seed(seed) => write!(f, "SeedMaterial::PreBIP39Seed({:?})", seed),
        }
    }
}
