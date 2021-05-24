use crate::board::{ Board, Color, Coordinates, Kind, Piece };
use crate::engine::moves::{ KING_OFFSETS, BISHOP_OFFSETS, ROOK_OFFSETS, KNIGHT_OFFSETS };

pub fn is_threatened_by(board: &Board, coordinates: Coordinates, color: Color) -> bool {
    // Check for threat by kings.
    for threat_coords in coordinates.offsets_filter(&KING_OFFSETS) {
        if board.get_square(threat_coords).is_occupied_by(Piece(color, Kind::King)) {
            // Threatened by a king.
            return true
        }
    }
    // Check for threats on the diagonals.
    for (file_offset, rank_offset) in &BISHOP_OFFSETS {
        for threat_coords in coordinates.offsets_repeated(*file_offset, *rank_offset).into_iter() {
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
    for (file_offset, rank_offset) in &ROOK_OFFSETS {
        for threat_coords in coordinates.offsets_repeated(*file_offset, *rank_offset).into_iter() {
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
    for threat_coords in coordinates.offsets_filter(&KNIGHT_OFFSETS) {
        if board.get_square(threat_coords).is_occupied_by(Piece(color, Kind::Knight)) {
            // Threatened on the diagonal.
            return true
        }
    }

    // Check for threats from pawns.
    let pawn_capture_rank_offset = if color == Color::White { -1 } else { 1 };
    let pawn_capture_offsets = [(-1, pawn_capture_rank_offset), (1, pawn_capture_rank_offset)];
    for threat_coords in coordinates.offsets_filter(&pawn_capture_offsets) {
        if board.get_square(threat_coords).is_occupied_by(Piece(color, Kind::Pawn)) {
            // Threatened by a pawn.
            return true
        }
    }

    false
}

pub fn is_checked(board: &Board, color: Color) -> bool {
    if let Some((king_index, _)) = board.squares().iter().enumerate().find(|(_, square)| square.is_occupied_by(Piece(color, Kind::King))) {
        if let Some(king_coordinates) = Coordinates::new_from_index(king_index) {
            return is_threatened_by(&board, king_coordinates, color.opposite())
        }
    }
    false
}
