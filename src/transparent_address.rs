use super::{DerivationInfo, TransparentSpendAuthority};

#[derive(Debug, Clone)]
pub struct TransparentAddress {
    address: String, // Unique
    spend_authority: Option<TransparentSpendAuthority>,
    derivation_info: Option<DerivationInfo>,
}

impl TransparentAddress {
    pub fn new(address: impl Into<String>) -> Self {
        TransparentAddress {
            address: address.into(),
            spend_authority: None,
            derivation_info: None,
        }
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn spend_authority(&self) -> Option<&TransparentSpendAuthority> {
        self.spend_authority.as_ref()
    }

    pub fn set_spend_authority(&mut self, spend_authority: TransparentSpendAuthority) {
        self.spend_authority = Some(spend_authority);
    }

    pub fn derivation_info(&self) -> Option<&DerivationInfo> {
        self.derivation_info.as_ref()
    }

    pub fn set_derivation_info(&mut self, derivation_info: DerivationInfo) {
        self.derivation_info = Some(derivation_info);
    }
}
