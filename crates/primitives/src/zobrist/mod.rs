mod constants;
mod table;

pub use constants::{CASTLING_RANDOMS, EN_PASSANT_RANDOMS, PIECE_RANDOMS, SIDE_RANDOMS};
pub use table::ZobristTable;

use chess_kit_derive::BitOps;
use std::fmt::{self, Display};
use std::num::ParseIntError;

/// `ZobristKey` is type-safe representation of a zobrist key
///
/// @type
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, BitOps)]
pub struct ZobristKey(u64);

impl ZobristKey {
    /// new creates a new zobrist key with the given u64 value
    ///
    /// @param: value - u64 value to create the zobrist key from
    /// @return: new zobrist key
    #[inline(always)]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// default creates a new zobrist key with the default value
    ///
    /// @return: new zobrist key
    #[inline(always)]
    pub const fn default() -> Self {
        Self(0)
    }
}

impl Display for ZobristKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl TryFrom<&str> for ZobristKey {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self::new(u64::from_str_radix(value, 16)?))
    }
}
