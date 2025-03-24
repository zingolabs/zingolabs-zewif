use crate::impl_attachable;
use super::{Anchor, Attachments, SproutProof, u256};

/// For legacy Sprout transactions: JoinSplit descriptions that mix transparent and shielded values.
#[derive(Debug, Clone)]
pub struct JoinSplitDescription {
    anchor: Anchor,
    nullifiers: [u256; 2],
    commitments: [u256; 2],
    zkproof: SproutProof,
    attachments: Attachments,
}

impl_attachable!(JoinSplitDescription);

impl JoinSplitDescription {
    pub fn new(
        anchor: Anchor,
        nullifiers: [u256; 2],
        commitments: [u256; 2],
        zkproof: SproutProof,
    ) -> Self {
        Self {
            anchor,
            nullifiers,
            commitments,
            zkproof,
            attachments: Attachments::default(),
        }
    }

    pub fn anchor(&self) -> Anchor {
        self.anchor
    }

    pub fn nullifiers(&self) -> [u256; 2] {
        self.nullifiers
    }

    pub fn commitments(&self) -> [u256; 2] {
        self.commitments
    }

    pub fn zkproof(&self) -> &SproutProof {
        &self.zkproof
    }

    pub fn attachments(&self) -> &Attachments {
        &self.attachments
    }
}
