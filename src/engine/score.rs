use crate::board::{ Board, Color, Kind, Piece, Square };

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
