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

    // is_empty checks if the bitboard is empty
    //
    // @param: self - immutable reference to the bitboard
    // @return: true if the bitboard is empty, false otherwise
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }
}
