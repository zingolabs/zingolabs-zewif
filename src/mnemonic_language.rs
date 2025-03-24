use anyhow::{Result, bail};

use crate::{parse, parser::prelude::*};

#[derive(Clone, Copy, PartialEq)]
pub enum MnemonicLanguage {
    English = 0,
    SimplifiedChinese = 1,
    TraditionalChinese = 2,
    Czech = 3,
    French = 4,
    Italian = 5,
    Japanese = 6,
    Korean = 7,
    Portuguese = 8,
    Spanish = 9,
}

impl MnemonicLanguage {
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

impl std::fmt::Display for MnemonicLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::fmt::Debug for MnemonicLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Parse for MnemonicLanguage {
    fn parse(p: &mut Parser) -> Result<Self> {
        let value = parse!(p, "value")?;
        MnemonicLanguage::from_u32(value)
    }
}
