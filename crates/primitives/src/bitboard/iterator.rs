use crate::Square;
use crate::bitboard::Bitboard;

impl Bitboard {
    // first returns the first square in the bitboard
    //
    // @param: self - immutable reference to the bitboard
    // @return: first square in the bitboard
    #[inline(always)]
    pub fn first(&self) -> Option<Square> {
        match self.0 {
            0 => None,
            _ => Some(self.must_first()),
        }
    }

    // must_first returns the first square in the bitboard
    //
    // @param: self - immutable reference to the bitboard
    // @return: first square in the bitboard
    #[inline(always)]
    pub fn must_first(&self) -> Square {
        debug_assert!(self.not_empty(), "bitboard is empty");
        Square::from_idx(self.0.trailing_zeros() as usize)
    }

    // pop_first pops the first square from the bitboard
    //
    // @param: self - mutable reference to the bitboard
    // @return: first square in the bitboard
    // @side-effects: modifies the `bitboard`
    #[inline(always)]
    pub fn pop_first(&mut self) -> Square {
        let square = self.must_first();
        self.0 &= self.0 - 1;
        square
    }

    // iter iterates over the squares in the bitboard
    //
    // @param: self - immutable reference to the bitboard
    // @return: iterator over the squares in the bitboard
    #[inline(always)]
    pub fn iter(&self) -> BitboardIter {
        BitboardIter(self.0)
    }
}

pub struct BitboardIter(u64);

impl Iterator for BitboardIter {
    type Item = Square;

    #[inline(always)]
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
