use crate::primitives::Square;

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct Bitboard(pub(crate) u64);

impl Bitboard {
    // new creates a new bitboard with the given u64 value
    //
    // @param: bits - u64 value to create the bitboard from
    // @return: new bitboard
    #[inline(always)]
    pub const fn new(bits: u64) -> Self {
        Self(bits)
    }

    // empty creates a new bitboard with all bits set to 0
    //
    // @return: new bitboard
    #[inline(always)]
    pub const fn empty() -> Self {
        Self(0)
    }

    // bits gets the underlying u64 value of the bitboard
    //
    // @param: self - immutable reference to the bitboard
    // @return: underlying u64 value of the bitboard
    #[inline(always)]
    pub const fn bits(&self) -> u64 {
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
