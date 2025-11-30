use crate::primitives::{Black, Castling, White};
use std::fmt;

impl fmt::Display for Castling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.kingside::<White>() {
            write!(f, "K")?;
        }
        if self.queenside::<White>() {
            write!(f, "Q")?;
        }
        if self.kingside::<Black>() {
            write!(f, "k")?;
        }
        if self.queenside::<Black>() {
            write!(f, "q")?;
        }
        Ok(())
    }
}
