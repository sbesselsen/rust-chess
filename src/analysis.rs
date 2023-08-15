use crate::board::{Board, CastlingSide, Color, Coordinates, File, Kind, Piece, Rank, Square};

pub const KNIGHT_OFFSETS: [(i8, i8); 8] = [
    (-2, -1),
    (-2, 1),
    (-1, -2),
    (-1, 2),
    (1, -2),
    (1, 2),
    (2, -1),
    (2, 1),
];

pub const BISHOP_OFFSETS: [(i8, i8); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

pub const KING_OFFSETS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn is_threatened_by(board: &Board, coordinates: Coordinates, color: Color) -> bool {
    // Check for threat by kings.
    for threat_coords in coordinates.offsets_filter(&KING_OFFSETS) {
        if board
            .get_square(threat_coords)
            .is_occupied_by(Piece(color, Kind::King))
        {
            // Threatened by a king.
            return true;
        }
    }
    // Check for threats on the diagonals.
    for (file_offset, rank_offset) in &BISHOP_OFFSETS {
        for threat_coords in coordinates
            .offsets_repeated(*file_offset, *rank_offset)
            .into_iter()
        {
            let square = board.get_square(threat_coords);
            if square.is_occupied_by(Piece(color, Kind::Bishop))
                || square.is_occupied_by(Piece(color, Kind::Queen))
            {
                // Threatened on the diagonal.
                return true;
            }
            if !square.is_empty() {
                // Blocked from here on out. Stop checking.
                break;
            }
        }
    }

    // Check for threats on the rank and file.
    for (file_offset, rank_offset) in &ROOK_OFFSETS {
        for threat_coords in coordinates
            .offsets_repeated(*file_offset, *rank_offset)
            .into_iter()
        {
            let square = board.get_square(threat_coords);
            if square.is_occupied_by(Piece(color, Kind::Rook))
                || square.is_occupied_by(Piece(color, Kind::Queen))
            {
                // Threatened on the diagonal.
                return true;
            }
            if !square.is_empty() {
                // Blocked from here on out. Stop checking.
                break;
            }
        }
    }

    // Check for threats from knights.
    for threat_coords in coordinates.offsets_filter(&KNIGHT_OFFSETS) {
        if board
            .get_square(threat_coords)
            .is_occupied_by(Piece(color, Kind::Knight))
        {
            // Threatened on the diagonal.
            return true;
        }
    }

    // Check for threats from pawns.
    let pawn_capture_rank_offset = if color == Color::White { -1 } else { 1 };
    let pawn_capture_offsets = [
        (-1, pawn_capture_rank_offset),
        (1, pawn_capture_rank_offset),
    ];
    for threat_coords in coordinates.offsets_filter(&pawn_capture_offsets) {
        if board
            .get_square(threat_coords)
            .is_occupied_by(Piece(color, Kind::Pawn))
        {
            // Threatened by a pawn.
            return true;
        }
    }

    false
}

pub fn is_checked(board: &Board, color: Color) -> bool {
    if let Some((king_index, _)) = board
        .squares()
        .iter()
        .enumerate()
        .find(|(_, square)| square.is_occupied_by(Piece(color, Kind::King)))
    {
        if let Some(king_coordinates) = Coordinates::new_from_index(king_index) {
            return is_threatened_by(&board, king_coordinates, color.opposite());
        }
    }
    false
}

// Assign a score to the board, with a positive score being good for white, a negative score being good for black.
pub fn score_board(board: &Board) -> i32 {
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
            Kind::King => 0,
        }
    }

    // Just sum the pieces score.
    let pieces_score = board
        .squares()
        .iter()
        .map(|square| match square {
            Square::Occupied(piece) => piece_score(*piece),
            _ => 0,
        })
        .sum();

    pieces_score
}

pub fn next_boards(board: &Board, color: Color) -> Vec<Board> {
    let mut boards: Vec<Board> = vec![];
    for (coordinates, square) in board.squares_coordinates_iter() {
        match *square {
            Square::Occupied(Piece(piece_color, Kind::Rook)) if piece_color == color => {
                add_rook_moves(&board, coordinates, color, &mut boards)
            }
            Square::Occupied(Piece(piece_color, Kind::Knight)) if piece_color == color => {
                add_knight_moves(&board, coordinates, color, &mut boards)
            }
            Square::Occupied(Piece(piece_color, Kind::Bishop)) if piece_color == color => {
                add_bishop_moves(&board, coordinates, color, &mut boards)
            }
            Square::Occupied(Piece(piece_color, Kind::Queen)) if piece_color == color => {
                add_queen_moves(&board, coordinates, color, &mut boards)
            }
            Square::Occupied(Piece(piece_color, Kind::King)) if piece_color == color => {
                add_king_moves(&board, coordinates, color, &mut boards)
            }
            Square::Occupied(Piece(piece_color, Kind::Pawn)) if piece_color == color => {
                add_pawn_moves(&board, coordinates, color, &mut boards)
            }
            _ => {}
        }
    }

    // Return the boards which are acceptable (not in check).
    boards.retain(|board| !is_checked(&board, color));
    boards
}

pub const ROOK_OFFSETS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn add_rook_moves(board: &Board, from: Coordinates, color: Color, boards: &mut Vec<Board>) {
    for (rank_offset, file_offset) in &ROOK_OFFSETS {
        for to in from.offsets_repeated(*file_offset, *rank_offset) {
            match board.get_square(to) {
                Square::Occupied(Piece(piece_color, _)) => {
                    if piece_color == color {
                        // Piece is blocked by another piece of its color. Stop right here.
                        break;
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

fn add_knight_moves(board: &Board, from: Coordinates, color: Color, boards: &mut Vec<Board>) {
    for to in from.offsets_filter(&KNIGHT_OFFSETS) {
        match board.get_square(to) {
            Square::Occupied(Piece(piece_color, _)) if piece_color == color => {}
            _ => {
                boards.push(board.clone_move_piece(from, to));
            }
        }
    }
}

fn add_bishop_moves(board: &Board, from: Coordinates, color: Color, boards: &mut Vec<Board>) {
    for (rank_offset, file_offset) in &BISHOP_OFFSETS {
        for to in from.offsets_repeated(*file_offset, *rank_offset) {
            match board.get_square(to) {
                Square::Occupied(Piece(piece_color, _)) => {
                    if piece_color == color {
                        // Piece is blocked by another piece of its color. Stop right here.
                        break;
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

fn add_king_moves(board: &Board, from: Coordinates, color: Color, boards: &mut Vec<Board>) {
    // Normal moves.
    for to in from.offsets_filter(&KING_OFFSETS) {
        match board.get_square(to) {
            Square::Occupied(Piece(piece_color, _)) if piece_color == color => {}
            _ => {
                boards.push(board.clone_move_piece(from, to));
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
    let rank: Rank = if color == Color::White {
        Rank::R1
    } else {
        Rank::R8
    };

    let opposite_color = color.opposite();
    if board.get_square(Coordinates::new(File::B, rank)).is_empty()
        && board.get_square(Coordinates::new(File::C, rank)).is_empty()
        && board.get_square(Coordinates::new(File::D, rank)).is_empty()
        && !is_threatened_by(&board, Coordinates::new(File::B, rank), opposite_color)
        && !is_threatened_by(&board, Coordinates::new(File::C, rank), opposite_color)
        && !is_threatened_by(&board, Coordinates::new(File::D, rank), opposite_color)
    {
        let mut new_board = board.clone_move_piece(
            Coordinates::new(File::E, rank),
            Coordinates::new(File::C, rank),
        );
        new_board.move_piece(
            Coordinates::new(File::A, rank),
            Coordinates::new(File::D, rank),
        );
        boards.push(new_board);
    }
}

fn add_kings_castle_move(board: &Board, color: Color, boards: &mut Vec<Board>) {
    let rank: Rank = if color == Color::White {
        Rank::R1
    } else {
        Rank::R8
    };

    let opposite_color = color.opposite();
    if board.get_square(Coordinates::new(File::F, rank)).is_empty()
        && board.get_square(Coordinates::new(File::G, rank)).is_empty()
        && !is_threatened_by(&board, Coordinates::new(File::F, rank), opposite_color)
        && !is_threatened_by(&board, Coordinates::new(File::G, rank), opposite_color)
    {
        let mut new_board = board.clone_move_piece(
            Coordinates::new(File::E, rank),
            Coordinates::new(File::G, rank),
        );
        new_board.move_piece(
            Coordinates::new(File::H, rank),
            Coordinates::new(File::F, rank),
        );
        boards.push(new_board);
    }
}

fn add_pawn_moves(board: &Board, from: Coordinates, color: Color, boards: &mut Vec<Board>) {
    let (move_direction, start_rank) = if color == Color::White {
        (1, Rank::R2)
    } else {
        (-1, Rank::R7)
    };

    if let Some(one_forward) = from.offset(0, 1 * move_direction) {
        if let Square::Empty = board.get_square(one_forward) {
            // Forward 1.
            boards.push(board.clone_move_piece(from, one_forward));

            if from.rank() == start_rank {
                if let Some(two_forward) = from.offset(0, 2 * move_direction) {
                    if let Square::Empty = board.get_square(two_forward) {
                        // Forward 2.
                        boards.push(board.clone_move_piece(from, two_forward));
                    }
                }
            }
        }
    }
    let capture_offsets = [(move_direction, -1), (move_direction, 1)];
    for to in from.offsets_filter(&capture_offsets) {
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
