use crate::castling::SideCastling;
use crate::{Castling, Side};

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

    // with_kingside adds the kingside castling rights for the given side
    //
    // @marker: S - side to add the kingside castling rights for
    // @return: castling rights value with the kingside rights added
    #[inline]
    pub fn with_kingside<S: Side>(&self) -> Self {
        Self(self.0 | SideCastling::KINGSIDE[S::SIDE].0)
    }

    // with_queenside adds the queenside castling rights for the given side
    //
    // @marker: S - side to add the queenside castling rights for
    // @return: castling rights value with the queenside rights added
    #[inline]
    pub fn with_queenside<S: Side>(&self) -> Self {
        Self(self.0 | SideCastling::QUEENSIDE[S::SIDE].0)
    }

    // revoke revokes all the castling rights for the given side
    //
    // @marker: S - side to revoke the castling rights for
    // @return: castling rights value with the castling rights revoked
    #[inline]
    pub fn revoke<S: Side>(&self) -> Self {
        Self(self.0 & !(SideCastling::ALL[S::SIDE].0))
    }

    // revoke_kingside revokes the kingside castling rights for the given side
    //
    // @marker: S - side to revoke the kingside castling rights for
    // @return: castling rights value with the kingside rights revoked
    #[inline]
    pub fn revoke_kingside<S: Side>(&self) -> Self {
        Self(self.0 & !(SideCastling::KINGSIDE[S::SIDE].0))
    }

    // revoke_queenside revokes the queenside castling rights for the given side
    //
    // @marker: S - side to revoke the queenside castling rights for
    // @return: castling rights value with the queenside rights revoked
    #[inline]
    pub fn revoke_queenside<S: Side>(&self) -> Self {
        Self(self.0 & !(SideCastling::QUEENSIDE[S::SIDE].0))
    }

    // can_castle checks if the castling rights allow the given side to castle
    //
    // @marker: S - side to check castling rights for
    // @return: true if the side can castle, false otherwise
    #[inline(always)]
    pub fn can_castle<S: Side>(&self) -> bool {
        (self.0 & SideCastling::ALL[S::SIDE].0) != Castling::NONE.0
    }

    // kingside checks if the castling rights allow the given side to castle
    // kingside
    //
    // @marker: S - side to check kingside castling rights for
    // @return: true if the side can castle kingside, false otherwise
    #[inline(always)]
    pub fn kingside<S: Side>(&self) -> bool {
        (self.0 & SideCastling::KINGSIDE[S::SIDE].0) != Castling::NONE.0
    }

    // queenside checks if the castling rights allow the given side to castle
    // queenside
    //
    // @marker: S - side to check queenside castling rights for
    // @return: true if the side can castle queenside, false otherwise
    #[inline(always)]
    pub fn queenside<S: Side>(&self) -> bool {
        (self.0 & SideCastling::QUEENSIDE[S::SIDE].0) != Castling::NONE.0
    }
}
