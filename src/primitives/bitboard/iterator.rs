use crate::primitives::bitboard::Bitboard;
use crate::primitives::Square;

impl Bitboard {
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
