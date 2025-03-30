/// A position index in a ZCash note commitment tree.
///
/// `Position` represents the index of a note commitment in Zcash's merkle trees,
/// which are used to efficiently prove note existence in shielded transactions.
/// Each shielded note is committed to a merkle tree at a specific position,
/// allowing for compact inclusion proofs.
///
/// # Zcash Concept Relation
/// In Zcash's shielded protocols (both Sapling and Orchard), note commitments
/// are stored in append-only merkle trees. Each note has a unique position in
/// the tree that:
///
/// - Identifies where in the tree the note commitment is stored
/// - Is needed to generate a witness path for proving note ownership
/// - Is required for spending a note in a later transaction
///
/// When a note is spent, the sender must prove they know the note at a particular
/// position in the tree, without revealing which specific note is being spent.
///
/// # Data Preservation
/// The `Position` type preserves the exact numeric position identifiers from wallet data,
/// which is critical for being able to spend notes after wallet migration.
///
/// Internally, positions are stored as unsigned 32-bit integers, allowing for
/// up to 4 billion notes in a commitment tree.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Position(u32);

/// Debug formatting that shows the numeric position value
impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Position({})", self.0)
    }
}

/// Creates a Position from a u32 value
impl From<u32> for Position {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

/// Extracts the u32 value from a Position
impl From<Position> for u32 {
    fn from(value: Position) -> Self {
        value.0
    }
}

/// Creates a Position from a usize value (useful for array indexing)
impl From<usize> for Position {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}
