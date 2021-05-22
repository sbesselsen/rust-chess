use crate::board::{ Board, CastlingSide, Color, Coordinates, Kind, Piece, Square };
use crate::engine::analysis::{ is_checked, is_threatened_by };

pub fn next_boards(board: &Board, color: Color) -> Vec<Board> {
    let mut boards: Vec<Board> = vec![];
    for (coordinates, square) in board.squares_coordinates_iter() {
        match *square {
            Square::Occupied(Piece(piece_color, Kind::Rook)) if piece_color == color => {
                add_rook_moves(&board, coordinates, color, &mut boards)
            },
            Square::Occupied(Piece(piece_color, Kind::Knight)) if piece_color == color => {
                add_knight_moves(&board, coordinates, color, &mut boards)
            },
            Square::Occupied(Piece(piece_color, Kind::Bishop)) if piece_color == color => {
                add_bishop_moves(&board, coordinates, color, &mut boards)
            },
            Square::Occupied(Piece(piece_color, Kind::Queen)) if piece_color == color => {
                add_queen_moves(&board, coordinates, color, &mut boards)
            },
            Square::Occupied(Piece(piece_color, Kind::King)) if piece_color == color => {
                add_king_moves(&board, coordinates, color, &mut boards)
            },
            Square::Occupied(Piece(piece_color, Kind::Pawn)) if piece_color == color => {
                add_pawn_moves(&board, coordinates, color, &mut boards)
            },
            _ => {}
        }
    }

    // Return the boards which are acceptable (not in check).
    boards.into_iter().filter(|board| !is_checked(&board, color)).collect()
}

pub const ROOK_OFFSETS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn add_rook_moves(board: &Board, from: Coordinates, color: Color, boards: &mut Vec<Board>) {
    for (rank_offset, file_offset) in &ROOK_OFFSETS {
        for to in from.offsets_repeated(*rank_offset, *file_offset).into_iter() {
            match board.get_square(to) {
                Square::Occupied(Piece(piece_color, _)) => {
                    if piece_color == color {
                        // Piece is blocked by another piece of its color. Stop right here.
                        break
                    } else {
                        // This is a capture. Stop after this move.
                        boards.push(board.clone_move_piece(from, to));
                        break;
                    }
                }
                _ => {
                    boards.push(board.clone_move_piece(from, to));
                }
            }
        }
    }
}

pub const KNIGHT_OFFSETS: [(i8, i8); 8] = [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];

fn add_knight_moves(board: &Board, from: Coordinates, color: Color, boards: &mut Vec<Board>) {
    for (rank_offset, file_offset) in &KNIGHT_OFFSETS {
        if let Some(to) = from.offset(*rank_offset, *file_offset) {
            match board.get_square(to) {
                Square::Occupied(Piece(piece_color, _)) if piece_color == color => {},
                _ => {
                    boards.push(board.clone_move_piece(from, to));
                }
            }
        }
    }
}

pub const BISHOP_OFFSETS: [(i8, i8); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

fn add_bishop_moves(board: &Board, from: Coordinates, color: Color, boards: &mut Vec<Board>) {
    for (rank_offset, file_offset) in &BISHOP_OFFSETS {
        for to in from.offsets_repeated(*rank_offset, *file_offset).into_iter() {
            match board.get_square(to) {
                Square::Occupied(Piece(piece_color, _)) => {
                    if piece_color == color {
                        // Piece is blocked by another piece of its color. Stop right here.
                        break
                    } else {
                        // This is a capture. Stop after this move.
                        boards.push(board.clone_move_piece(from, to));
                        break;
                    }
                }
                _ => {
                    boards.push(board.clone_move_piece(from, to));
                }
            }
        }
    }
}

fn add_queen_moves(board: &Board, from: Coordinates, color: Color, boards: &mut Vec<Board>) {
    add_rook_moves(&board, from, color, boards);
    add_bishop_moves(&board, from, color, boards);
}

pub const KING_OFFSETS: [(i8, i8); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

fn add_king_moves(board: &Board, from: Coordinates, color: Color, boards: &mut Vec<Board>) {
    // Normal moves.
    for (rank_offset, file_offset) in &KING_OFFSETS {
        if let Some(to) = from.offset(*rank_offset, *file_offset) {
            match board.get_square(to) {
                Square::Occupied(Piece(piece_color, _)) if piece_color == color => {},
                _ => {
                    boards.push(board.clone_move_piece(from, to));
                }
            }
        }
    }

    // Castling.
    if board.is_castling_allowed(color, CastlingSide::Queen) {
        add_queens_castle_move(&board, color, boards);
    }
    if board.is_castling_allowed(color, CastlingSide::King) {
        add_kings_castle_move(&board, color, boards);
    }
}

fn add_queens_castle_move(board: &Board, color: Color, boards: &mut Vec<Board>) {
    let rank: u8 = if color == Color::White { 0 } else { 7 };

    let opposite_color = color.opposite();
    if board.get_square(Coordinates::new(rank, 1).unwrap()).is_empty()
            && board.get_square(Coordinates::new(rank, 2).unwrap()).is_empty()
            && board.get_square(Coordinates::new(rank, 3).unwrap()).is_empty()
            && !is_threatened_by(&board, Coordinates::new(rank, 1).unwrap(), opposite_color)
            && !is_threatened_by(&board, Coordinates::new(rank, 2).unwrap(), opposite_color)
            && !is_threatened_by(&board, Coordinates::new(rank, 3).unwrap(), opposite_color) {
        let mut new_board = board.clone_move_piece(Coordinates::new(rank, 4).unwrap(), Coordinates::new(rank, 2).unwrap());
        new_board.move_piece(Coordinates::new(rank, 0).unwrap(), Coordinates::new(rank, 3).unwrap());
        boards.push(new_board);
    }
}

fn add_kings_castle_move(board: &Board, color: Color, boards: &mut Vec<Board>) {
    let rank: u8 = if color == Color::White { 0 } else { 7 };

    let opposite_color = color.opposite();
    if board.get_square(Coordinates::new(rank, 5).unwrap()).is_empty()
            && board.get_square(Coordinates::new(rank, 6).unwrap()).is_empty()
            && !is_threatened_by(&board, Coordinates::new(rank, 5).unwrap(), opposite_color)
            && !is_threatened_by(&board, Coordinates::new(rank, 6).unwrap(), opposite_color) {
        let mut new_board = board.clone_move_piece(Coordinates::new(rank, 4).unwrap(), Coordinates::new(rank, 6).unwrap());
        new_board.move_piece(Coordinates::new(rank, 7).unwrap(), Coordinates::new(rank, 5).unwrap());
        boards.push(new_board);
    }
}

fn add_pawn_moves(board: &Board, from: Coordinates, color: Color, boards: &mut Vec<Board>) {
    let move_direction: i8 = match color {
        Color::White => 1,
        _ => -1
    };
    let start_rank: u8 = match color {
        Color::White => 1,
        _ => 6
    };
    if let Some(one_forward) = from.offset(1 * move_direction, 0) {
        if let Square::Empty = board.get_square(one_forward) {
            // Forward 1.
            boards.push(board.clone_move_piece(from, one_forward));

            if from.rank() == start_rank {
                if let Some(two_forward) = from.offset(2 * move_direction, 0) {
                    if let Square::Empty = board.get_square(two_forward) {
                        // Forward 2.
                        boards.push(board.clone_move_piece(from, two_forward));
                    }
                }
            }
        }
    }
    let capture_offsets = [(move_direction, -1), (move_direction, 1)];
    for (rank_offset, file_offset) in &capture_offsets {
        if let Some(to) = from.offset(*rank_offset, *file_offset) {
            if let Square::Occupied(Piece(piece_color, _)) = board.get_square(to) {
                if piece_color != color {
                    // Can capture this piece.
                    boards.push(board.clone_move_piece(from, to));
                }
            }

            if board.is_en_passant_capturable(to) {
                boards.push(board.clone_move_piece(from, to));
            }
        }
    }
}
