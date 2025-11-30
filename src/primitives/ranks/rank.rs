use crate::primitives::Rank;

impl Rank {
    // inc increments the rank by one
    //
    // Note: this function is unsafe because it calls `from_idx`
    //
    // @param: self - immutable reference to the rank
    // @return: rank incremented by one
    #[inline(always)]
    pub fn inc(&mut self) {
        *self = Self::from_idx(self.idx() + 1);
    }

    // dec decrements the rank by one
    //
    // Note: this function is unsafe because it calls `from_idx`
    //
    // @param: self - immutable reference to the rank
    // @return: rank decremented by one
    #[inline(always)]
    pub fn dec(&mut self) {
        *self = Self::from_idx(self.idx() - 1);
    }
}
