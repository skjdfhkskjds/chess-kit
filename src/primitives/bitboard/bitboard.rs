use chess_kit_derive::{Arithmetic, BitOps};
use crate::primitives::{BITBOARD_SQUARES, Square};

pub type BitboardVec = Vec<Bitboard>;

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Default, Hash, BitOps, Arithmetic)]
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

    // is_empty checks if the bitboard is empty
    //
    // @param: self - immutable reference to the bitboard
    // @return: true if the bitboard is empty, false otherwise
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    // remove_at removes the piece at the given square
    //
    // @param: self - mutable reference to the bitboard
    // @param: square - square to remove the piece from
    // @return: void
    // @side-effects: modifies the `bitboard`
    #[inline(always)]
    pub fn remove_at(&mut self, square: Square) {
        self.0 &= !BITBOARD_SQUARES[square.idx()].0;
    }

    // set_at sets the piece at the given square
    //
    // @param: self - mutable reference to the bitboard
    // @param: square - square to set the piece on
    // @return: void
    // @side-effects: modifies the `bitboard`
    #[inline(always)]
    pub fn set_at(&mut self, square: Square) {
        self.0 |= BITBOARD_SQUARES[square.idx()].0;
    }
}
