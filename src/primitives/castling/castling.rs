use crate::primitives::{Castling, CastleRights, SideCastling};

impl Castling {
    // bits returns the bits of the castling rights
    //
    // @return: bits of the castling rights
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.0
    }

    // none returns a castling rights value with no castling rights
    //
    // @return: castling rights value with no castling rights
    #[inline(always)]
    pub const fn none() -> Self {
        Self(CastleRights::None as u8)
    }

    // all returns a castling rights value with all castling rights
    //
    // @return: castling rights value with all castling rights
    #[inline(always)]
    pub const fn all() -> Self {
        Self(CastleRights::All as u8)
    }

    // with_kingside adds the kingside castling rights for the given side
    //
    // @return: a new castling rights value with the kingside rights added
    #[inline]
    pub fn with_kingside<S: SideCastling>(&self) -> Self {
        Self(self.0 | S::KINGSIDE as u8)
    }

    // with_queenside adds the queenside castling rights for the given side
    //
    // @return: a new castling rights value with the queenside rights added
    #[inline]
    pub fn with_queenside<S: SideCastling>(&self) -> Self {
        Self(self.0 | S::QUEENSIDE as u8)
    }

    // revoke revokes the castling rights for the given side
    //
    // @param: side - side to revoke castling rights from
    // @return: a new castling rights value with the castling rights revoked
    #[inline]
    pub fn revoke<S: SideCastling>(&self) -> Self {
        Self(self.0 & !(S::ALL as u8))
    }

    // revoke_kingside revokes the kingside castling rights for the given side
    //
    // @return: a new castling rights value with the kingside rights revoked
    #[inline]
    pub fn revoke_kingside<S: SideCastling>(&self) -> Self {
        Self(self.0 & !(S::KINGSIDE as u8))
    }

    // revoke_queenside revokes the queenside castling rights for the given side
    //
    // @return: a new castling rights value with the queenside rights revoked
    #[inline]
    pub fn revoke_queenside<S: SideCastling>(&self) -> Self {
        Self(self.0 & !(S::QUEENSIDE as u8))
    }

    // can_castle checks if the castling rights allow the given side to castle
    //
    // @return: true if the given side can castle, false otherwise
    #[inline(always)]
    pub fn can_castle<S: SideCastling>(&self) -> bool {
        self.0 & S::ALL as u8 != CastleRights::None as u8
    }

    // kingside checks if the castling rights allow the given side to castle kingside
    //
    // @return: true if the given side can castle kingside, false otherwise
    #[inline(always)]
    pub fn kingside<S: SideCastling>(&self) -> bool {
        self.0 & S::KINGSIDE as u8 != CastleRights::None as u8
    }

    // queenside checks if the castling rights allow the given side to castle queenside
    //
    // @return: true if the given side can castle queenside, false otherwise
    #[inline(always)]
    pub fn queenside<S: SideCastling>(&self) -> bool {
        self.0 & S::QUEENSIDE as u8 != CastleRights::None as u8
    }
}
