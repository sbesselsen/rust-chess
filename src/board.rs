use std::fmt;
use std::str::FromStr;

use crate::parser::{parse_board, ParseBoardError};

#[derive(Debug, Copy, PartialEq, Clone, Hash)]
pub struct Board {
    squares: [Square; 64],
    en_passant_capturable: Option<usize>,
    white_can_king_castle: bool,
    white_can_queen_castle: bool,
    black_can_king_castle: bool,
    black_can_queen_castle: bool,
}

pub enum CastlingSide {
    King,
    Queen,
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
            }
            Square::Occupied(Piece(Color::White, Kind::Rook)) if target_index == 7 => {
                // White can't castle with its king's rook after it is captured.
                self.white_can_king_castle = false;
            }
            Square::Occupied(Piece(Color::Black, Kind::Rook)) if target_index == 56 => {
                // Black can't castle with its queen's rook after it is captured.
                self.black_can_queen_castle = false;
            }
            Square::Occupied(Piece(Color::Black, Kind::Rook)) if target_index == 63 => {
                // Black can't castle with its queen's rook after it is captured.
                self.black_can_king_castle = false;
            }
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
                    }
                    Square::Occupied(Piece(Color::Black, Kind::Pawn)) => {
                        self.squares[target_index + 8] = Square::Empty;
                    }
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
            }
            Square::Occupied(Piece(Color::White, Kind::King)) => {
                // White can't castle after moving its king.
                self.white_can_king_castle = false;
                self.white_can_queen_castle = false;
            }
            Square::Occupied(Piece(Color::Black, Kind::King)) => {
                // Black can't castle after moving its king.
                self.black_can_king_castle = false;
                self.black_can_queen_castle = false;
            }
            Square::Occupied(Piece(Color::White, Kind::Rook)) if index == 0 => {
                // White can't castle with its queen's rook after moving it.
                self.white_can_queen_castle = false;
            }
            Square::Occupied(Piece(Color::White, Kind::Rook)) if index == 7 => {
                // White can't castle with its king's rook after moving it.
                self.white_can_king_castle = false;
            }
            Square::Occupied(Piece(Color::Black, Kind::Rook)) if index == 56 => {
                // Black can't castle with its queen's rook after moving it.
                self.black_can_queen_castle = false;
            }
            Square::Occupied(Piece(Color::Black, Kind::Rook)) if index == 63 => {
                // Black can't castle with its queen's rook after moving it.
                self.black_can_king_castle = false;
            }
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

    pub fn squares_coordinates_iter(&self) -> impl Iterator<Item = (Coordinates, &Square)> + '_ {
        self.squares()
            .iter()
            .enumerate()
            .map(|(index, square)| (Coordinates::new_from_index(index).unwrap(), square))
    }

    pub fn set_castling_allowed(&mut self, color: Color, side: CastlingSide, allowed: bool) {
        match (color, side) {
            (Color::White, CastlingSide::King) => {
                self.white_can_king_castle = allowed;
            }
            (Color::White, CastlingSide::Queen) => {
                self.white_can_queen_castle = allowed;
            }
            (Color::Black, CastlingSide::King) => {
                self.black_can_king_castle = allowed;
            }
            (Color::Black, CastlingSide::Queen) => {
                self.black_can_queen_castle = allowed;
            }
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

    pub fn set_en_passant_capturable(&mut self, coordinates: Option<Coordinates>) {
        self.en_passant_capturable = coordinates.map(|c| c.index());
    }

    pub fn is_en_passant_capturable(&self, coordinates: Coordinates) -> bool {
        matches!(self.en_passant_capturable, Some(index) if index == coordinates.index())
    }
}

impl FromStr for Board {
    type Err = ParseBoardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_board(s)
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Board::new();
        board.setup();
        board
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
        write!(
            f,
            "  +-{}-------------{}-+\n",
            display_can_castle(self.black_can_queen_castle),
            display_can_castle(self.black_can_king_castle)
        )?;
        for rank in 0..8 {
            write!(f, "{} | ", 8 - rank)?;
            for file in 0..8 {
                let index: usize = (7 - rank) * 8 + file;
                if self
                    .en_passant_capturable
                    .and_then(|i| Some(i == index))
                    .unwrap_or(false)
                {
                    write!(f, "* ")?;
                } else {
                    write!(f, "{} ", self.squares[index])?;
                }
            }
            write!(f, "|\n")?;
        }
        write!(
            f,
            "  +-{}-------------{}-+\n",
            display_can_castle(self.white_can_queen_castle),
            display_can_castle(self.white_can_king_castle)
        )?;
        write!(f, "    a b c d e f g h")?;
        write!(f, "\n")
    }
}

#[derive(Debug, Copy, PartialEq, Clone, Hash)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Color {
        if *self == Color::White {
            Color::Black
        } else {
            Color::White
        }
    }
}

#[derive(Debug, Copy, PartialEq, Clone, Hash)]
pub enum Kind {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}

#[derive(Debug, Copy, PartialEq, Clone, Hash)]
pub struct Piece(pub Color, pub Kind);

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
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
            }
        )
    }
}

#[derive(Debug, Copy, PartialEq, Clone, Hash)]
pub enum Square {
    Occupied(Piece),
    Empty,
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
            &Square::Occupied(piece) => write!(f, "{}", piece),
        }
    }
}

#[derive(Debug, Copy, PartialEq, Clone, Hash)]
pub enum Rank {
    R1 = 0,
    R2 = 1,
    R3 = 2,
    R4 = 3,
    R5 = 4,
    R6 = 5,
    R7 = 6,
    R8 = 7,
}

const ALL_RANKS: [Rank; 8] = [
    Rank::R1,
    Rank::R2,
    Rank::R3,
    Rank::R4,
    Rank::R5,
    Rank::R6,
    Rank::R7,
    Rank::R8,
];

impl Rank {
    pub fn new_from_index(index: u8) -> Option<Rank> {
        if index > 7 {
            None
        } else {
            Some(ALL_RANKS[index as usize])
        }
    }

    pub fn index(&self) -> u8 {
        *self as u8
    }
}

impl fmt::Display for Rank {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.index() + 1)
    }
}

#[derive(Debug, Copy, PartialEq, Clone, Hash)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

const ALL_FILES: [File; 8] = [
    File::A,
    File::B,
    File::C,
    File::D,
    File::E,
    File::F,
    File::G,
    File::H,
];

impl File {
    pub fn new_from_index(index: u8) -> Option<File> {
        if index > 7 {
            None
        } else {
            Some(ALL_FILES[index as usize])
        }
    }

    pub fn index(&self) -> u8 {
        *self as u8
    }
}

const FILE_LETTERS: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

impl fmt::Display for File {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", FILE_LETTERS[self.index() as usize])
    }
}

#[derive(Debug, Copy, PartialEq, Clone, Hash)]
pub struct Coordinates {
    pub file: File,
    pub rank: Rank,
    index: usize,
}

impl Coordinates {
    pub fn new_from_index(index: usize) -> Option<Coordinates> {
        match index {
            0..=63 => {
                let index_u8 = index as u8;
                let file_u8 = index_u8 % 8;
                let file: File = File::new_from_index(file_u8).unwrap();
                let rank: Rank = Rank::new_from_index((index_u8 - file_u8) / 8).unwrap();
                Some(Coordinates { rank, file, index })
            }
            _ => None,
        }
    }

    pub fn new(file: File, rank: Rank) -> Coordinates {
        return Coordinates {
            rank,
            file,
            index: calculate_index(file.index(), rank.index()),
        };
    }

    pub fn offset(&self, file_offset: i8, rank_offset: i8) -> Option<Coordinates> {
        let new_file = self.file as i8 + file_offset;
        let new_rank = self.rank as i8 + rank_offset;
        if new_file >= 0 && new_rank >= 0 {
            if let Some(file) = File::new_from_index(new_file as u8) {
                if let Some(rank) = Rank::new_from_index(new_rank as u8) {
                    return Some(Coordinates::new(file, rank));
                }
            }
        }
        None
    }

    pub fn offsets_filter(&self, offsets: &[(i8, i8)]) -> Vec<Coordinates> {
        offsets
            .iter()
            .filter_map(|(file_offset, rank_offset)| self.offset(*file_offset, *rank_offset))
            .collect()
    }

    pub fn offsets_repeated(&self, file_offset: i8, rank_offset: i8) -> Vec<Coordinates> {
        (1..)
            .map(|multiple| self.offset(file_offset * multiple, rank_offset * multiple))
            .take_while(|opt| opt.is_some())
            .map(|opt| opt.unwrap())
            .collect()
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }

    #[allow(dead_code)]
    pub fn file(&self) -> File {
        self.file
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

fn calculate_index(file: u8, rank: u8) -> usize {
    (rank as usize) * 8 + (file as usize)
}

impl fmt::Display for Coordinates {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}
