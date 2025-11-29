use crate::primitives::{Castling, Sides};
use std::fmt;

impl fmt::Display for Castling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.kingside(Sides::WHITE) {
            write!(f, "K")?;
        }
        if self.queenside(Sides::WHITE) {
            write!(f, "Q")?;
        }
        if self.kingside(Sides::BLACK) {
            write!(f, "k")?;
        }
        if self.queenside(Sides::BLACK) {
            write!(f, "q")?;
        }
        Ok(())
    }
}
