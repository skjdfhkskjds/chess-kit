use chess_kit_derive::IndexableEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, IndexableEnum)]
#[repr(u8)]
pub enum Side {
    White,
    Black,
}

impl Side {
    pub const TOTAL: usize = 2;

    // other returns the other side
    //
    // @return: other side
    #[inline(always)]
    pub const fn other(self) -> Self {
        unsafe { core::mem::transmute::<u8, Self>(self as u8 ^ 1) }
    }
}
