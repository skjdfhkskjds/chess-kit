use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    #[repr(transparent)]
    pub struct CastleFlags: u8 {
        const NONE = 0;
        const WHITE_KING = 0b00000001;
        const WHITE_QUEEN = 0b00000010;
        const BLACK_KING = 0b00000100;
        const BLACK_QUEEN = 0b00001000;

        const KING  = Self::WHITE_KING.bits()  | Self::BLACK_KING.bits();
        const QUEEN = Self::WHITE_QUEEN.bits() | Self::BLACK_QUEEN.bits();
        const WHITE = Self::WHITE_KING.bits()  | Self::WHITE_QUEEN.bits();
        const BLACK = Self::BLACK_KING.bits()  | Self::BLACK_QUEEN.bits();
        const ALL   = Self::WHITE.bits() | Self::BLACK.bits();
    }
}

// Castling rights are stored in a u8 containing the following bits:
//
// | pad  | bq | bk | wq | wk |
// |:----:|:--:|:--:|:--:|:--:|
// | 0101 |  1 |  1 |  1 |  1 |
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Castling(CastleFlags);

impl Castling {
    pub const TOTAL: usize = 16;

    pub fn bits(&self) -> u8 {
        self.0.bits()
    }

    pub fn none() -> Self {
        Self(CastleFlags::NONE)
    }

    pub fn all() -> Self {
        Self(CastleFlags::ALL)
    }
}
