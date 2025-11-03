use crate::primitives::Square;
use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct Bitboard(u64);

impl Bitboard {
    // new creates a new bitboard with the given u64 value
    //
    // @param: bits - u64 value to create the bitboard from
    // @return: new bitboard
    #[inline(always)]
    pub fn new(bits: u64) -> Self {
        Self(bits)
    }

    // empty creates a new bitboard with all bits set to 0
    //
    // @return: new bitboard
    #[inline(always)]
    pub fn empty() -> Self {
        Self(0)
    }

    // bits gets the underlying u64 value of the bitboard
    //
    // @param: self - immutable reference to the bitboard
    // @return: underlying u64 value of the bitboard
    #[inline(always)]
    pub fn bits(&self) -> u64 {
        self.0
    }

    // iter iterates over the squares in the bitboard
    //
    // @param: self - immutable reference to the bitboard
    // @return: iterator over the squares in the bitboard
    pub fn iter(&self) -> BitboardIter {
        BitboardIter(self.0)
    }
}

// ================================================
//                bitwise operations
// ================================================

impl BitOr for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl Not for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl BitXor for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitboard {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

// ================================================
//                    formatting
// ================================================

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const LAST_BIT: u64 = 63;
        for rank in 0..8 {
            for file in (0..8).rev() {
                let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
                let char = if self.0 & mask != 0 { '1' } else { '0' };
                write!(f, "{char} ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// ================================================
//                    iteration
// ================================================

pub struct BitboardIter(u64);

impl Iterator for BitboardIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            0 => None,
            _ => {
                let square = self.0.trailing_zeros();
                self.0 ^= 1u64 << square;
                Some(Square::new(square as usize))
            }
        }
    }
}

impl IntoIterator for Bitboard {
    type Item = Square;
    type IntoIter = BitboardIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
