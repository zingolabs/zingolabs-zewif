use crate::impl_attachable;
use super::super::{Amount, Attachments, BlockHeight, GrothProof, u256};

/// Data specific to Sapling spends.
#[derive(Debug, Clone, Default)]
pub struct SaplingSpendDescription {
    spend_index: u32,
    /// The value of the input note, if known.
    value: Option<Amount>,
    /// The height that the anchor corresponds to, if known.
    anchor_height: Option<BlockHeight>,
    /// A nullifier to ensure the note is spent only once.
    nullifier: u256,
    /// A zero-knowledge proof that the spend is valid.
    zkproof: GrothProof,
    // Additional fields (e.g., spending key components) may be required.
    attachments: Attachments,
}

impl_attachable!(SaplingSpendDescription);

impl SaplingSpendDescription {
    /// Creates a new empty SaplingSpendDescription.
    pub fn new() -> Self {
        Self {
            spend_index: 0,
            value: None,
            anchor_height: None,
            nullifier: u256::default(),
            zkproof: GrothProof::default(),
            attachments: Attachments::new(),
        }
    }

    // Getters
    pub fn spend_index(&self) -> u32 {
        self.spend_index
    }

    pub fn value(&self) -> Option<Amount> {
        self.value
    }

    pub fn anchor_height(&self) -> Option<BlockHeight> {
        self.anchor_height
    }

    pub fn nullifier(&self) -> &u256 {
        &self.nullifier
    }

    pub fn zkproof(&self) -> &GrothProof {
        &self.zkproof
    }

    pub fn attachments(&self) -> &Attachments {
        &self.attachments
    }

    // Setters
    pub fn set_spend_index(&mut self, spend_index: u32) -> &mut Self {
        self.spend_index = spend_index;
        self
    }

    pub fn set_value(&mut self, value: Option<Amount>) -> &mut Self {
        self.value = value;
        self
    }

    pub fn set_anchor_height(&mut self, anchor_height: Option<BlockHeight>) -> &mut Self {
        self.anchor_height = anchor_height;
        self
    }

    pub fn set_nullifier(&mut self, nullifier: u256) -> &mut Self {
        self.nullifier = nullifier;
        self
    }

    pub fn set_zkproof(&mut self, zkproof: GrothProof) -> &mut Self {
        self.zkproof = zkproof;
        self
    }
}
