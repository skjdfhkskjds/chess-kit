use crate::primitives::File;

impl File {
    // inc increments the file by one
    //
    // Note: this function is unsafe because it calls `from_idx`
    //
    // @param: self - immutable reference to the file
    // @return: file incremented by one
    #[inline(always)]
    pub fn inc(&mut self) {
        *self = Self::from_idx(self.idx() + 1);
    }

    // dec decrements the file by one
    //
    // Note: this function is unsafe because it calls `from_idx`
    //
    // @param: self - immutable reference to the file
    // @return: file decremented by one
    #[inline(always)]
    pub fn dec(&mut self) {
        *self = Self::from_idx(self.idx() - 1);
    }
}
