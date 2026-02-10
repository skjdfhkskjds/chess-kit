mod castling;
mod display;

use crate::{Black, Side, White};
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

// `SideCastling` is a trait that defines constants for the castling rights of
// a given side
//
// @trait
pub trait SideCastling: Side {
    // ALL is the combined castling rights for the given side
    const ALL: Castling;

    // KINGSIDE is the kingside castling rights for the given side
    const KINGSIDE: Castling;

    // QUEENSIDE is the queenside castling rights for the given side
    const QUEENSIDE: Castling;
}

impl SideCastling for White {
    const ALL: Castling = Castling::WHITE;
    const KINGSIDE: Castling = Castling::WHITE_KING;
    const QUEENSIDE: Castling = Castling::WHITE_QUEEN;
}

impl SideCastling for Black {
    const ALL: Castling = Castling::BLACK;
    const KINGSIDE: Castling = Castling::BLACK_KING;
    const QUEENSIDE: Castling = Castling::BLACK_QUEEN;
}
