use crate::Square;
use crate::bitboard::Bitboard;

use std::iter::FusedIterator;

impl Bitboard {
    /// first returns the first square in the bitboard
    ///
    /// @param: self - immutable reference to the bitboard
    /// @return: first square in the bitboard
    #[inline(always)]
    pub fn first(&self) -> Option<Square> {
        let bits = self.0;
        if bits == 0 {
            return None;
        }

        Some(self.first_unchecked())
    }

    /// first_unchecked returns the first square in the bitboard
    ///
    /// @param: self - immutable reference to the bitboard
    /// @return: first square in the bitboard
    #[inline(always)]
    pub fn first_unchecked(&self) -> Square {
        debug_assert!(self.not_empty(), "bitboard is empty");
        Square::from_idx(self.0.trailing_zeros() as usize)
    }

    /// pop_front_unchecked pops the first square from the bitboard
    ///
    /// @param: self - mutable reference to the bitboard
    /// @return: first square in the bitboard
    /// @side-effects: modifies the `bitboard`
    #[inline(always)]
    pub fn pop_front_unchecked(&mut self) -> Square {
        let square = self.first_unchecked();
        self.0 &= self.0 - 1;
        square
    }

    /// iter iterates over the squares in the bitboard
    ///
    /// @param: self - immutable reference to the bitboard
    /// @return: iterator over the squares in the bitboard
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
        let bits = self.0;
        if bits == 0 {
            return None;
        }

        let square = bits.trailing_zeros();
        self.0 = bits & (bits - 1);
        Some(Square::from_idx(square as usize))
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.0.count_ones() as usize;
        (n, Some(n))
    }
}

impl ExactSizeIterator for BitboardIter {
    #[inline(always)]
    fn len(&self) -> usize {
        self.0.count_ones() as usize
    }
}

impl FusedIterator for BitboardIter {}

impl IntoIterator for Bitboard {
    type Item = Square;
    type IntoIter = BitboardIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
