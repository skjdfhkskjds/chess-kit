use crate::{Castling, SideCastling};

impl Castling {
    // none returns a castling rights value with no castling rights
    //
    // @return: castling rights value with no castling rights
    #[inline(always)]
    pub const fn none() -> Self {
        Castling::NONE
    }

    // all returns a castling rights value with all castling rights
    //
    // @return: castling rights value with all castling rights
    #[inline(always)]
    pub const fn all() -> Self {
        Castling::ALL
    }

    // unwrap unwraps the castling rights value into a u8
    //
    // @return: castling rights value as a u8
    #[inline(always)]
    pub const fn unwrap(self) -> u8 {
        self.0
    }

    // with_kingside adds the kingside castling rights for SideT
    //
    // @return: castling rights value with the kingside rights added
    #[inline]
    pub fn with_kingside<S: SideCastling>(&self) -> Self {
        Self(self.0 | S::KINGSIDE.0)
    }

    // with_queenside adds the queenside castling rights for SideT
    //
    // @return: castling rights value with the queenside rights added
    #[inline]
    pub fn with_queenside<S: SideCastling>(&self) -> Self {
        Self(self.0 | S::QUEENSIDE.0)
    }

    // revoke revokes all the castling rights for SideT
    //
    // @return: castling rights value with the castling rights revoked
    #[inline]
    pub fn revoke<S: SideCastling>(&self) -> Self {
        Self(self.0 & !(S::ALL.0))
    }

    // revoke_kingside revokes the kingside castling rights for SideT
    //
    // @return: castling rights value with the kingside rights revoked
    #[inline]
    pub fn revoke_kingside<S: SideCastling>(&self) -> Self {
        Self(self.0 & !(S::KINGSIDE.0))
    }

    // revoke_queenside revokes the queenside castling rights for SideT
    //
    // @return: castling rights value with the queenside rights revoked
    #[inline]
    pub fn revoke_queenside<S: SideCastling>(&self) -> Self {
        Self(self.0 & !(S::QUEENSIDE.0))
    }

    // can_castle checks if the castling rights allow SideT to castle
    //
    // @return: true if SideT can castle, false otherwise
    #[inline(always)]
    pub fn can_castle<S: SideCastling>(&self) -> bool {
        (self.0 & S::ALL.0) != Castling::NONE.0
    }

    // kingside checks if the castling rights allow SideT to castle kingside
    //
    // @return: true if SideT can castle kingside, false otherwise
    #[inline(always)]
    pub fn kingside<S: SideCastling>(&self) -> bool {
        (self.0 & S::KINGSIDE.0) != Castling::NONE.0
    }

    // queenside checks if the castling rights allow SideT to castle queenside
    //
    // @return: true if SideT can castle queenside, false otherwise
    #[inline(always)]
    pub fn queenside<S: SideCastling>(&self) -> bool {
        (self.0 & S::QUEENSIDE.0) != Castling::NONE.0
    }
}
