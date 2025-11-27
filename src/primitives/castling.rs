use chess_kit_derive::{Arithmetic, BitOps};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, BitOps, Arithmetic)]
#[repr(transparent)]
pub struct CastleFlags(u8);

impl CastleFlags {
    pub const NONE: CastleFlags = CastleFlags(0);
    pub const WHITE_KING: CastleFlags = CastleFlags(0b00000001);
    pub const WHITE_QUEEN: CastleFlags = CastleFlags(0b00000010);
    pub const BLACK_KING: CastleFlags = CastleFlags(0b00000100);
    pub const BLACK_QUEEN: CastleFlags = CastleFlags(0b00001000);

    pub const KING: CastleFlags = CastleFlags(Self::WHITE_KING.0 | Self::BLACK_KING.0);
    pub const QUEEN: CastleFlags = CastleFlags(Self::WHITE_QUEEN.0 | Self::BLACK_QUEEN.0);
    pub const WHITE: CastleFlags = CastleFlags(Self::WHITE_KING.0 | Self::WHITE_QUEEN.0);
    pub const BLACK: CastleFlags = CastleFlags(Self::BLACK_KING.0 | Self::BLACK_QUEEN.0);
    pub const ALL: CastleFlags = CastleFlags(Self::WHITE.0 | Self::BLACK.0);
}

// Castling rights are stored in a u8 containing the following bits:
//
// | pad  | bq | bk | wq | wk |
// |:----:|:--:|:--:|:--:|:--:|
// | 0101 |  1 |  1 |  1 |  1 |
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, BitOps)]
pub struct Castling(CastleFlags);

impl Castling {
    pub const TOTAL: usize = 16;

    pub fn bits(&self) -> u8 {
        self.0.0
    }

    pub const fn none() -> Self {
        Self(CastleFlags::NONE)
    }

    pub const fn all() -> Self {
        Self(CastleFlags::ALL)
    }
}

impl From<CastleFlags> for Castling {
    fn from(flags: CastleFlags) -> Self {
        Self(flags)
    }
}

// ================================================
//                    formatting
// ================================================

impl fmt::Display for Castling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if (*self & Castling::from(CastleFlags::WHITE_KING)) != Castling::none() {
            write!(f, "K")?;
        }
        if (*self & Castling::from(CastleFlags::WHITE_QUEEN)) != Castling::none() {
            write!(f, "Q")?;
        }
        if (*self & Castling::from(CastleFlags::BLACK_KING)) != Castling::none() {
            write!(f, "k")?;
        }
        if (*self & Castling::from(CastleFlags::BLACK_QUEEN)) != Castling::none() {
            write!(f, "q")?;
        }
        Ok(())
    }
}
