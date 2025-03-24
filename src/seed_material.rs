use super::Blob;

/// Seed material used to generate the keys in the wallet.
/// Proposal as minimal set of sources of truth
#[derive(Debug, Clone)]
pub enum SeedMaterial {
    Bip39Mnemonic(String),
    PreBIP39Seed(Blob<32>),
}
