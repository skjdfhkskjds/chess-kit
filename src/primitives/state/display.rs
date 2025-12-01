use crate::primitives::State;
use std::fmt;

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} to move", self.turn)?;
        writeln!(f, "Castling rights: {}", self.castling)?;
        match self.en_passant {
            Some(en_passant) => writeln!(f, "En passant square: {}", en_passant)?,
            None => writeln!(f, "En passant square: None")?,
        };
        writeln!(f, "Halfmove clock: {}", self.halfmoves)?;
        writeln!(f, "Fullmove clock: {}", self.fullmoves)?;
        Ok(())
    }
}
