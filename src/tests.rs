use crate::board::{ Board, CastlingSide, Color, Coordinates, File, Kind, Piece, Rank, Square };
use crate::engine::{ is_threatened_by, next_boards, score_board, scored_next_boards };

pub fn test_all() {
    test_board_parser();
    test_opening_moves();
    test_en_passant();
    test_threats();
    test_castle();
    test_check();
    test_score();
    test_display_board();
    test_score_deep();
    println!("OK");
}

fn test_display_board() {
    let mut board = Board::new();
    board.setup();

    let board_data = "
  +-v-------------v-+
8 | ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ |
7 | ♟︎ ♟︎ ♟︎ ♟︎ ♟︎ ♟︎ ♟︎ ♟︎ |
6 |                 |
5 |                 |
4 |                 |
3 |                 |
2 | ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ |
1 | ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ |
  +-v-------------v-+
    a b c d e f g h";

    assert_eq!(String::from(format!("{}", board).trim()), String::from(board_data.trim()));
}

fn test_opening_moves() {
    let mut board = Board::new();
    board.setup();
    assert_eq!(20, next_boards(&board, Color::White).len());
}

fn test_en_passant() {
    // TODO: replace all this code with a simple board diagram, like above.
    // Just print it out and copy it.
    let board = Board::parse_str("
      +-----------------+
    8 |                 |
    7 |                 |
    6 |                 |
    5 |                 |
    4 |   ♟︎             |
    3 |                 |
    2 | ♙               |
    1 |                 |
      +-----------------+
        a b c d e f g h").unwrap();

    assert_eq!(true, next_boards(&board, Color::White).into_iter().flat_map(|next_board| next_boards(&next_board, Color::Black))
        .filter(|board| board.get_square(Coordinates::new(File::A, Rank::R3)).is_occupied_by(Piece(Color::Black, Kind::Pawn)))
        .any(|_| true));
}

fn test_threats() {
    // Check pawns.
    let board = Board::parse_str("
      +-----------------+
    8 |                 |
    7 |                 |
    6 |                 |
    5 |                 |
    4 |                 |
    3 |                 |
    2 | ♙               |
    1 |                 |
      +-----------------+
        a b c d e f g h").unwrap();
    assert_eq!(true, is_threatened_by(&board, Coordinates::new(File::B, Rank::R3), Color::White));

    let board = Board::parse_str("
      +-----------------+
    8 |                 |
    7 |                 |
    6 |           ♟︎     |
    5 |                 |
    4 |                 |
    3 |                 |
    2 | ♙               |
    1 |                 |
      +-----------------+
        a b c d e f g h").unwrap();
    assert_eq!(true, is_threatened_by(&board, Coordinates::new(File::G, Rank::R5), Color::Black));
    assert_eq!(false, is_threatened_by(&board, Coordinates::new(File::G, Rank::R5), Color::White));

    // Check bishops and queens.
    let board = Board::parse_str("
      +-----------------+
    8 |                 |
    7 |                 |
    6 |                 |
    5 |                 |
    4 |                 |
    3 |                 |
    2 |                 |
    1 | ♗               |
      +-----------------+
        a b c d e f g h").unwrap();
    assert_eq!(true, is_threatened_by(&board, Coordinates::new(File::E, Rank::R5), Color::White));
    assert_eq!(false, is_threatened_by(&board, Coordinates::new(File::F, Rank::R5), Color::White));

    let board = Board::parse_str("
      +-----------------+
    8 |                 |
    7 |                 |
    6 |                 |
    5 |                 |
    4 |                 |
    3 |     ♙           |
    2 |                 |
    1 | ♗               |
      +-----------------+
        a b c d e f g h").unwrap();
    assert_eq!(false, is_threatened_by(&board, Coordinates::new(File::E, Rank::R5), Color::White));
}

fn test_castle() {
    // White king's castle.
    let board = Board::parse_str("
      +-----------------+
    8 |                 |
    7 |                 |
    6 |                 |
    5 |                 |
    4 |                 |
    3 |                 |
    2 |                 |
    1 |         ♔     ♖ |
      +---------------v-+
        a b c d e f g h").unwrap();
    assert_eq!(true, next_boards(&board, Color::White).into_iter()
        .filter(|board| board.get_square(Coordinates::new(File::G, Rank::R1)).is_occupied_by(Piece(Color::White, Kind::King)))
        .filter(|board| board.get_square(Coordinates::new(File::F, Rank::R1)).is_occupied_by(Piece(Color::White, Kind::Rook)))
        .filter(|board| !board.is_castling_allowed(Color::White, CastlingSide::King))
        .filter(|board| !board.is_castling_allowed(Color::White, CastlingSide::Queen))
        .any(|_| true));

    // But not if threatened.
    let board = Board::parse_str("
      +-----------------+
    8 |             ♜   |
    7 |                 |
    6 |                 |
    5 |                 |
    4 |                 |
    3 |                 |
    2 |                 |
    1 |         ♔     ♖ |
      +---------------v-+
        a b c d e f g h").unwrap();
    assert_eq!(false, next_boards(&board, Color::White).into_iter()
        .filter(|board| board.get_square(Coordinates::new(File::G, Rank::R1)).is_occupied_by(Piece(Color::White, Kind::King)))
        .filter(|board| board.get_square(Coordinates::new(File::F, Rank::R1)).is_occupied_by(Piece(Color::White, Kind::Rook)))
        .filter(|board| !board.is_castling_allowed(Color::White, CastlingSide::King))
        .filter(|board| !board.is_castling_allowed(Color::White, CastlingSide::Queen))
        .any(|_| true));

    // Black queen's castle.
    let board = Board::parse_str("
      +-v---------------+
    8 | ♜       ♚       |
    7 |                 |
    6 |                 |
    5 |                 |
    4 |                 |
    3 |                 |
    2 |                 |
    1 |                 |
      +-----------------+
        a b c d e f g h").unwrap();
    assert_eq!(true, next_boards(&board, Color::Black).into_iter()
        .filter(|board| board.get_square(Coordinates::new(File::C, Rank::R8)).is_occupied_by(Piece(Color::Black, Kind::King)))
        .filter(|board| board.get_square(Coordinates::new(File::D, Rank::R8)).is_occupied_by(Piece(Color::Black, Kind::Rook)))
        .filter(|board| !board.is_castling_allowed(Color::Black, CastlingSide::King))
        .filter(|board| !board.is_castling_allowed(Color::Black, CastlingSide::Queen))
        .any(|_| true));
}

fn test_check() {
    // Test that we cannot move the king into check.
    let board = Board::parse_str("
      +-----------------+
    8 |       ♖         |
    7 |                 |
    6 |                 |
    5 |                 |
    4 |         ♚       |
    3 |                 |
    2 |                 |
    1 |           ♖     |
      +-----------------+
        a b c d e f g h").unwrap();
    assert_eq!(2, next_boards(&board, Color::Black).len());

    // Test that we cannot expose the king to check by moving another piece.
    let board = Board::parse_str("
      +-----------------+
    8 | ♗     ♖         |
    7 |   ♜             |
    6 |                 |
    5 |                 |
    4 |         ♚       |
    3 |                 |
    2 |                 |
    1 |           ♖     |
      +-----------------+
        a b c d e f g h").unwrap();
    assert_eq!(2, next_boards(&board, Color::Black).len());
}

fn test_score() {
    let mut board = Board::new();
    assert_eq!(0, score_board(&board));

    board.setup();
    assert_eq!(0, score_board(&board));

    board.set_square(Coordinates::new(File::A, Rank::R1), Square::Empty);
    assert_eq!(true, score_board(&board) < 0);

    board.set_square(Coordinates::new(File::A, Rank::R8), Square::Empty);
    assert_eq!(0, score_board(&board));

    board.set_square(Coordinates::new(File::A, Rank::R7), Square::Empty);
    assert_eq!(true, score_board(&board) > 0);
}

fn test_board_parser() {
    let board_data = "
  +-v-------------v-+
8 | ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ |
7 | ♟︎ ♟︎ ♟︎ ♟︎ ♟︎   ♟︎ ♟︎ |
6 |                 |
5 |     ♕           |
4 |             ♙   |
3 |             *   |
2 | ♙ ♙ ♙ ♙ ♙ ♙   ♙ |
1 | ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ |
  +-v---------------+
    a b c d e f g h";

    let board = Board::parse_str(board_data);
    assert_eq!(board.map(|board| String::from(format!("{}", board).trim())), Ok(String::from(board_data.trim())));
}

fn test_score_deep() {
  // TODO: Take a look at this. White thinks it can checkmate, which it cannot!
  let board = Board::parse_str("
    +-----------------+
  8 |             ♖   |
  7 |                 |
  6 |                 |
  5 |                 |
  4 |                 |
  3 |                 |
  2 |               ♚ |
  1 |             ♖   |
    +-----------------+
      a b c d e f g h").unwrap();


  let mut steps_remaining = 10000;

  let scored_boards = scored_next_boards(&board, Color::White, &mut || {
    steps_remaining = steps_remaining - 1;
    steps_remaining >= 0
  });

  for (score, board) in scored_boards {
    println!("SCORE: {}\n{}\n\n", score, board);
  }
}
