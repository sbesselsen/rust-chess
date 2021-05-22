use std::fmt;

use super::Piece;

#[derive(Debug,Copy,PartialEq,Clone,Hash)]
pub enum Square {
    Occupied(Piece),
    Empty
}

impl Square {
    pub fn is_empty(&self) -> bool {
        matches!(self, Square::Empty)
    }

    pub fn is_occupied_by(&self, piece: Piece) -> bool {
        matches!(self, Square::Occupied(my_piece) if piece == *my_piece)
    }
}

impl fmt::Display for Square {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Square::Empty => write!(f, " "),
            &Square::Occupied(piece) => write!(f, "{}", piece)
        }
    }
}
