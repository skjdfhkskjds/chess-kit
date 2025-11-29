use crate::primitives::Side;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum Rank {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

impl Rank {
    pub const TOTAL: usize = 8;

    // double_step_rank returns the rank that a pawn can double step to
    //
    // @param: side - side to get the double step rank for
    // @return: double step rank for the side
    pub const fn double_step_rank(side: Side) -> Rank {
        match side {
            Side::White => Rank::R4,
            Side::Black => Rank::R5,
        }
    }

    // promotion_rank returns the rank that a pawn can promote to
    //
    // @param: side - side to get the promotion rank for
    // @return: promotion rank for the side
    pub const fn promotion_rank(side: Side) -> Rank {
        match side {
            Side::White => Rank::R8,
            Side::Black => Rank::R1,
        }
    }

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
