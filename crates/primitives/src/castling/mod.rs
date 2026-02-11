mod castling;
mod display;

use chess_kit_derive::BitOps;

// Castling rights are stored in a u8 containing the following bits:
//
// | pad  | bq | bk | wq | wk |
// |:----:|:--:|:--:|:--:|:--:|
// | 0101 |  1 |  1 |  1 |  1 |
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, BitOps)]
pub struct Castling(u8);

impl Castling {
    pub const NONE: Self = Self(0b00000000);
    pub const WHITE_KING: Self = Self(0b00000001);
    pub const WHITE_QUEEN: Self = Self(0b00000010);
    pub const BLACK_KING: Self = Self(0b00000100);
    pub const BLACK_QUEEN: Self = Self(0b00001000);

    pub const WHITE: Self = Self(Self::WHITE_KING.0 | Self::WHITE_QUEEN.0);
    pub const BLACK: Self = Self(Self::BLACK_KING.0 | Self::BLACK_QUEEN.0);
    pub const ALL: Self = Self(Self::WHITE.0 | Self::BLACK.0);

    pub const TOTAL: usize = 16;
}

// SideCastling is a per-side table of castling rights
crate::define_sides! {
    SideCastling: Castling {
        ALL => (Castling::WHITE, Castling::BLACK),
        KINGSIDE => (Castling::WHITE_KING, Castling::BLACK_KING),
        QUEENSIDE => (Castling::WHITE_QUEEN, Castling::BLACK_QUEEN),
    }
}
