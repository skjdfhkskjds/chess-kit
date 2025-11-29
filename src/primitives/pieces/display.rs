use crate::primitives::Piece;
use std::fmt;

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece = match self.0 {
            0 => "P",
            1 => "N",
            2 => "B",
            3 => "R",
            4 => "Q",
            5 => "K",
            6 => ".",
            _ => "?",
        };

        match piece {
            "?" => Err(fmt::Error),
            _ => write!(f, "{}", piece),
        }
    }
}
