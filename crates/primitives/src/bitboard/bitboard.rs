use crate::bitboard::constants::{
    BITBOARD_ANTI_DIAGONALS, BITBOARD_BETWEEN, BITBOARD_DIAGONALS, BITBOARD_FILES, BITBOARD_LINES,
    BITBOARD_RANKS,
};
use crate::{File, Rank, Square};
use chess_kit_derive::{Arithmetic, BitOps};

pub type BitboardVec = Vec<Bitboard>;

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Default, BitOps, Arithmetic)]
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
        Self(1 << square.idx())
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

    // diagonal creates a new bitboard with the bits for the given diagonal of
    // the given square set to 1
    //
    // @param: sq - square to create the bitboard from
    // @return: new bitboard
    #[inline(always)]
    pub const fn diagonal(sq: Square) -> Self {
        BITBOARD_DIAGONALS[sq.idx()]
    }

    // anti_diagonal creates a new bitboard with the bits for the given anti-
    // diagonal of the given square set to 1
    //
    // @param: sq - square to create the bitboard from
    // @return: new bitboard
    #[inline(always)]
    pub const fn anti_diagonal(sq: Square) -> Self {
        BITBOARD_ANTI_DIAGONALS[sq.idx()]
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

    // line returns the edge to edge line of the bitboard intersecting the two
    // given squares
    //
    // @param: s1 - first square
    // @param: s2 - second square
    // @return: line bitboard
    #[inline(always)]
    pub const fn line(s1: Square, s2: Square) -> Self {
        BITBOARD_LINES[s1.idx()][s2.idx()]
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
        self.not_empty() && !self.more_than_one()
    }

    // more_than_one checks if the bitboard has more than one bit set
    //
    // @return: true if the bitboard has more than one bit set, false otherwise
    #[inline(always)]
    pub const fn more_than_one(&self) -> bool {
        self.0 & (self.0 - 1) != 0
    }

    // intersects checks whether there is an intersection between this and the
    // other bitboard
    //
    // @param: other - other bitboard to check for intersection
    // @return: true if there is an intersection between this and the other bitboard, false otherwise
    #[inline(always)]
    pub fn intersects(&self, other: Bitboard) -> bool {
        (self & other).not_empty()
    }

    // remove_at removes the piece at the given square
    //
    // @param: square - square to remove the piece from
    // @return: void
    // @side-effects: modifies the `bitboard`
    #[inline(always)]
    pub fn remove_at(&mut self, square: Square) {
        self.0 ^= Bitboard::square(square).0;
    }

    // set_at sets the piece at the given square
    //
    // @param: square - square to set the piece on
    // @return: void
    // @side-effects: modifies the `bitboard`
    #[inline(always)]
    pub fn set_at(&mut self, square: Square) {
        self.0 |= Bitboard::square(square).0;
    }

    // has_square checks if the given square is set in the bitboard
    //
    // @param: square - square to check
    // @return: true if the given square is set in the bitboard, false otherwise
    #[inline(always)]
    pub fn has_square(&self, square: Square) -> bool {
        self.intersects(Bitboard::square(square))
    }

    // in_between checks if the given square is on the "between" line of s1 and
    // s2, again as noted in `Bitboard::between`, excludes s1 and includes s2
    //
    // @param: s1 - first square forming the "between" line to check
    // @param: s2 - second square forming the "between" line to check
    // @param: square - square to check
    // @return: true if the given square is between s1 and s2, false otherwise
    #[inline(always)]
    pub fn in_between(s1: Square, s2: Square, square: Square) -> bool {
        Bitboard::between(s1, s2).has_square(square)
    }

    // in_line checks if the given square is on the edge to edge line which
    // intersects s1 and s2
    //
    // @param: s1 - first square along the line to check
    // @param: s2 - second square along the line to check
    // @param: square - square to check
    // @return: true if the given square is on the edge to edge line which
    //          intersects s1 and s2, false otherwise
    #[inline(always)]
    pub fn in_line(s1: Square, s2: Square, square: Square) -> bool {
        Bitboard::line(s1, s2).has_square(square)
    }
}
