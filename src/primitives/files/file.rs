use crate::primitives::File;

impl File {
    // inc increments the file by one
    //
    // Note: this function is unsafe because it calls `from_idx`
    //
    // @param: self - immutable reference to the file
    // @return: file incremented by one
    #[inline(always)]
    pub const fn inc(&mut self) {
        *self = Self::from_idx(self.idx() + 1);
    }

    // dec decrements the file by one
    //
    // Note: this function is unsafe because it calls `from_idx`
    //
    // @param: self - immutable reference to the file
    // @return: file decremented by one
    #[inline(always)]
    pub const fn dec(&mut self) {
        *self = Self::from_idx(self.idx() - 1);
    }

    // const_eq is a constant function that checks if the file is equal to the
    // other file
    //
    // @param: self - immutable reference to the file
    // @param: other - immutable reference to the other file
    // @return: true if the file is equal to the other file, false otherwise
    #[inline(always)]
    pub const fn const_eq(self, other: Self) -> bool {
        self.idx() == other.idx()
    }
}
