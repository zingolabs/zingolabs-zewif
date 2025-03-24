/// A position in a note commitment tree.
#[derive(Clone, Copy, Debug, Default)]
pub struct Position(u32);

impl From<u32> for Position {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Position> for u32 {
    fn from(value: Position) -> Self {
        value.0
    }
}

impl From<usize> for Position {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}
