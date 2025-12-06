use crate::primitives::bitboard::constants::{
    BITBOARD_BETWEEN, BITBOARD_FILES, BITBOARD_RANKS, BITBOARD_SQUARES,
};
use crate::primitives::{File, Rank, Square};
use chess_kit_derive::{Arithmetic, BitOps};

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

    // all creates a new bitboard with all bits set to 1
    //
    // @return: new bitboard
    #[inline(always)]
    pub const fn all() -> Self {
        Self(u64::MAX)
    }

    // square creates a new bitboard with the bit for the given square set to 1
    //
    // @param: square - square to create the bitboard from
    // @return: new bitboard
    #[inline(always)]
    pub const fn square(square: Square) -> Self {
        BITBOARD_SQUARES[square.idx()]
    }

    // file creates a new bitboard with the bits for the given file set to 1
    //
    // @param: file - file to create the bitboard from
    // @return: new bitboard
    #[inline(always)]
    pub const fn file(file: File) -> Self {
        BITBOARD_FILES[file.idx()]
    }

    // rank creates a new bitboard with the bits for the given rank set to 1
    //
    // @param: rank - rank to create the bitboard from
    // @return: new bitboard
    #[inline(always)]
    pub const fn rank(rank: Rank) -> Self {
        BITBOARD_RANKS[rank.idx()]
    }

    // between returns the between bitboard for the given start (exclusive) and
    // end (inclusive) squares
    //
    // @param: start - start square (exclusive)
    // @param: end - end square (inclusive)
    // @return: between bitboard
    #[inline(always)]
    pub const fn between(start: Square, end: Square) -> Self {
        BITBOARD_BETWEEN[start.idx()][end.idx()]
    }

    // is_empty checks if the bitboard is empty
    //
    // @return: true if the bitboard is empty, false otherwise
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    // not_empty checks if the bitboard is not empty
    //
    // @return: true if the bitboard is not empty, false otherwise
    #[inline(always)]
    pub const fn not_empty(&self) -> bool {
        self.0 != 0
    }

    // exactly_one checks if the bitboard has exactly one bit set
    //
    // @return: true if the bitboard has exactly one bit set, false otherwise
    #[inline(always)]
    pub const fn exactly_one(&self) -> bool {
        self.0.count_ones() == 1
    }

    // more_than_one checks if the bitboard has more than one bit set
    //
    // @return: true if the bitboard has more than one bit set, false otherwise
    #[inline(always)]
    pub const fn more_than_one(&self) -> bool {
        self.0.count_ones() > 1
    }

    // remove_at removes the piece at the given square
    //
    // @param: square - square to remove the piece from
    // @return: void
    // @side-effects: modifies the `bitboard`
    #[inline(always)]
    pub fn remove_at(&mut self, square: Square) {
        self.0 &= !BITBOARD_SQUARES[square.idx()].0;
    }

    // set_at sets the piece at the given square
    //
    // @param: square - square to set the piece on
    // @return: void
    // @side-effects: modifies the `bitboard`
    #[inline(always)]
    pub fn set_at(&mut self, square: Square) {
        self.0 |= BITBOARD_SQUARES[square.idx()].0;
    }
}
