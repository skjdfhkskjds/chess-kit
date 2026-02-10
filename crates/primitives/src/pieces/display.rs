use crate::Pieces;
use std::fmt;

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
