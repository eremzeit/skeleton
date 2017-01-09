use std::fmt;
use constants::*;
use util::*;
use std::cmp;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position(pub File, pub Rank);

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", file_to_char(self.0), self.1 + 1)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct PiecePosition(pub PieceType, pub File, pub Rank);

impl PiecePosition {
    pub fn to_position(&self) -> Position {
        Position(self.1, self.2)
    }
}

impl fmt::Debug for PiecePosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", piece_type_to_char(self.0), file_to_char(self.1), self.2 + 1)
    }
}

pub type PieceList = Vec<PiecePosition>;

