use crate::primitives::{File, Rank};
use chess_kit_derive::{EnumBitOps, IndexableEnum};
use std::fmt;

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash, EnumBitOps, IndexableEnum)]
pub enum Square {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,

    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,

    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,

    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,

    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,

    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,

    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,

    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

impl Square {
    pub const TOTAL: usize = 64;

    #[rustfmt::skip]
    pub const ALL: [Square; Self::TOTAL] = [
        Self::A1, Self::B1, Self::C1, Self::D1, Self::E1, Self::F1, Self::G1, Self::H1,
        Self::A2, Self::B2, Self::C2, Self::D2, Self::E2, Self::F2, Self::G2, Self::H2,
        Self::A3, Self::B3, Self::C3, Self::D3, Self::E3, Self::F3, Self::G3, Self::H3,
        Self::A4, Self::B4, Self::C4, Self::D4, Self::E4, Self::F4, Self::G4, Self::H4,
        Self::A5, Self::B5, Self::C5, Self::D5, Self::E5, Self::F5, Self::G5, Self::H5,
        Self::A6, Self::B6, Self::C6, Self::D6, Self::E6, Self::F6, Self::G6, Self::H6,
        Self::A7, Self::B7, Self::C7, Self::D7, Self::E7, Self::F7, Self::G7, Self::H7,
        Self::A8, Self::B8, Self::C8, Self::D8, Self::E8, Self::F8, Self::G8, Self::H8,
    ];

    // rank returns the rank of the square
    //
    // @param: self - immutable reference to the square
    // @return: rank of the square
    #[inline(always)]
    pub fn rank(&self) -> Rank {
        Rank::from_idx(self.idx() / 8)
    }

    // file returns the file of the square
    //
    // @param: self - immutable reference to the square
    // @return: file of the square
    #[inline(always)]
    pub fn file(&self) -> File {
        File::from_idx(self.idx() % 8)
    }

    // is_white returns true if the square is a white square
    //
    // @param: self - immutable reference to the square
    // @return: true if the square is a white square, false otherwise
    #[inline(always)]
    pub fn is_white(&self) -> bool {
        let even_rank = ((self.idx() / 8) & 1) == 0;
        let even_square = (self.idx() & 1) == 0;
        even_rank ^ even_square
    }

    // distance returns the distance between two squares
    //
    // @param: self - immutable reference to the square
    // @param: other - square to calculate the distance to
    // @return: distance between the two squares
    #[inline(always)]
    pub const fn distance(&self, other: Square) -> u8 {
        (self.idx() as i8 - other.idx() as i8).abs() as u8
    }

    // on_rank returns true if the square is on the given rank
    //
    // @param: self - immutable reference to the square
    // @param: rank - rank to check
    // @return: true if the square is on the given rank, false otherwise
    #[inline(always)]
    pub fn on_rank(&self, rank: Rank) -> bool {
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
            '1' => Rank::R1,
            '2' => Rank::R2,
            '3' => Rank::R3,
            '4' => Rank::R4,
            '5' => Rank::R5,
            '6' => Rank::R6,
            '7' => Rank::R7,
            '8' => Rank::R8,
            _ => return Err("invalid square"),
        };

        Ok(Square::from_idx((rank.idx() * 8) + file.idx()))
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
            Rank::R1 => write!(f, "1"),
            Rank::R2 => write!(f, "2"),
            Rank::R3 => write!(f, "3"),
            Rank::R4 => write!(f, "4"),
            Rank::R5 => write!(f, "5"),
            Rank::R6 => write!(f, "6"),
            Rank::R7 => write!(f, "7"),
            Rank::R8 => write!(f, "8"),
        }?;
        Ok(())
    }
}
