use crate::primitives::{Castling, Side};
use std::fmt;

impl fmt::Display for Castling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.kingside(Side::White) {
            write!(f, "K")?;
        }
        if self.queenside(Side::White) {
            write!(f, "Q")?;
        }
        if self.kingside(Side::Black) {
            write!(f, "k")?;
        }
        if self.queenside(Side::Black) {
            write!(f, "q")?;
        }
        Ok(())
    }
}
