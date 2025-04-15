use bc_envelope::prelude::*;
use anyhow::{Result, Context};

use crate::{parse, test_envelope_roundtrip, u256, MnemonicLanguage, NoQuotesDebugOption};
use crate::parser::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Bip39Mnemonic {
    mnemonic: String,
    language: Option<MnemonicLanguage>,
    fingerprint: Option<u256>,
}

impl std::fmt::Debug for Bip39Mnemonic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MnemonicSeed")
            .field("language", &NoQuotesDebugOption(&self.language))
            .field("mnemonic", &self.mnemonic)
            .field("fingerprint", &NoQuotesDebugOption(&self.fingerprint))
            .finish()
    }
}

impl Bip39Mnemonic {
    pub fn new(mnemonic: impl AsRef<str>, language: Option<MnemonicLanguage>) -> Self {
        Self { mnemonic: mnemonic.as_ref().to_string(), language, fingerprint: None }
    }

    pub fn set_fingerprint(&mut self, fingerprint: u256) {
        self.fingerprint = Some(fingerprint);
    }

    pub fn mnemonic(&self) -> &String {
        &self.mnemonic
    }

    pub fn set_mnemonic(&mut self, mnemonic: String) {
        self.mnemonic = mnemonic;
    }

    pub fn language(&self) -> Option<&MnemonicLanguage> {
        self.language.as_ref()
    }

    pub fn fingerprint(&self) -> Option<&u256> {
        self.fingerprint.as_ref()
    }

    pub fn set_language(&mut self, language: MnemonicLanguage) {
        self.language = Some(language);
    }
}

impl Parse for Bip39Mnemonic {
    fn parse(p: &mut Parser) -> Result<Self> {
        let language = parse!(p, "language")?;
        let mnemonic = parse!(p, String, "mnemonic")?;
        let bip39_mnemonic = Self::new(mnemonic, language);
        Ok(bip39_mnemonic)
    }
}

impl From<Bip39Mnemonic> for Envelope {
    fn from(value: Bip39Mnemonic) -> Self {
        Envelope::new(value.mnemonic)
            .add_type("Bip39Mnemonic")
            .add_optional_assertion("language", value.language)
            .add_optional_assertion("fingerprint", value.fingerprint)
    }
}

impl TryFrom<Envelope> for Bip39Mnemonic {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self, Self::Error> {
        envelope.check_type_envelope("Bip39Mnemonic").context("Bip39Mnemonic")?;
        let mnemonic = envelope.extract_subject().context("mnemonic")?;
        let language = envelope.try_optional_object_for_predicate("language").context("language")?;
        let fingerprint = envelope.try_optional_object_for_predicate("fingerprint").context("fingerprint")?;
        Ok(Self {
            mnemonic,
            language,
            fingerprint,
        })
    }
}

#[cfg(test)]
impl crate::RandomInstance for Bip39Mnemonic {
    fn random() -> Self {
        Self {
            mnemonic: String::random(),
            language: MnemonicLanguage::opt_random(),
            fingerprint: u256::opt_random(),
        }
    }
}

test_envelope_roundtrip!(Bip39Mnemonic);
