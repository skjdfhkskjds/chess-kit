use crate::primitives::Piece;
use std::fmt;

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Piece::Pawn => write!(f, "P"),
            Piece::Knight => write!(f, "N"),
            Piece::Bishop => write!(f, "B"),
            Piece::Rook => write!(f, "R"),
            Piece::Queen => write!(f, "Q"),
            Piece::King => write!(f, "K"),
            Piece::None => write!(f, "."),
        }
    }
}
