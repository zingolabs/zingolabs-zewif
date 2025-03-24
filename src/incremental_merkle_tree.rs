use anyhow::Result;

use super::u256;

use super::{parse, parser::prelude::*};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IncrementalMerkleTree {
    left: Option<u256>,
    right: Option<u256>,
    parents: Vec<Option<u256>>,
}

impl IncrementalMerkleTree {
    pub fn new() -> Self {
        Self {
            left: None,
            right: None,
            parents: Vec::new(),
        }
    }

    pub fn with_fields(left: Option<u256>, right: Option<u256>, parents: Vec<Option<u256>>) -> Self {
        Self {
            left,
            right,
            parents,
        }
    }

    pub fn left(&self) -> Option<u256> {
        self.left
    }

    pub fn set_left(&mut self, left: u256) {
        self.left = Some(left);
    }

    pub fn right(&self) -> Option<u256> {
        self.right
    }

    pub fn set_right(&mut self, right: u256) {
        self.right = Some(right);
    }

    pub fn parents(&self) -> &Vec<Option<u256>> {
        &self.parents
    }

    pub fn push_parent(&mut self, parent: Option<u256>) {
        self.parents.push(parent);
    }
}

impl Default for IncrementalMerkleTree {
    fn default() -> Self {
        Self::new()
    }
}

impl Parse for IncrementalMerkleTree {
    fn parse(p: &mut Parser) -> Result<Self> {
        let left = parse!(p, "left")?;
        let right = parse!(p, "right")?;
        let parents = parse!(p, "parents")?;
        Ok(Self::with_fields(left, right, parents))
    }
}
