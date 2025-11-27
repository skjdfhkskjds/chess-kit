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
}
