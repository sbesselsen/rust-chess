use std::fmt;

#[derive(Debug,Copy,PartialEq,Clone,Hash)]
pub struct Position {
    pub rank: u8,
    pub file: u8,
    index: usize
}

impl Position {
    pub fn new_from_index(index: usize) -> Option<Position> {
        match index {
            0..=63 => {
                let index_u8 = index as u8;
                let file: u8 = index_u8 % 8;
                let rank: u8 = (index_u8 - file) / 8;
                Some(Position { rank, file, index })
            },
            _ => None
        }
    }

    pub fn new(rank: u8, file: u8) -> Option<Position>  {
        Position::new_signed(rank as i8, file as i8)
    }

    fn new_signed(rank: i8, file: i8) -> Option<Position>  {
        match (rank, file) {
            (0..=7, 0..=7) => Some(Position { rank: rank as u8, file: file as u8, index: calculate_index(rank as u8, file as u8) }),
            _ => None
        }
    }

    pub fn offset(&self, rank: i8, file: i8) -> Option<Position> {
        Position::new_signed(self.rank as i8 + rank, self.file as i8 + file)
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

fn calculate_index(rank: u8, file: u8) -> usize {
    (rank as usize) * 8 + (file as usize)
}

const FILE_LETTERS: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

impl fmt::Display for Position {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", FILE_LETTERS[self.file as usize], self.rank + 1)
    }
}
