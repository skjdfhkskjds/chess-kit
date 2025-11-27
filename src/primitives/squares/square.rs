use crate::primitives::{File, Files, Rank, Ranks};
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
    pub const fn file(&self) -> File {
        (self.0 % 8) as File
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
            'A' => Files::A,
            'B' => Files::B,
            'C' => Files::C,
            'D' => Files::D,
            'E' => Files::E,
            'F' => Files::F,
            'G' => Files::G,
            'H' => Files::H,
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

        Ok(Square::new((rank * 8) + file))
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.file() {
            Files::A => write!(f, "A"),
            Files::B => write!(f, "B"),
            Files::C => write!(f, "C"),
            Files::D => write!(f, "D"),
            Files::E => write!(f, "E"),
            Files::F => write!(f, "f"),
            Files::G => write!(f, "G"),
            Files::H => write!(f, "H"),
            _ => return Err(fmt::Error),
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
