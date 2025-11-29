use crate::primitives::{File, Rank, Ranks};
use chess_kit_derive::{Arithmetic, BitOps};
use std::fmt;

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Default, Hash, BitOps, Arithmetic)]
pub struct Square(usize);

impl Square {
    // new creates a new square with the given usize value
    //
    // @param: square - usize value to create the square from
    // @return: new square
    #[inline(always)]
    pub const fn new(square: usize) -> Self {
        Self(square)
    }

    // unwrap unwraps the square to get the underlying usize value
    //
    // @param: self - immutable reference to the square
    // @return: underlying usize value
    #[inline(always)]
    pub fn unwrap(&self) -> usize {
        self.0
    }

    // rank returns the rank of the square
    //
    // @param: self - immutable reference to the square
    // @return: rank of the square
    #[inline(always)]
    pub const fn rank(&self) -> Rank {
        (self.0 / 8) as Rank
    }

    // file returns the file of the square
    //
    // @param: self - immutable reference to the square
    // @return: file of the square
    #[inline(always)]
    pub fn file(&self) -> File {
        File::from_idx(self.0 % 8)
    }

    // is_white returns true if the square is a white square
    //
    // @param: self - immutable reference to the square
    // @return: true if the square is a white square, false otherwise
    #[inline(always)]
    pub fn is_white(&self) -> bool {
        let even_rank = ((self.0 / 8) & 1) == 0;
        let even_square = (self.0 & 1) == 0;
        even_rank ^ even_square
    }

    // distance returns the distance between two squares
    //
    // @param: self - immutable reference to the square
    // @param: other - square to calculate the distance to
    // @return: distance between the two squares
    #[inline(always)]
    pub const fn distance(&self, other: Square) -> u8 {
        (self.0 as i8 - other.0 as i8).abs() as u8
    }

    // on_rank returns true if the square is on the given rank
    //
    // @param: self - immutable reference to the square
    // @param: rank - rank to check
    // @return: true if the square is on the given rank, false otherwise
    #[inline(always)]
    pub const fn on_rank(&self, rank: Rank) -> bool {
        self.rank() == rank
    }

    // on_file returns true if the square is on the given file
    //
    // @param: self - immutable reference to the square
    // @param: file - file to check
    // @return: true if the square is on the given file, false otherwise
    #[inline(always)]
    pub fn on_file(&self, file: File) -> bool {
        self.file() == file
    }
}

impl TryFrom<&str> for Square {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() != 2 {
            return Err("invalid square");
        }

        let file_str = s.chars().next().unwrap();
        let rank_str = s.chars().nth(1).unwrap();

        let file = match file_str {
            'A' => File::A,
            'B' => File::B,
            'C' => File::C,
            'D' => File::D,
            'E' => File::E,
            'F' => File::F,
            'G' => File::G,
            'H' => File::H,
            _ => return Err("invalid square"),
        };

        let rank = match rank_str {
            '1' => Ranks::R1,
            '2' => Ranks::R2,
            '3' => Ranks::R3,
            '4' => Ranks::R4,
            '5' => Ranks::R5,
            '6' => Ranks::R6,
            '7' => Ranks::R7,
            '8' => Ranks::R8,
            _ => return Err("invalid square"),
        };

        Ok(Square::new((rank * 8) + file.idx()))
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.file() {
            File::A => write!(f, "a"),
            File::B => write!(f, "b"),
            File::C => write!(f, "c"),
            File::D => write!(f, "d"),
            File::E => write!(f, "e"),
            File::F => write!(f, "f"),
            File::G => write!(f, "g"),
            File::H => write!(f, "h"),
        }?;
        match self.rank() {
            Ranks::R1 => write!(f, "1"),
            Ranks::R2 => write!(f, "2"),
            Ranks::R3 => write!(f, "3"),
            Ranks::R4 => write!(f, "4"),
            Ranks::R5 => write!(f, "5"),
            Ranks::R6 => write!(f, "6"),
            Ranks::R7 => write!(f, "7"),
            Ranks::R8 => write!(f, "8"),
            _ => return Err(fmt::Error),
        }?;
        Ok(())
    }
}
