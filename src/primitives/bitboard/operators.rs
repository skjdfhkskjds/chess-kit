use crate::primitives::bitboard::Bitboard;
use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign, Add
};

impl BitOr for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn bitor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl<'a> BitOr<&'a Bitboard> for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn bitor(self, rhs: &'a Bitboard) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl<'a> BitOr<Bitboard> for &'a Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn bitor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl<'a> BitOrAssign<&'a Bitboard> for Bitboard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: &'a Bitboard) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl<'a> BitAnd<Bitboard> for &'a Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn bitand(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl<'a> BitAnd<&'a Bitboard> for Bitboard {
    type Output = Bitboard;
    #[inline(always)]
    fn bitand(self, rhs: &'a Bitboard) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl<'a> BitAndAssign<&'a Bitboard> for Bitboard {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: &'a Bitboard) {
        self.0 &= rhs.0;
    }
}

impl Not for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl<'a> BitXor<Bitboard> for &'a Bitboard {
    type Output = Bitboard;
    #[inline(always)]
    fn bitxor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl<'a> BitXor<&'a Bitboard> for Bitboard {
    type Output = Bitboard;
    #[inline(always)]
    fn bitxor(self, rhs: &'a Bitboard) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitboard {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl<'a> BitXorAssign<&'a Bitboard> for Bitboard {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: &'a Bitboard) {
        self.0 ^= rhs.0;
    }
}

impl Shl<u32> for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn shl(self, rhs: u32) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl Shr<u32> for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn shr(self, rhs: u32) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

impl ShrAssign<u32> for Bitboard {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: u32) {
        self.0 >>= rhs;
    }
}

impl ShlAssign<u32> for Bitboard {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: u32) {
        self.0 <<= rhs;
    }
}

impl Add<u64> for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn add(self, rhs: u64) -> Self::Output {
        Bitboard(self.0 + rhs)
    }
}

// TODO: figure out how to deduplicate operator defs for different bitsize
//       support that resolve at compile time
impl Shl<u8> for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn shl(self, rhs: u8) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl Shr<u8> for Bitboard {
    type Output = Bitboard;

    #[inline(always)]
    fn shr(self, rhs: u8) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

impl ShrAssign<u8> for Bitboard {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: u8) {
        self.0 >>= rhs;
    }
}

impl ShlAssign<u8> for Bitboard {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: u8) {
        self.0 <<= rhs;
    }
}

// ================================================
//                  u64 operations
// ================================================

impl Into<u64> for Bitboard {
    fn into(self) -> u64 {
        self.0
    }
}

impl Bitboard {
    // wrapping_sub performs a wrapping subtraction of a bitboard with any type
    // that can be converted to a u64
    //
    // @param: self - immutable reference to the bitboard
    // @param: rhs - value to wrapping subtract from the bitboard
    // @return: result of the wrapping subtraction
    #[inline(always)]
    pub fn wrapping_sub<T: Into<u64>>(self, rhs: T) -> Self {
        Self(self.0.wrapping_sub(rhs.into()))
    }

    // wrapping_mul performs a wrapping multiplication of a bitboard with any
    // type that can be converted to a u64
    //
    // @param: self - immutable reference to the bitboard
    // @param: rhs - value to wrapping multiply the bitboard by
    // @return: result of the wrapping multiplication
    #[inline(always)]
    pub fn wrapping_mul<T: Into<u64>>(self, rhs: T) -> Self {
        Self(self.0.wrapping_mul(rhs.into()))
    }
}
