use crate::{Black, Pieces, Side, White};
use std::{fmt, marker::PhantomData};

/// `PieceDisplay` formats a piece using the Unicode symbol for `SideT`.
pub struct PieceDisplay<SideT: Side> {
    piece: Pieces,
    _side: PhantomData<SideT>,
}

impl<SideT: Side> PieceDisplay<SideT> {
    pub const NONE: &'static str = ".";
}

impl PieceDisplay<White> {
    pub const PAWN: &'static str = "\u{2659}"; // ♙
    pub const KNIGHT: &'static str = "\u{2658}"; // ♘
    pub const BISHOP: &'static str = "\u{2657}"; // ♗
    pub const ROOK: &'static str = "\u{2656}"; // ♖
    pub const QUEEN: &'static str = "\u{2655}"; // ♕
    pub const KING: &'static str = "\u{2654}"; // ♔

    pub const ALL: [&'static str; Pieces::TOTAL - 1] = [
        Self::PAWN,
        Self::KNIGHT,
        Self::BISHOP,
        Self::ROOK,
        Self::QUEEN,
        Self::KING,
    ];
}

impl PieceDisplay<Black> {
    pub const PAWN: &'static str = "\u{265f}"; // ♟
    pub const KNIGHT: &'static str = "\u{265e}"; // ♞
    pub const BISHOP: &'static str = "\u{265d}"; // ♝
    pub const ROOK: &'static str = "\u{265c}"; // ♜
    pub const QUEEN: &'static str = "\u{265b}"; // ♛
    pub const KING: &'static str = "\u{265a}"; // ♚

    pub const ALL: [&'static str; Pieces::TOTAL - 1] = [
        Self::PAWN,
        Self::KNIGHT,
        Self::BISHOP,
        Self::ROOK,
        Self::QUEEN,
        Self::KING,
    ];
}

impl Pieces {
    /// display returns a side-aware Unicode display adapter for this piece.
    ///
    /// @marker: SideT - side whose symbol should be displayed
    /// @return: side-aware piece display adapter
    pub const fn display<SideT: Side>(self) -> PieceDisplay<SideT> {
        PieceDisplay {
            piece: self,
            _side: PhantomData,
        }
    }
}

impl fmt::Display for Pieces {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pieces::Pawn => write!(f, "P"),
            Pieces::Knight => write!(f, "N"),
            Pieces::Bishop => write!(f, "B"),
            Pieces::Rook => write!(f, "R"),
            Pieces::Queen => write!(f, "Q"),
            Pieces::King => write!(f, "K"),
            Pieces::None => write!(f, "."),
        }
    }
}

impl fmt::Display for PieceDisplay<White> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.piece {
            Pieces::Pawn => Self::PAWN,
            Pieces::Knight => Self::KNIGHT,
            Pieces::Bishop => Self::BISHOP,
            Pieces::Rook => Self::ROOK,
            Pieces::Queen => Self::QUEEN,
            Pieces::King => Self::KING,
            Pieces::None => Self::NONE,
        })
    }
}

impl fmt::Display for PieceDisplay<Black> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.piece {
            Pieces::Pawn => Self::PAWN,
            Pieces::Knight => Self::KNIGHT,
            Pieces::Bishop => Self::BISHOP,
            Pieces::Rook => Self::ROOK,
            Pieces::Queen => Self::QUEEN,
            Pieces::King => Self::KING,
            Pieces::None => Self::NONE,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displays_white_unicode_pieces() {
        let displayed = Pieces::ALL
            .map(|piece| piece.display::<White>().to_string())
            .join("");

        assert_eq!(displayed, PieceDisplay::<White>::ALL.join(""));
        assert_eq!(
            Pieces::None.display::<White>().to_string(),
            PieceDisplay::<White>::NONE
        );
    }

    #[test]
    fn displays_black_unicode_pieces() {
        let displayed = Pieces::ALL
            .map(|piece| piece.display::<Black>().to_string())
            .join("");

        assert_eq!(displayed, PieceDisplay::<Black>::ALL.join(""));
        assert_eq!(
            Pieces::None.display::<Black>().to_string(),
            PieceDisplay::<Black>::NONE
        );
    }
}
