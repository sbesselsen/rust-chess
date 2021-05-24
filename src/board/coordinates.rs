use std::fmt;

#[derive(Debug,Copy,PartialEq,Clone,Hash)]
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

const ALL_RANKS: [Rank; 8] = [Rank::R1, Rank::R2, Rank::R3, Rank::R4, Rank::R5, Rank::R6, Rank::R7, Rank::R8];

impl Rank {
    pub fn new_from_index(index: u8) -> Option<Rank> {
        if index > 7 { None } else { Some(ALL_RANKS[index as usize]) }
    }

    pub fn index(&self) -> u8 {
        *self as u8
    }
}

impl fmt::Display for Rank {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.index() + 1)
    }
}

#[derive(Debug,Copy,PartialEq,Clone,Hash)]
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

const ALL_FILES: [File; 8] = [File::A, File::B, File::C, File::D, File::E, File::F, File::G, File::H];

impl File {
    pub fn new_from_index(index: u8) -> Option<File> {
        if index > 7 { None } else { Some(ALL_FILES[index as usize]) }
    }

    pub fn index(&self) -> u8 {
        *self as u8
    }
}

const FILE_LETTERS: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

impl fmt::Display for File {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", FILE_LETTERS[self.index() as usize])
    }
}

#[derive(Debug,Copy,PartialEq,Clone,Hash)]
pub struct Coordinates {
    pub file: File,
    pub rank: Rank,
    index: usize
}

impl Coordinates {
    pub fn new_from_index(index: usize) -> Option<Coordinates> {
        match index {
            0..=63 => {
                let index_u8 = index as u8;
                let file_u8 = index_u8 % 8;
                let file: File = File::new_from_index(file_u8).unwrap();
                let rank: Rank = Rank::new_from_index((index_u8 - file_u8) / 8).unwrap();
                Some(Coordinates { rank, file, index })
            },
            _ => None
        }
    }

    pub fn new(file: File, rank: Rank) -> Coordinates {
        return Coordinates { rank, file, index: calculate_index(file.index(), rank.index()) }
    }

    pub fn offset(&self, file_offset: i8, rank_offset: i8) -> Option<Coordinates> {
        let new_file = self.file as i8 + file_offset;
        let new_rank = self.rank as i8 + rank_offset;
        if new_file >= 0 && new_rank >= 0 {
            if let Some(file) = File::new_from_index(new_file as u8) {
                if let Some(rank) = Rank::new_from_index(new_rank as u8) {
                    return Some(Coordinates::new(file, rank))
                }
            }
        }
        None
    }

    pub fn offsets_filter(&self, offsets: &[(i8, i8)]) -> Vec<Coordinates> {
        offsets.iter()
            .filter_map(|(file_offset, rank_offset)| self.offset(*file_offset, *rank_offset))
            .collect()
    }

    pub fn offsets_repeated(&self, file_offset: i8, rank_offset: i8) -> Vec<Coordinates> {
        (1..).map(|multiple| self.offset(file_offset * multiple, rank_offset * multiple))
            .take_while(|opt| opt.is_some())
            .map(|opt| opt.unwrap())
            .collect()
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }

    #[allow(dead_code)]
    pub fn file(&self) -> File {
        self.file
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

fn calculate_index(file: u8, rank: u8) -> usize {
    (rank as usize) * 8 + (file as usize)
}

impl fmt::Display for Coordinates {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}
