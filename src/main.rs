use std::fmt;

#[derive(Debug,Copy,PartialEq,Clone,Hash)]
enum Color {
    White,
    Black
}

impl Color {
    fn opposite(&self) -> Color {
        if *self == Color::White { Color::Black } else { Color::White }
    }
}

#[derive(Debug,Copy,PartialEq,Clone,Hash)]
enum Kind {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn
}

#[derive(Debug,Copy,PartialEq,Clone,Hash)]
struct Piece(Color, Kind);

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

#[derive(Debug,Copy,PartialEq,Clone,Hash)]
enum Square {
    Occupied(Piece),
    Empty
}

impl Square {
    fn is_empty(&self) -> bool {
        matches!(self, Square::Empty)
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

#[derive(Debug,Copy,PartialEq,Clone,Hash)]
struct Position {
    rank: u8,
    file: u8
}

impl Position {
    fn from_index(index: usize) -> Option<Position> {
        match index {
            0..=63 => {
                let index_u8 = index as u8;
                let file: u8 = index_u8 % 8;
                let rank: u8 = (index_u8 - file) / 8;
                Some(Position { rank, file })
            },
            _ => None
        }
    }

    fn offset(&self, rank: i8, file: i8) -> Option<Position> {
        let target_rank = (self.rank as i8) + rank;
        let target_file = (self.file as i8) + file;
        match (target_rank, target_file) {
            (0..=7, 0..=7) => Some(Position { rank: target_rank as u8, file: target_file as u8 }),
            _ => None
        }
    }

    fn to_index(&self) -> usize {
        (self.rank as usize) * 8 + (self.file as usize)
    }
}

const FILE_LETTERS: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

impl fmt::Display for Position {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", FILE_LETTERS[self.file as usize], self.rank + 1)
    }
}

#[derive(Debug,Copy,PartialEq,Clone,Hash)]
struct Board {
    squares: [Square; 64],
    en_passant_capturable: Option<usize>,
    white_can_king_castle: bool,
    white_can_queen_castle: bool,
    black_can_king_castle: bool,
    black_can_queen_castle: bool,
}

impl Board {
    fn create_empty() -> Board {
        Board {
            squares: [Square::Empty; 64],
            en_passant_capturable: None,
            white_can_king_castle: false,
            white_can_queen_castle: false,
            black_can_king_castle: false,
            black_can_queen_castle: false,
        }
    }

    fn setup(&mut self) {
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

    fn move_piece(&mut self, index: usize, target_index: usize) {
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

    fn clone_move_piece(&self, index: usize, target_index: usize) -> Board {
        let mut clone = self.clone();
        clone.move_piece(index, target_index);
        clone
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

fn main() {
    let mut board = Board::create_empty();
    board.setup();

    // println!("Start: \n{}", board);
    let next_boards_1 = next_boards(&board, Color::White);

    // See if we can get some rook moves here.
    // let next_boards_2 = next_boards(&next_boards_1[0], Color::White);
    for next_board in next_boards_1 {
        println!("{}", next_board);
    }
}

fn next_boards(board: &Board, color: Color) -> Vec<Board> {
    let mut boards: Vec<Board> = vec![];
    for (index, square) in board.squares.iter().enumerate() {
        match *square {
            Square::Occupied(Piece(piece_color, Kind::Rook)) if piece_color == color => {
                add_rook_moves(board, index, color, &mut boards)
            },
            Square::Occupied(Piece(piece_color, Kind::Knight)) if piece_color == color => {
                add_knight_moves(board, index, color, &mut boards)
            },
            Square::Occupied(Piece(piece_color, Kind::Bishop)) if piece_color == color => {
                add_bishop_moves(board, index, color, &mut boards)
            },
            Square::Occupied(Piece(piece_color, Kind::Queen)) if piece_color == color => {
                add_queen_moves(board, index, color, &mut boards)
            },
            Square::Occupied(Piece(piece_color, Kind::King)) if piece_color == color => {
                add_king_moves(board, index, color, &mut boards)
            },
            Square::Occupied(Piece(piece_color, Kind::Pawn)) if piece_color == color => {
                add_pawn_moves(board, index, color, &mut boards)
            },
            _ => {}
        }
    }
    // TODO: filter the moves based on check.
    boards
}

const ROOK_OFFSETS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn add_rook_moves(board: &Board, index: usize, color: Color, boards: &mut Vec<Board>) {
    for (rank_offset, file_offset) in &ROOK_OFFSETS {
        for target_index in index_board_offset_iterate(index, *rank_offset, *file_offset).into_iter() {
            match board.squares[target_index] {
                Square::Occupied(Piece(piece_color, _)) => {
                    if piece_color == color {
                        // Piece is blocked by another piece of its color. Stop right here.
                        break
                    } else {
                        // This is a capture. Stop after this move.
                        boards.push(board.clone_move_piece(index, target_index));
                        break;
                    }
                }
                _ => {
                    boards.push(board.clone_move_piece(index, target_index));
                }
            }
        }
    }
}

const KNIGHT_OFFSETS: [(i8, i8); 8] = [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];

fn add_knight_moves(board: &Board, index: usize, color: Color, boards: &mut Vec<Board>) {
    for (rank_offset, file_offset) in &KNIGHT_OFFSETS {
        if let Some(target_index) = index_board_offset(index, *rank_offset, *file_offset) {
            match board.squares[target_index] {
                Square::Occupied(Piece(piece_color, _)) if piece_color == color => {},
                _ => {
                    boards.push(board.clone_move_piece(index, target_index));
                }
            }
        }
    }
}

const BISHOP_OFFSETS: [(i8, i8); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

fn add_bishop_moves(board: &Board, index: usize, color: Color, boards: &mut Vec<Board>) {
    for (rank_offset, file_offset) in &BISHOP_OFFSETS {
        for target_index in index_board_offset_iterate(index, *rank_offset, *file_offset).into_iter() {
            match board.squares[target_index] {
                Square::Occupied(Piece(piece_color, _)) => {
                    if piece_color == color {
                        // Piece is blocked by another piece of its color. Stop right here.
                        break
                    } else {
                        // This is a capture. Stop after this move.
                        boards.push(board.clone_move_piece(index, target_index));
                        break;
                    }
                }
                _ => {
                    boards.push(board.clone_move_piece(index, target_index));
                }
            }
        }
    }
}

fn add_queen_moves(board: &Board, index: usize, color: Color, boards: &mut Vec<Board>) {
    add_rook_moves(board, index, color, boards);
    add_bishop_moves(board, index, color, boards);
}

const KING_OFFSETS: [(i8, i8); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

fn add_king_moves(board: &Board, index: usize, color: Color, boards: &mut Vec<Board>) {
    // Normal moves.
    for (rank_offset, file_offset) in &KING_OFFSETS {
        if let Some(target_index) = index_board_offset(index, *rank_offset, *file_offset) {
            match board.squares[target_index] {
                Square::Occupied(Piece(piece_color, _)) if piece_color == color => {},
                _ => {
                    boards.push(board.clone_move_piece(index, target_index));
                }
            }
        }
    }

    // Castling.
    if color == Color::White {
        if board.white_can_queen_castle {
            add_queens_castle_move(board, color, boards);
        }
        if board.white_can_king_castle {
            add_kings_castle_move(board, color, boards);
        }
    } else if color == Color::Black {
        if board.black_can_queen_castle {
            add_queens_castle_move(board, color, boards);
        }
        if board.black_can_king_castle {
            add_kings_castle_move(board, color, boards);
        }
    }
}

fn add_queens_castle_move(board: &Board, color: Color, boards: &mut Vec<Board>) {
    let index_offset: usize = if color == Color::White { 0 }  else { 56 };
    let opposite_color = color.opposite();
    if board.squares[index_offset + 1].is_empty()
            && board.squares[index_offset + 2].is_empty()
            && board.squares[index_offset + 3].is_empty()
            && !is_threatened_by(board, index_offset + 1, opposite_color)
            && !is_threatened_by(board, index_offset + 2, opposite_color)
            && !is_threatened_by(board, index_offset + 3, opposite_color) {
        let mut new_board = board.clone_move_piece(index_offset + 4, index_offset + 2);
        new_board.move_piece(index_offset, index_offset + 3);
        boards.push(new_board);
    }
}

fn add_kings_castle_move(board: &Board, color: Color, boards: &mut Vec<Board>) {
    let index_offset: usize = if color == Color::White { 0 }  else { 56 };
    let opposite_color = color.opposite();
    if board.squares[index_offset + 5].is_empty()
            && board.squares[index_offset + 6].is_empty()
            && !is_threatened_by(board, index_offset + 5, opposite_color)
            && !is_threatened_by(board, index_offset + 6, opposite_color) {
        let mut new_board = board.clone_move_piece(index_offset + 4, index_offset + 6);
        new_board.move_piece(index_offset + 7, index_offset + 5);
        boards.push(new_board);
    }
}

fn add_pawn_moves(board: &Board, index: usize, color: Color, boards: &mut Vec<Board>) {
    if let Some(position) = Position::from_index(index) {
        let move_direction: i8 = match color {
            Color::White => 1,
            _ => -1
        };
        let start_rank: u8 = match color {
            Color::White => 1,
            _ => 6
        };
        if let Some(one_forward_index) = index_board_offset(index, 1 * move_direction, 0) {
            if let Square::Empty = board.squares[one_forward_index] {
                // Forward 1.
                boards.push(board.clone_move_piece(index, one_forward_index));

                if position.rank == start_rank {
                    if let Some(two_forward_index) = index_board_offset(index, 2 * move_direction, 0) {
                        if let Square::Empty = board.squares[two_forward_index] {
                            // Forward 2.
                            boards.push(board.clone_move_piece(index, two_forward_index));
                        }
                    }
                }
            }
        }
        // TODO: capture
        // TODO: en passant capture
    }
}

fn index_board_offset(index: usize, rank_offset: i8, file_offset: i8) -> Option<usize> {
    if let Some(position) = Position::from_index(index) {
        if let Some(target_position) = position.offset(rank_offset, file_offset) {
            return Some(target_position.to_index())
        }
    }
    None
}

// Iterate indexes with the same offset until we hit the edge of the board or a piece we can capture.
fn index_board_offset_iterate(index: usize, rank_offset: i8, file_offset: i8) -> Vec<usize> {
    let mut indices = vec![];
    if let Some(position) = Position::from_index(index) {
        for multiple in 1.. {
            if let Some(target_position) = position.offset(rank_offset * multiple, file_offset * multiple) {
                let target_index = target_position.to_index();
                indices.push(target_index);
            } else {
                break
            }
        }
    }
    indices
}

fn is_threatened_by(board: &Board, index: usize, color: Color) -> bool {
    false // TODO TODO TODO
}
