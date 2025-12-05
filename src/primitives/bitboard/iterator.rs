use crate::primitives::bitboard::Bitboard;
use crate::primitives::Square;

impl Bitboard {
    // first returns the first square in the bitboard
    //
    // @param: self - immutable reference to the bitboard
    // @return: first square in the bitboard
    #[inline(always)]
    pub const fn first(&self) -> Option<Square> {
        match self.0 {
            0 => None,
            _ => Some(Square::from_idx(self.0.trailing_zeros() as usize)),
        }
    }

    // must_first returns the first square in the bitboard
    //
    // @param: self - immutable reference to the bitboard
    // @return: first square in the bitboard
    #[inline(always)]
    pub const fn must_first(&self) -> Square {
        debug_assert!(!self.is_empty(), "bitboard is empty");
        Square::from_idx(self.0.trailing_zeros() as usize)
    }

    // iter iterates over the squares in the bitboard
    //
    // @param: self - immutable reference to the bitboard
    // @return: iterator over the squares in the bitboard
    pub fn iter(&self) -> BitboardIter {
        BitboardIter(self.0)
    }
}

pub struct BitboardIter(u64);

impl Iterator for BitboardIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            0 => None,
            _ => {
                let square = self.0.trailing_zeros();
                self.0 &= self.0 - 1;
                Some(Square::from_idx(square as usize))
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
