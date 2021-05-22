mod board;

use board::{ Board, CastlingSide, Color, Coordinates, Kind, Piece, Square };

fn main() {
    test_all();
}

fn test_all() {
    for _ in 0..10000 {
        test_opening_moves();
        test_en_passant();
        test_threats();
        test_castle();
        test_check();
        test_score();
    }
    test_display_board();
    println!("OK");
}

fn test_display_board() {
    let mut board = Board::new();
    board.setup();
    println!("{}", board);
}

fn test_opening_moves() {
    let mut board = Board::new();
    board.setup();
    assert_eq!(20, next_boards(&board, Color::White).len());
}

fn test_en_passant() {
    let mut board = Board::new();
    board.set_square(Coordinates::new(1, 0).unwrap(), Square::Occupied(Piece(Color::White, Kind::Pawn)));
    board.set_square(Coordinates::new(3, 1).unwrap(), Square::Occupied(Piece(Color::Black, Kind::Pawn)));

    assert_eq!(true, next_boards(&board, Color::White).into_iter().flat_map(|next_board| next_boards(&next_board, Color::Black))
        .filter(|board| board.get_square(Coordinates::new(2, 0).unwrap()).is_occupied_by(Piece(Color::Black, Kind::Pawn)))
        .any(|_| true));
}

fn test_threats() {
    let mut board = Board::new();

    // Check pawns.
    board.set_square(Coordinates::new(1, 0).unwrap(), Square::Occupied(Piece(Color::White, Kind::Pawn)));
    assert_eq!(true, is_threatened_by(&board, Coordinates::new(2, 1).unwrap(), Color::White));
    board.set_square(Coordinates::new(5, 5).unwrap(), Square::Occupied(Piece(Color::Black, Kind::Pawn)));
    assert_eq!(true, is_threatened_by(&board, Coordinates::new(4, 6).unwrap(), Color::Black));
    assert_eq!(false, is_threatened_by(&board, Coordinates::new(4, 6).unwrap(), Color::White));

    // Check bishops and queens.
    let mut board = Board::new();
    board.set_square(Coordinates::new(0, 0).unwrap(), Square::Occupied(Piece(Color::White, Kind::Bishop)));
    assert_eq!(true, is_threatened_by(&board, Coordinates::new(4, 4).unwrap(), Color::White));
    assert_eq!(false, is_threatened_by(&board, Coordinates::new(4, 5).unwrap(), Color::White));
    board.set_square(Coordinates::new(2, 2).unwrap(), Square::Occupied(Piece(Color::White, Kind::Pawn)));
    assert_eq!(false, is_threatened_by(&board, Coordinates::new(4, 4).unwrap(), Color::White));
}

fn test_castle() {
    // White king's castle.
    let mut board = Board::new();
    board.set_castling_allowed(Color::White, CastlingSide::King, true);
    board.set_square(Coordinates::new(0, 4).unwrap(), Square::Occupied(Piece(Color::White, Kind::King)));
    board.set_square(Coordinates::new(0, 7).unwrap(), Square::Occupied(Piece(Color::White, Kind::Rook)));
    assert_eq!(true, next_boards(&board, Color::White).into_iter()
        .filter(|board| board.get_square(Coordinates::new(0, 6).unwrap()).is_occupied_by(Piece(Color::White, Kind::King)))
        .filter(|board| board.get_square(Coordinates::new(0, 5).unwrap()).is_occupied_by(Piece(Color::White, Kind::Rook)))
        .filter(|board| !board.is_castling_allowed(Color::White, CastlingSide::King))
        .filter(|board| !board.is_castling_allowed(Color::White, CastlingSide::Queen))
        .any(|_| true));

    // But not if threatened.
    board.set_square(Coordinates::new(7, 6).unwrap(), Square::Occupied(Piece(Color::Black, Kind::Rook)));
    assert_eq!(false, next_boards(&board, Color::White).into_iter()
        .filter(|board| board.get_square(Coordinates::new(0, 6).unwrap()).is_occupied_by(Piece(Color::White, Kind::King)))
        .filter(|board| board.get_square(Coordinates::new(0, 5).unwrap()).is_occupied_by(Piece(Color::White, Kind::Rook)))
        .filter(|board| !board.is_castling_allowed(Color::White, CastlingSide::King))
        .filter(|board| !board.is_castling_allowed(Color::White, CastlingSide::Queen))
        .any(|_| true));

    // Black queen's castle.
    let mut board = Board::new();
    board.set_castling_allowed(Color::Black, CastlingSide::Queen, true);
    board.set_square(Coordinates::new(7, 4).unwrap(), Square::Occupied(Piece(Color::Black, Kind::King)));
    board.set_square(Coordinates::new(7, 0).unwrap(), Square::Occupied(Piece(Color::Black, Kind::Rook)));

    assert_eq!(true, next_boards(&board, Color::Black).into_iter()
        .filter(|board| board.get_square(Coordinates::new(7, 2).unwrap()).is_occupied_by(Piece(Color::Black, Kind::King)))
        .filter(|board| board.get_square(Coordinates::new(7, 3).unwrap()).is_occupied_by(Piece(Color::Black, Kind::Rook)))
        .filter(|board| !board.is_castling_allowed(Color::Black, CastlingSide::King))
        .filter(|board| !board.is_castling_allowed(Color::Black, CastlingSide::Queen))
        .any(|_| true));
}

fn test_check() {
    // Test that we cannot move the king into check.
    let mut board = Board::new();
    board.set_square(Coordinates::new(3, 4).unwrap(), Square::Occupied(Piece(Color::Black, Kind::King)));
    board.set_square(Coordinates::new(7, 3).unwrap(), Square::Occupied(Piece(Color::White, Kind::Rook)));
    board.set_square(Coordinates::new(0, 5).unwrap(), Square::Occupied(Piece(Color::White, Kind::Rook)));

    assert_eq!(2, next_boards(&board, Color::Black).len());

    // Test that we cannot expose the king to check by moving another piece.
    board.set_square(Coordinates::new(7, 0).unwrap(), Square::Occupied(Piece(Color::White, Kind::Bishop)));
    board.set_square(Coordinates::new(6, 1).unwrap(), Square::Occupied(Piece(Color::Black, Kind::Rook)));

    assert_eq!(2, next_boards(&board, Color::Black).len());
}

fn test_score() {
    let mut board = Board::new();
    assert_eq!(0, score_board(&board));

    board.setup();
    assert_eq!(0, score_board(&board));

    board.set_square(Coordinates::new(0, 0).unwrap(), Square::Empty);
    assert_eq!(true, score_board(&board) < 0);

    board.set_square(Coordinates::new(7, 0).unwrap(), Square::Empty);
    assert_eq!(0, score_board(&board));

    board.set_square(Coordinates::new(6, 0).unwrap(), Square::Empty);
    assert_eq!(true, score_board(&board) > 0);
}

fn next_boards(board: &Board, color: Color) -> Vec<Board> {
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

const ROOK_OFFSETS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

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

const KNIGHT_OFFSETS: [(i8, i8); 8] = [(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];

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

const BISHOP_OFFSETS: [(i8, i8); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

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

const KING_OFFSETS: [(i8, i8); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

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

            // TODO: make en_passant_capturable private
            if let Some(en_passant_index) = board.en_passant_capturable {
                if to.index() == en_passant_index {
                    boards.push(board.clone_move_piece(from, to));
                }
            }
        }
    }
}

fn is_threatened_by(board: &Board, coordinates: Coordinates, color: Color) -> bool {
    // Check for threat by kings.
    for (rank_offset, file_offset) in &KING_OFFSETS {
        if let Some(threat_coords) = coordinates.offset(*rank_offset, *file_offset) {
            if board.get_square(threat_coords).is_occupied_by(Piece(color, Kind::King)) {
                // Threatened by a king.
                return true
            }
        }
    }

    // Check for threats on the diagonals.
    for (rank_offset, file_offset) in &BISHOP_OFFSETS {
        for threat_coords in coordinates.offsets_repeated(*rank_offset, *file_offset).into_iter() {
            let square = board.get_square(threat_coords);
            if square.is_occupied_by(Piece(color, Kind::Bishop)) || square.is_occupied_by(Piece(color, Kind::Queen)) {
                // Threatened on the diagonal.
                return true
            }
            if !square.is_empty() {
                // Blocked from here on out. Stop checking.
                break
            }
        }
    }

    // Check for threats on the rank and file.
    for (rank_offset, file_offset) in &ROOK_OFFSETS {
        for threat_coords in coordinates.offsets_repeated(*rank_offset, *file_offset).into_iter() {
            let square = board.get_square(threat_coords);
            if square.is_occupied_by(Piece(color, Kind::Rook)) || square.is_occupied_by(Piece(color, Kind::Queen)) {
                // Threatened on the diagonal.
                return true
            }
            if !square.is_empty() {
                // Blocked from here on out. Stop checking.
                break
            }
        }
    }

    // Check for threats from knights.
    for (rank_offset, file_offset) in &KNIGHT_OFFSETS {
        if let Some(threat_coords) = coordinates.offset(*rank_offset, *file_offset) {
            if board.get_square(threat_coords).is_occupied_by(Piece(color, Kind::Knight)) {
                // Threatened on the diagonal.
                return true
            }
        }
    }

    // Check for threats from pawns.
    let rank_offset = if color == Color::White { -1 } else { 1 };
    for file_offset in &[1, -1] {
        if let Some(threat_coords) = coordinates.offset(rank_offset, *file_offset) {
            if board.get_square(threat_coords).is_occupied_by(Piece(color, Kind::Pawn)) {
                // Threatened by a pawn.
                return true
            }
        }
    }

    false
}

fn is_checked(board: &Board, color: Color) -> bool {
    if let Some((king_index, _)) = board.squares().iter().enumerate().find(|(_, square)| square.is_occupied_by(Piece(color, Kind::King))) {
        if let Some(king_coordinates) = Coordinates::new_from_index(king_index) {
            return is_threatened_by(&board, king_coordinates, color.opposite())
        }
    }
    false
}

// Assign a score to the board, with a positive score being good for white, a negative score being good for black.
fn score_board(board: &Board) -> i32 {
    fn piece_score(piece: Piece) -> i32 {
        kind_score(piece.1) * (if piece.0 == Color::Black { -1 } else { 1 })
    }
    fn kind_score(kind: Kind) -> i32 {
        match kind {
            Kind::Pawn => 1,
            Kind::Knight => 3,
            Kind::Bishop => 3,
            Kind::Rook => 5,
            Kind::Queen => 9,
            Kind::King => 0
        }
    }

    // Just sum the pieces score.
    let pieces_score = board.squares().iter().map(|square| match square {
        Square::Occupied(piece) => piece_score(*piece),
        _ => 0
    }).sum();

    pieces_score
}
