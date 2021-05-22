use std::fmt;

#[derive(Debug,Copy,PartialEq,Clone,Hash)]
pub struct Position {
    pub rank: u8,
    pub file: u8
}

impl Position {
    pub fn from_index(index: usize) -> Option<Position> {
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

    pub fn offset(&self, rank: i8, file: i8) -> Option<Position> {
        let target_rank = (self.rank as i8) + rank;
        let target_file = (self.file as i8) + file;
        match (target_rank, target_file) {
            (0..=7, 0..=7) => Some(Position { rank: target_rank as u8, file: target_file as u8 }),
            _ => None
        }
    }

    pub fn to_index(&self) -> usize {
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
