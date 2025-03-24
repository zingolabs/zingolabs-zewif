use super::u256;

use super::IncrementalWitness;

const INCREMENTAL_MERKLE_TREE_DEPTH: usize = 29;
pub type SHA256Compress = u256;
pub type SproutWitness = IncrementalWitness<INCREMENTAL_MERKLE_TREE_DEPTH, SHA256Compress>;
