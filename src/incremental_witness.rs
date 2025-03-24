use anyhow::Result;

use super::IncrementalMerkleTree;
use super::{parse, parser::prelude::*};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IncrementalWitness<const DEPTH: usize, Hash> {
    tree: IncrementalMerkleTree,
    filled: Vec<Hash>,
    cursor: Option<IncrementalMerkleTree>,
}

impl<const DEPTH: usize, Hash> IncrementalWitness<DEPTH, Hash> {
    pub fn with_fields(
        tree: IncrementalMerkleTree,
        filled: Vec<Hash>,
        cursor: Option<IncrementalMerkleTree>,
    ) -> Self {
        Self {
            tree,
            filled,
            cursor,
        }
    }

    pub fn tree(&self) -> &IncrementalMerkleTree {
        &self.tree
    }

    pub fn filled(&self) -> &Vec<Hash> {
        &self.filled
    }

    pub fn cursor(&self) -> &Option<IncrementalMerkleTree> {
        &self.cursor
    }
}

impl<const DEPTH: usize, Hash: Parse> Parse for IncrementalWitness<DEPTH, Hash> {
    fn parse(p: &mut Parser) -> Result<Self> {
        let tree = parse!(p, "tree")?;
        let filled = parse!(p, "filled")?;
        let cursor = parse!(p, "cursor")?;
        Ok(Self::with_fields(tree, filled, cursor))
    }
}
