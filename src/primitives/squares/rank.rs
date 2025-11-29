use crate::primitives::Side;
use chess_kit_derive::IndexableEnum;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, IndexableEnum)]
#[repr(u8)]
pub enum Rank {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

impl Rank {
    pub const TOTAL: usize = 8;

    // double_step_rank returns the rank that a pawn can double step to
    //
    // @param: side - side to get the double step rank for
    // @return: double step rank for the side
    pub const fn double_step_rank(side: Side) -> Rank {
        match side {
            Side::White => Rank::R4,
            Side::Black => Rank::R5,
        }
    }

    // promotion_rank returns the rank that a pawn can promote to
    //
    // @param: side - side to get the promotion rank for
    // @return: promotion rank for the side
    pub const fn promotion_rank(side: Side) -> Rank {
        match side {
            Side::White => Rank::R8,
            Side::Black => Rank::R1,
        }
    }

    // inc increments the rank by one
    //
    // Note: this function is unsafe because it calls `from_idx`
    //
    // @param: self - immutable reference to the rank
    // @return: rank incremented by one
    #[inline(always)]
    pub fn inc(&mut self) {
        *self = Self::from_idx(self.idx() + 1);
    }

    // dec decrements the rank by one
    //
    // Note: this function is unsafe because it calls `from_idx`
    //
    // @param: self - immutable reference to the rank
    // @return: rank decremented by one
    #[inline(always)]
    pub fn dec(&mut self) {
        *self = Self::from_idx(self.idx() - 1);
    }
}

impl TryFrom<&str> for Rank {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() != 1 {
            return Err("invalid rank");
        }

        let char = s.chars().next().unwrap();
        match char {
            '1' => Ok(Rank::R1),
            '2' => Ok(Rank::R2),
            '3' => Ok(Rank::R3),
            '4' => Ok(Rank::R4),
            '5' => Ok(Rank::R5),
            '6' => Ok(Rank::R6),
            '7' => Ok(Rank::R7),
            '8' => Ok(Rank::R8),
            _ => Err("invalid rank"),
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rank::R1 => write!(f, "1"),
            Rank::R2 => write!(f, "2"),
            Rank::R3 => write!(f, "3"),
            Rank::R4 => write!(f, "4"),
            Rank::R5 => write!(f, "5"),
            Rank::R6 => write!(f, "6"),
            Rank::R7 => write!(f, "7"),
            Rank::R8 => write!(f, "8"),
        }
    }
}
