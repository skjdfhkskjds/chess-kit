use crate::primitives::bitboard::Bitboard;

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

    // rotate_left performs a left rotation of a bitboard by a given number of
    // bits
    //
    // @param: self - immutable reference to the bitboard
    // @param: rhs - number of bits to rotate the bitboard by
    // @return: result of the left rotation
    #[inline(always)]
    pub fn rotate_left(self, rhs: u32) -> Self {
        Self(self.0.rotate_left(rhs))
    }

    // count_ones counts the number of bits set in the bitboard
    //
    // @param: self - immutable reference to the bitboard
    // @return: number of bits set in the bitboard
    #[inline(always)]
    pub const fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }

    // trailing_zeros counts the number of trailing zeros in the bitboard
    //
    // @param: self - immutable reference to the bitboard
    // @return: number of trailing zeros in the bitboard
    #[inline(always)]
    pub const fn trailing_zeros(&self) -> u32 {
        self.0.trailing_zeros()
    }
}
