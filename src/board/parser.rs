use crate::board::{ Board, Color, CastlingSide, Coordinates, Kind, Piece, Square };

pub fn parse_board(input: &str) -> Result<Board, String> {
    let lines: Vec<&str> = input.trim().split("\n").collect();

    if lines.len() != 11 {
        return Err(String::from("Invalid number of lines for board"));
    }

    let mut board = Board::new();

    parse_header(&mut board, lines[0])?;
    for (neg_rank, line) in (&lines[1..=8]).iter().enumerate() {
        parse_line(&mut board, (7 - neg_rank) as u8, line)?;
    }
    parse_footer(&mut board, lines[9])?;

    Ok(board)
}

const HEADER_FOOTER_PREFIXES: [&str; 1] = ["+-"];
const HEADER_FOOTER_MIDDLE: [&str; 1] = ["-------------"];
const HEADER_FOOTER_CASTLING_YES: &str = "v";
const HEADER_FOOTER_CASTLING_NO: &str = "-";
const HEADER_FOOTER_CASTLING_MARKERS: [&str; 2] = [HEADER_FOOTER_CASTLING_YES, HEADER_FOOTER_CASTLING_NO];
const HEADER_FOOTER_SUFFIXES: [&str; 1] = ["-+"];

fn parse_header(board: &mut Board, line: &str) -> Result<(), String> {
    parse_header_or_footer(board, Color::Black, line)
}

fn parse_footer(board: &mut Board, line: &str) -> Result<(), String> {
    parse_header_or_footer(board, Color::White, line)
}

fn parse_header_or_footer(board: &mut Board, color: Color, line: &str) -> Result<(), String> {
    let remainder = line.trim();
    let (remainder, _) = expect_prefixes(remainder, &HEADER_FOOTER_PREFIXES)?;
    let (remainder, prefix) = expect_prefixes(remainder, &HEADER_FOOTER_CASTLING_MARKERS)?;
    if prefix == HEADER_FOOTER_CASTLING_YES {
        board.set_castling_allowed(color, CastlingSide::Queen, true)
    }
    let (remainder, _) = expect_prefixes(remainder, &HEADER_FOOTER_MIDDLE)?;
    let (remainder, prefix) = expect_prefixes(remainder, &HEADER_FOOTER_CASTLING_MARKERS)?;
    if prefix == HEADER_FOOTER_CASTLING_YES {
        board.set_castling_allowed(color, CastlingSide::King, true)
    }
    let (remainder, _) = expect_prefixes(remainder, &HEADER_FOOTER_SUFFIXES)?;
    expect_empty(remainder)
}

const LINE_PREFIXES: [&str; 1] = [" |"];
const LINE_PIECES: [&str; 14] = ["♜", "♞", "♝", "♛", "♚", "♟︎", "♖", "♘", "♗", "♕", "♔", "♙", " ", "*"];
const LINE_COLSEPS: [&str; 1] = [" "];
const LINE_SUFFIXES: [&str; 1] = [" |"];

fn parse_line(board: &mut Board, rank: u8, line: &str) -> Result<(), String> {
    let remainder = line.trim();
    let (remainder, _) = expect_prefixes(remainder, &[&format!("{}", rank + 1)])?;
    let (mut remainder, _) = expect_prefixes(remainder, &LINE_PREFIXES)?;
    for file in 0..8 {
        let coordinates = Coordinates::new(rank, file).unwrap();

        let (loop_remainder, _) = expect_prefixes(remainder, &LINE_COLSEPS)?;
        let (loop_remainder, piece) = expect_prefixes(loop_remainder, &LINE_PIECES)?;
        if piece == "*" {
            // En passant.
            board.set_en_passant_capturable(Some(coordinates));
        } else {
            let square: Square = match piece {
                "♜" => Square::Occupied(Piece(Color::Black, Kind::Rook)),
                "♞" => Square::Occupied(Piece(Color::Black, Kind::Knight)),
                "♝" => Square::Occupied(Piece(Color::Black, Kind::Bishop)),
                "♛" => Square::Occupied(Piece(Color::Black, Kind::Queen)),
                "♚" => Square::Occupied(Piece(Color::Black, Kind::King)),
                "♟︎" => Square::Occupied(Piece(Color::Black, Kind::Pawn)),
                "♖" => Square::Occupied(Piece(Color::White, Kind::Rook)),
                "♘" => Square::Occupied(Piece(Color::White, Kind::Knight)),
                "♗" => Square::Occupied(Piece(Color::White, Kind::Bishop)),
                "♕" => Square::Occupied(Piece(Color::White, Kind::Queen)),
                "♔" => Square::Occupied(Piece(Color::White, Kind::King)),
                "♙" => Square::Occupied(Piece(Color::White, Kind::Pawn)),
                " " => Square::Empty,
                _ => { return Err(String::from(format!("Unexpected piece: {}", piece))); }
            };
            board.set_square(coordinates, square);
        }

        remainder = loop_remainder;
    }
    let (remainder, _) = expect_prefixes(remainder, &LINE_SUFFIXES)?;
    expect_empty(remainder)
}

fn try_prefixes<'a, 'b>(input: &'a str, options: &'b [&str]) -> Option<(&'a str, &'b str)> {
    for option in options {
        match input.strip_prefix(option) {
            Some(suffix) => { return Some((suffix, option)); },
            _ => {}
        }
    }
    None
}

fn expect_prefixes<'a, 'b>(input: &'a str, options: &'b [&str]) -> Result<(&'a str, &'b str), String> {
    if let Some(result) = try_prefixes(input, options) {
        Ok(result)
    } else {
        Err(format!("Expect one of: {}", options.join(", ")))
    }
}

fn expect_empty(input: &str) -> Result<(), String> {
    if input.is_empty() {
        Ok(())
    } else {
        Err(format!("Expect end of string, got {}", input))
    }
}
