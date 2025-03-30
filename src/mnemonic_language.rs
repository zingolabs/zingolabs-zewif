use anyhow::{Result, bail};

use crate::{parse, parser::prelude::*};

/// The language used for BIP-39/BIP-44 mnemonic seed phrases in a wallet.
///
/// `MnemonicLanguage` represents the human language in which a wallet's recovery
/// seed phrase is expressed. Zcash wallets, like many cryptocurrency wallets, use
/// mnemonic phrases (typically 12, 18, or 24 words) as a human-readable backup
/// for the wallet's seed material.
///
/// # Zcash Concept Relation
/// Zcash wallets commonly use BIP-39 mnemonics as the basis for hierarchical
/// deterministic (HD) wallet generation. The language of the mnemonic matters because:
///
/// - Each language has its own standardized wordlist with 2048 specific words
/// - Words must be from the same language to be valid for a given mnemonic
/// - The language determines which wordlist is used for validation and generation
///
/// Zcash wallets typically follow the same standards as Bitcoin for mnemonic
/// implementation, supporting multiple languages for wider accessibility.
///
/// # Data Preservation
/// During wallet migration, preserving the mnemonic language is important for:
///
/// - Correctly interpreting any stored mnemonic phrases
/// - Maintaining consistent wallet recovery procedures
/// - Ensuring users can recover funds using their preferred language
///
/// # Examples
/// ```
/// use zewif::MnemonicLanguage;
/// use anyhow::Result;
///
/// // Create a language identifier from a numeric value
/// let language = MnemonicLanguage::from_u32(0)?;
/// assert_eq!(language, MnemonicLanguage::English);
///
/// // Get the language name
/// assert_eq!(language.name(), "English");
///
/// // Languages can be displayed as strings
/// println!("Mnemonic language: {}", language); // Outputs "Mnemonic language: English"
/// # Ok::<(), anyhow::Error>(())
/// ```
#[derive(Clone, Copy, PartialEq)]
pub enum MnemonicLanguage {
    /// English wordlist (the most commonly used)
    English = 0,
    /// Simplified Chinese wordlist
    SimplifiedChinese = 1,
    /// Traditional Chinese wordlist
    TraditionalChinese = 2,
    /// Czech wordlist
    Czech = 3,
    /// French wordlist
    French = 4,
    /// Italian wordlist
    Italian = 5,
    /// Japanese wordlist
    Japanese = 6,
    /// Korean wordlist
    Korean = 7,
    /// Portuguese wordlist
    Portuguese = 8,
    /// Spanish wordlist
    Spanish = 9,
}

impl MnemonicLanguage {
    /// Creates a `MnemonicLanguage` from a numeric identifier.
    ///
    /// This method converts a raw numeric value into the corresponding language
    /// enum variant. Zcash wallets typically store the language as a numeric value
    /// in their configuration or database.
    ///
    /// # Arguments
    /// * `value` - The numeric identifier for the language (0-9)
    ///
    /// # Returns
    /// A `Result` containing the `MnemonicLanguage` if the value is valid,
    /// or an error if the value doesn't correspond to a supported language.
    ///
    /// # Examples
    /// ```
    /// use zewif::MnemonicLanguage;
    /// use anyhow::Result;
    ///
    /// // Create English (most common) language
    /// let english = MnemonicLanguage::from_u32(0)?;
    ///
    /// // Create French language
    /// let french = MnemonicLanguage::from_u32(4)?;
    ///
    /// // Invalid value results in error
    /// let result = MnemonicLanguage::from_u32(99);
    /// assert!(result.is_err());
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn from_u32(value: u32) -> Result<Self> {
        match value {
            0 => Ok(MnemonicLanguage::English),
            1 => Ok(MnemonicLanguage::SimplifiedChinese),
            2 => Ok(MnemonicLanguage::TraditionalChinese),
            3 => Ok(MnemonicLanguage::Czech),
            4 => Ok(MnemonicLanguage::French),
            5 => Ok(MnemonicLanguage::Italian),
            6 => Ok(MnemonicLanguage::Japanese),
            7 => Ok(MnemonicLanguage::Korean),
            8 => Ok(MnemonicLanguage::Portuguese),
            9 => Ok(MnemonicLanguage::Spanish),
            _ => bail!("Invalid language value: {}", value),
        }
    }

    /// Returns the string name of the mnemonic language.
    ///
    /// This method returns the canonical name of the language as a static string,
    /// which is useful for display purposes or when translating between numeric
    /// and human-readable representations.
    ///
    /// # Returns
    /// A static string containing the name of the language.
    ///
    /// # Examples
    /// ```
    /// use zewif::MnemonicLanguage;
    ///
    /// let language = MnemonicLanguage::English;
    /// assert_eq!(language.name(), "English");
    ///
    /// let language = MnemonicLanguage::Japanese;
    /// assert_eq!(language.name(), "Japanese");
    /// ```
    pub fn name(&self) -> &'static str {
        match self {
            MnemonicLanguage::English => "English",
            MnemonicLanguage::SimplifiedChinese => "SimplifiedChinese",
            MnemonicLanguage::TraditionalChinese => "TraditionalChinese",
            MnemonicLanguage::Czech => "Czech",
            MnemonicLanguage::French => "French",
            MnemonicLanguage::Italian => "Italian",
            MnemonicLanguage::Japanese => "Japanese",
            MnemonicLanguage::Korean => "Korean",
            MnemonicLanguage::Portuguese => "Portuguese",
            MnemonicLanguage::Spanish => "Spanish",
        }
    }
}

/// Formats the mnemonic language as a human-readable string
impl std::fmt::Display for MnemonicLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Formats the mnemonic language for debugging output
impl std::fmt::Debug for MnemonicLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

/// Parses a mnemonic language from binary data
///
/// This implementation allows `MnemonicLanguage` to be parsed from a binary
/// stream using the `parse!` macro, which is particularly useful when reading
/// wallet file data.
///
/// # Examples
/// ```no_run
/// # use zewif::MnemonicLanguage;
/// # use zewif::parser::Parser;
/// # use zewif::parse;
/// # use anyhow::Result;
/// #
/// # fn example(parser: &mut Parser) -> Result<()> {
/// // Parse a mnemonic language from wallet binary data
/// let language = parse!(parser, MnemonicLanguage, "mnemonic language")?;
/// # Ok(())
/// # }
/// ```
impl Parse for MnemonicLanguage {
    fn parse(p: &mut Parser) -> Result<Self> {
        let value = parse!(p, "language value")?;
        MnemonicLanguage::from_u32(value)
    }
}
