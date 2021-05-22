use std::fmt;

use crate::board::{ Color, Kind };

#[derive(Debug,Copy,PartialEq,Clone,Hash)]
pub struct Piece(pub Color, pub Kind);

impl fmt::Display for Piece {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            &Piece(Color::White, Kind::Rook) => "♖",
            &Piece(Color::White, Kind::Knight) => "♘",
            &Piece(Color::White, Kind::Bishop) => "♗",
            &Piece(Color::White, Kind::Queen) => "♕",
            &Piece(Color::White, Kind::King) => "♔",
            &Piece(Color::White, Kind::Pawn) => "♙",
            &Piece(Color::Black, Kind::Rook) => "♜",
            &Piece(Color::Black, Kind::Knight) => "♞",
            &Piece(Color::Black, Kind::Bishop) => "♝",
            &Piece(Color::Black, Kind::Queen) => "♛",
            &Piece(Color::Black, Kind::King) => "♚",
            &Piece(Color::Black, Kind::Pawn) => "♟︎",
        })
    }
}
