use bitflags::bitflags;
use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

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
//               bitwise operations
// ================================================

impl BitOr for Castling {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Castling {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for Castling {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Castling {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitXor for Castling {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Castling {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for Castling {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
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
