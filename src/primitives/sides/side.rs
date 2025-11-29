#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum Side {
    White,
    Black,
}

impl Side {
    pub const TOTAL: usize = 2;

    // idx returns the side as an index for array access
    //
    // @return: index of the side
    #[inline(always)]
    pub const fn idx(self) -> usize {
        self as usize
    }

    // from_idx creates a side from the given index
    //
    // Note: this is an unsafe operation, see `from_idx_safe` for a safe version
    // 
    // @param: idx - index to create the side from
    // @return: side created from the index
    #[inline(always)]
    pub fn from_idx(idx: usize) -> Self {
        unsafe { core::mem::transmute::<u8, Self>(idx as u8) }
    }

    // from_idx_safe is the same as from_idx, but performs a safety check to
    // ensure the index is valid
    //
    // @param: idx - index to create the side from
    // @return: side created from the index, or None if the index is invalid
    #[inline(always)]
    pub fn from_idx_safe(idx: usize) -> Option<Self> {
        if idx < Self::TOTAL {
            Some(unsafe { core::mem::transmute::<u8, Self>(idx as u8) })
        } else {
            None
        }
    }

    // other returns the other side
    //
    // @return: other side
    #[inline(always)]
    pub const fn other(self) -> Self {
        unsafe { core::mem::transmute::<u8, Self>(self as u8 ^ 1) }
    }
}
