use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct Castling: u8 {
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
pub struct CastleRights(u8);

impl CastleRights {
    fn empty() -> Self {
        Self(Castling::NONE.bits())
    }

    fn all() -> Self {
        Self(Castling::ALL.bits())
    }
}
