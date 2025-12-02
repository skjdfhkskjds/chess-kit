use crate::primitives::Rank;

impl Rank {
    // inc increments the rank by one
    //
    // Note: this function is unsafe because it calls `from_idx`
    //
    // @param: self - immutable reference to the rank
    // @return: rank incremented by one
    #[inline(always)]
    pub const fn inc(&mut self) {
        *self = Self::from_idx(self.idx() + 1);
    }

    // dec decrements the rank by one
    //
    // Note: this function is unsafe because it calls `from_idx`
    //
    // @param: self - immutable reference to the rank
    // @return: rank decremented by one
    #[inline(always)]
    pub const fn dec(&mut self) {
        *self = Self::from_idx(self.idx() - 1);
    }

    // const_eq is a constant function that checks if the rank is equal to the
    // other rank
    //
    // @param: self - immutable reference to the rank
    // @param: other - immutable reference to the other rank
    // @return: true if the rank is equal to the other rank, false otherwise
    #[inline(always)]
    pub const fn const_eq(self, other: Self) -> bool {
        self.idx() == other.idx()
    }
}
