#[derive(Debug, Clone, Copy)]
pub struct DerivationInfo {
    change: NonHardenedChildIndex,
    address_index: NonHardenedChildIndex,
}

impl DerivationInfo {
    pub fn change(&self) -> NonHardenedChildIndex {
        self.change
    }

    pub fn address_index(&self) -> NonHardenedChildIndex {
        self.address_index
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NonHardenedChildIndex(u32);

impl From<u32> for NonHardenedChildIndex {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<NonHardenedChildIndex> for u32 {
    fn from(value: NonHardenedChildIndex) -> Self {
        value.0
    }
}
