use std::fmt;

#[derive(Debug,Copy,PartialEq,Clone,Hash)]
pub struct Coordinates {
    pub rank: u8,
    pub file: u8,
    index: usize
}

#[allow(dead_code)]
pub enum Rank {
    R1 = 0,
    R2 = 1,
    R3 = 2,
    R4 = 3,
    R5 = 4,
    R6 = 5,
    R7 = 6,
    R8 = 7
}

#[allow(dead_code)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7
}

impl Coordinates {
    pub fn new_from_index(index: usize) -> Option<Coordinates> {
        match index {
            0..=63 => {
                let index_u8 = index as u8;
                let file: u8 = index_u8 % 8;
                let rank: u8 = (index_u8 - file) / 8;
                Some(Coordinates { rank, file, index })
            },
            _ => None
        }
    }

    pub fn new(file: File, rank: Rank) -> Coordinates {
        Coordinates::new_signed(rank as i8, file as i8).unwrap()
    }

    pub fn new_unsigned(rank: u8, file: u8) -> Option<Coordinates>  {
        Coordinates::new_signed(rank as i8, file as i8)
    }

    fn new_signed(rank: i8, file: i8) -> Option<Coordinates>  {
        match (rank, file) {
            (0..=7, 0..=7) => Some(Coordinates { rank: rank as u8, file: file as u8, index: calculate_index(rank as u8, file as u8) }),
            _ => None
        }
    }

    pub fn offset(&self, rank: i8, file: i8) -> Option<Coordinates> {
        Coordinates::new_signed(self.rank as i8 + rank, self.file as i8 + file)
    }

    pub fn offsets_repeated(&self, rank_offset: i8, file_offset: i8) -> Vec<Coordinates> {
        let mut iterated_coords = vec![];
        for multiple in 1.. {
            if let Some(target_coordinates) = self.offset(rank_offset * multiple, file_offset * multiple) {
                iterated_coords.push(target_coordinates);
            } else {
                break
            }
        }
        iterated_coords
    }

    pub fn rank(&self) -> u8 {
        self.rank
    }

    pub fn file(&self) -> u8 {
        self.file
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

fn calculate_index(rank: u8, file: u8) -> usize {
    (rank as usize) * 8 + (file as usize)
}

const FILE_LETTERS: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

impl fmt::Display for Coordinates {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", FILE_LETTERS[self.file as usize], self.rank + 1)
    }
}
