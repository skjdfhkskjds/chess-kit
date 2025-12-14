mod constants;
mod table;

pub use constants::{CASTLING_RANDOMS, EN_PASSANT_RANDOMS, PIECE_RANDOMS, SIDE_RANDOMS};

use chess_kit_derive::BitOps;

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, BitOps)]
pub struct ZobristKey(u64);

impl ZobristKey {
    // new creates a new zobrist key with the given u64 value
    //
    // @param: value - u64 value to create the zobrist key from
    // @return: new zobrist key
    #[inline(always)]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    // default creates a new zobrist key with the default value
    //
    // @return: new zobrist key
    #[inline(always)]
    pub const fn default() -> Self {
        Self(0)
    }
}

// ZobristTable is a collection of random values used to generate/apply a zobrist
// key transformations for a given board position.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZobristTable {}
