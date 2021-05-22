use std::fmt;

use super::{ Color, Coordinates, Kind, Piece, Square };

#[derive(Debug,Copy,PartialEq,Clone,Hash)]
pub struct Board {
    squares: [Square; 64],
    pub en_passant_capturable: Option<usize>,
    white_can_king_castle: bool,
    white_can_queen_castle: bool,
    black_can_king_castle: bool,
    black_can_queen_castle: bool,
}

pub enum CastlingSide {
    King,
    Queen
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: [Square::Empty; 64],
            en_passant_capturable: None,
            white_can_king_castle: false,
            white_can_queen_castle: false,
            black_can_king_castle: false,
            black_can_queen_castle: false,
        }
    }

    pub fn setup(&mut self) {
        fn assign_start_row(board: &mut Board, rank: usize, color: Color) {
            let start_index = rank * 8;
            board.squares[start_index + 0] = Square::Occupied(Piece(color, Kind::Rook));
            board.squares[start_index + 1] = Square::Occupied(Piece(color, Kind::Knight));
            board.squares[start_index + 2] = Square::Occupied(Piece(color, Kind::Bishop));
            board.squares[start_index + 3] = Square::Occupied(Piece(color, Kind::Queen));
            board.squares[start_index + 4] = Square::Occupied(Piece(color, Kind::King));
            board.squares[start_index + 5] = Square::Occupied(Piece(color, Kind::Bishop));
            board.squares[start_index + 6] = Square::Occupied(Piece(color, Kind::Knight));
            board.squares[start_index + 7] = Square::Occupied(Piece(color, Kind::Rook));
        }

        fn assign_pawn_row(board: &mut Board, rank: usize, color: Color) {
            let start_index = rank * 8;
            for i in start_index..(start_index + 8) {
                board.squares[i] = Square::Occupied(Piece(color, Kind::Pawn));
            }
        }

        assign_start_row(self, 0, Color::White);
        assign_pawn_row(self, 1, Color::White);
        assign_start_row(self, 7, Color::Black);
        assign_pawn_row(self, 6, Color::Black);

        *self = Self {
            en_passant_capturable: None,
            white_can_king_castle: true,
            white_can_queen_castle: true,
            black_can_king_castle: true,
            black_can_queen_castle: true,
            ..*self
        }
    }

    pub fn move_piece(&mut self, from: Coordinates, to: Coordinates) {
        let index = from.index();
        let target_index = to.index();

        // Match on what's on the target square before the move, to process captures.
        match self.squares[target_index] {
            Square::Occupied(Piece(Color::White, Kind::Rook)) if target_index == 0 => {
                // White can't castle with its queen's rook after it is captured.
                self.white_can_queen_castle = false;
            },
            Square::Occupied(Piece(Color::White, Kind::Rook)) if target_index == 7 => {
                // White can't castle with its king's rook after it is captured.
                self.white_can_king_castle = false;
            },
            Square::Occupied(Piece(Color::Black, Kind::Rook)) if target_index == 56 => {
                // Black can't castle with its queen's rook after it is captured.
                self.black_can_queen_castle = false;
            },
            Square::Occupied(Piece(Color::Black, Kind::Rook)) if target_index == 63 => {
                // Black can't castle with its queen's rook after it is captured.
                self.black_can_king_castle = false;
            },
            _ => {}
        }

        // Make the move.
        self.squares[target_index] = self.squares[index];
        self.squares[index] = Square::Empty;

        // Handle en passant captures.
        if let Some(en_passant_index) = self.en_passant_capturable {
            if en_passant_index == target_index {
                // We are making an en passant capture. Remove the captured piece.
                match self.squares[target_index] {
                    Square::Occupied(Piece(Color::White, Kind::Pawn)) => {
                        self.squares[target_index - 8] = Square::Empty;
                    },
                    Square::Occupied(Piece(Color::Black, Kind::Pawn)) => {
                        self.squares[target_index + 8] = Square::Empty;
                    },
                    _ => {}
                }
            }
        }

        self.en_passant_capturable = None;

        // Process the board based on what we have done.
        match self.squares[target_index] {
            Square::Occupied(Piece(_, Kind::Pawn)) => {
                if target_index == index + 16 {
                    // Pawn moved 2 places. Reflect this on the board.
                    self.en_passant_capturable = Some(index + 8)
                } else if index == target_index + 16 {
                    // Pawn moved 2 places. Reflect this on the board.
                    self.en_passant_capturable = Some(target_index + 8)
                }
            },
            Square::Occupied(Piece(Color::White, Kind::King)) => {
                // White can't castle after moving its king.
                self.white_can_king_castle = false;
                self.white_can_queen_castle = false;
            },
            Square::Occupied(Piece(Color::Black, Kind::King)) => {
                // Black can't castle after moving its king.
                self.black_can_king_castle = false;
                self.black_can_queen_castle = false;
            },
            Square::Occupied(Piece(Color::White, Kind::Rook)) if index == 0 => {
                // White can't castle with its queen's rook after moving it.
                self.white_can_queen_castle = false;
            },
            Square::Occupied(Piece(Color::White, Kind::Rook)) if index == 7 => {
                // White can't castle with its king's rook after moving it.
                self.white_can_king_castle = false;
            },
            Square::Occupied(Piece(Color::Black, Kind::Rook)) if index == 56 => {
                // Black can't castle with its queen's rook after moving it.
                self.black_can_queen_castle = false;
            },
            Square::Occupied(Piece(Color::Black, Kind::Rook)) if index == 63 => {
                // Black can't castle with its queen's rook after moving it.
                self.black_can_king_castle = false;
            },
            _ => {}
        }
    }

    pub fn clone_move_piece(&self, from: Coordinates, to: Coordinates) -> Board {
        let mut clone = self.clone();
        clone.move_piece(from, to);
        clone
    }

    pub fn get_square(&self, coordinates: Coordinates) -> Square {
        self.squares[coordinates.index()]
    }

    pub fn set_square(&mut self, coordinates: Coordinates, value: Square) {
        self.squares[coordinates.index()] = value
    }

    pub fn squares(&self) -> &[Square] {
        &self.squares
    }

    pub fn squares_coordinates_iter(&self) -> impl Iterator<Item=(Coordinates, &Square)> + '_ {
        self.squares().iter().enumerate().map(|(index, square)| (Coordinates::new_from_index(index).unwrap(), square))
    }

    pub fn set_castling_allowed(&mut self, color: Color, side: CastlingSide, allowed: bool) {
        match (color, side) {
            (Color::White, CastlingSide::King) => {
                self.white_can_king_castle = allowed;
            },
            (Color::White, CastlingSide::Queen) => {
                self.white_can_queen_castle = allowed;
            },
            (Color::Black, CastlingSide::King) => {
                self.black_can_king_castle = allowed;
            },
            (Color::Black, CastlingSide::Queen) => {
                self.black_can_queen_castle = allowed;
            },
        }
    }

    pub fn is_castling_allowed(&self, color: Color, side: CastlingSide) -> bool {
        match (color, side) {
            (Color::White, CastlingSide::King) => self.white_can_king_castle,
            (Color::White, CastlingSide::Queen) => self.white_can_queen_castle,
            (Color::Black, CastlingSide::King) => self.black_can_king_castle,
            (Color::Black, CastlingSide::Queen) => self.black_can_queen_castle,
        }
    }
}

impl fmt::Display for Board {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn display_can_castle(value: bool) -> &'static str {
            if value {
                "v"
            } else {
                "-"
            }
        }
        write!(f, "  +-{}-------------{}-+\n", display_can_castle(self.black_can_queen_castle), display_can_castle(self.black_can_king_castle))?;
        for rank in 0..8 {
            write!(f, "{} | ", 8 - rank)?;
            for file in 0..8 {
                let index: usize = (7 - rank) * 8 + file;
                if self.en_passant_capturable.and_then(|i| Some(i == index)).unwrap_or(false) {
                    write!(f, "* ")?;
                } else {
                    write!(f, "{} ", self.squares[index])?;
                }
            }
            write!(f, "|\n")?;
        }
        write!(f, "  +-{}-------------{}-+\n", display_can_castle(self.white_can_queen_castle), display_can_castle(self.white_can_king_castle))?;
        write!(f, "    a b c d e f g h")?;
        write!(f, "\n")
    }
}
