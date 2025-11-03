use crate::board::state::State;
use crate::primitives::Sides;
use std::fmt;

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} to move", match self.turn {
            Sides::WHITE => "White",
            Sides::BLACK => "Black",
            _ => return Err(fmt::Error),
        })?;
        writeln!(f, "Castling rights: {}", self.castling)?;
        if let Some(en_passant) = self.en_passant {
            writeln!(f, "En passant square: {}", en_passant)?;
        } else {
            writeln!(f, "En passant square: None")?;
        }
        writeln!(f, "Halfmove clock: {}", self.halfmoves)?;
        writeln!(f, "Fullmove clock: {}", self.fullmoves)?;
        Ok(())
    }
}
