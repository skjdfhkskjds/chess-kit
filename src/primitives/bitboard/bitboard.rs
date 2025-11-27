pub type BitboardVec = Vec<Bitboard>;

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Default, Hash)]
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

    // is_empty checks if the bitboard is empty
    //
    // @param: self - immutable reference to the bitboard
    // @return: true if the bitboard is empty, false otherwise
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    // count counts the number of bits set in the bitboard
    //
    // @param: self - immutable reference to the bitboard
    // @return: number of bits set in the bitboard
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        self.0.count_ones()
    }

    // trailing_zeros counts the number of trailing zeros in the bitboard
    //
    // @param: self - immutable reference to the bitboard
    // @return: number of trailing zeros in the bitboard
    #[inline(always)]
    pub const fn trailing_zeros(&self) -> u32 {
        self.0.trailing_zeros()
    }
}
