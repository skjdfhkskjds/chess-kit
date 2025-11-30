mod castling;
mod display;

use crate::primitives::{Black, Side, White};
use chess_kit_derive::BitOps;

#[repr(u8)]
pub enum CastleRights {
    None = 0b00000000,
    WhiteKing = 0b00000001,
    WhiteQueen = 0b00000010,
    BlackKing = 0b00000100,
    BlackQueen = 0b00001000,

    White = CastleRights::WhiteKing as u8 | CastleRights::WhiteQueen as u8,
    Black = CastleRights::BlackKing as u8 | CastleRights::BlackQueen as u8,
    All = CastleRights::White as u8 | CastleRights::Black as u8,
}

impl CastleRights {
    pub const TOTAL: usize = 16;
}

// Castling rights are stored in a u8 containing the following bits:
//
// | pad  | bq | bk | wq | wk |
// |:----:|:--:|:--:|:--:|:--:|
// | 0101 |  1 |  1 |  1 |  1 |
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, BitOps)]
pub struct Castling(u8);

// `SideCastling` is a trait that defines constants for the castling rights of
// a given side
// 
// @trait
pub trait SideCastling: Side {
    // ALL is the combined castling rights for the given side
    const ALL: CastleRights;

    // KINGSIDE is the kingside castling rights for the given side
    const KINGSIDE: CastleRights;

    // QUEENSIDE is the queenside castling rights for the given side
    const QUEENSIDE: CastleRights;
}

impl SideCastling for White {
    const ALL: CastleRights = CastleRights::White;
    const KINGSIDE: CastleRights = CastleRights::WhiteKing;
    const QUEENSIDE: CastleRights = CastleRights::WhiteQueen;
}

impl SideCastling for Black {
    const ALL: CastleRights = CastleRights::Black;
    const KINGSIDE: CastleRights = CastleRights::BlackKing;
    const QUEENSIDE: CastleRights = CastleRights::BlackQueen;
}
