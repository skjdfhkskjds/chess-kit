use crate::primitives::{Side};
use chess_kit_derive::BitOps;

#[repr(u8)]
pub(crate) enum CastleRight {
    None = 0b00000000,
    WhiteKing = 0b00000001,
    WhiteQueen = 0b00000010,
    BlackKing = 0b00000100,
    BlackQueen = 0b00001000,
    White = CastleRight::WhiteKing as u8 | CastleRight::WhiteQueen as u8,
    Black = CastleRight::BlackKing as u8 | CastleRight::BlackQueen as u8,
    All = CastleRight::White as u8 | CastleRight::Black as u8,
}

// Castling rights are stored in a u8 containing the following bits:
//
// | pad  | bq | bk | wq | wk |
// |:----:|:--:|:--:|:--:|:--:|
// | 0101 |  1 |  1 |  1 |  1 |
#[repr(transparent)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, BitOps)]
pub struct Castling(u8);

impl Castling {
    pub const TOTAL: usize = 16;

    pub fn bits(&self) -> u8 {
        self.0
    }

    // none returns a castling rights value with no castling rights
    //
    // @return: castling rights value with no castling rights
    pub const fn none() -> Self {
        Self(CastleRight::None as u8)
    }

    // all returns a castling rights value with all castling rights
    //
    // @return: castling rights value with all castling rights
    pub const fn all() -> Self {
        Self(CastleRight::All as u8)
    }

    // with_kingside adds the kingside castling rights for the given side
    //
    // @param: side - side to add kingside castling rights to
    // @return: a new castling rights value with the kingside rights added
    #[inline]
    pub fn with_kingside(&self, side: Side) -> Self {
        match side {
            Side::White => Self(self.0 | CastleRight::WhiteKing as u8),
            Side::Black => Self(self.0 | CastleRight::BlackKing as u8),
        }
    }

    // with_queenside adds the queenside castling rights for the given side
    //
    // @param: side - side to add queenside castling rights to
    // @return: a new castling rights value with the queenside rights added
    #[inline]
    pub fn with_queenside(&self, side: Side) -> Self {
        match side {
            Side::White => Self(self.0 | CastleRight::WhiteQueen as u8),
            Side::Black => Self(self.0 | CastleRight::BlackQueen as u8),
        }
    }

    // revoke revokes the castling rights for the given side
    //
    // @param: side - side to revoke castling rights from
    // @return: a new castling rights value with the castling rights revoked
    #[inline]
    pub fn revoke(&self, side: Side) -> Self {
        match side {
            Side::White => Self(self.0 & !(CastleRight::White as u8)),
            Side::Black => Self(self.0 & !(CastleRight::Black as u8)),
        }
    }

    // revoke_kingside revokes the kingside castling rights for the given side
    //
    // @param: side - side to revoke kingside castling rights from
    // @return: a new castling rights value with the kingside rights revoked
    #[inline]
    pub fn revoke_kingside(&self, side: Side) -> Self {
        match side {
            Side::White => Self(self.0 & !(CastleRight::WhiteKing as u8)),
            Side::Black => Self(self.0 & !(CastleRight::BlackKing as u8)),
        }
    }

    // revoke_queenside revokes the queenside castling rights for the given side
    //
    // @param: side - side to revoke queenside castling rights from
    // @return: a new castling rights value with the queenside rights revoked
    #[inline]
    pub fn revoke_queenside(&self, side: Side) -> Self {
        match side {
            Side::White => Self(self.0 & !(CastleRight::WhiteQueen as u8)),
            Side::Black => Self(self.0 & !(CastleRight::BlackQueen as u8)),
        }
    }

    // can_castle checks if the castling rights allow the given side to castle
    //
    // @param: side - side to check if can castle
    // @return: true if the given side can castle, false otherwise
    #[inline(always)]
    pub fn can_castle(&self, side: Side) -> bool {
        match side {
            Side::White => self.0 & CastleRight::White as u8 != CastleRight::None as u8,
            Side::Black => self.0 & CastleRight::Black as u8 != CastleRight::None as u8,
        }
    }

    // kingside checks if the castling rights allow the given side to castle kingside
    //
    // @param: side - side to check if can castle kingside
    // @return: true if the given side can castle kingside, false otherwise
    #[inline(always)]
    pub fn kingside(&self, side: Side) -> bool {
        match side {
            Side::White => self.0 & CastleRight::WhiteKing as u8 != CastleRight::None as u8,
            Side::Black => self.0 & CastleRight::BlackKing as u8 != CastleRight::None as u8,
        }
    }

    // queenside checks if the castling rights allow the given side to castle queenside
    //
    // @param: side - side to check if can castle queenside
    // @return: true if the given side can castle queenside, false otherwise
    #[inline(always)]
    pub fn queenside(&self, side: Side) -> bool {
        match side {
            Side::White => self.0 & CastleRight::WhiteQueen as u8 != CastleRight::None as u8,
            Side::Black => self.0 & CastleRight::BlackQueen as u8 != CastleRight::None as u8,
        }
    }
}
