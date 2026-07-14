use super::DefaultState;
use std::fmt;

impl fmt::Display for DefaultState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} to move", self.header.turn)?;
        writeln!(f, "Castling rights: {}", self.header.castling)?;
        match self.header.en_passant {
            Some(en_passant) => writeln!(f, "En passant square: {}", en_passant)?,
            None => writeln!(f, "En passant square: None")?,
        };
        writeln!(f, "Halfmove clock: {}", self.header.halfmoves)?;
        writeln!(f, "Fullmove clock: {}", self.header.fullmoves)?;
        writeln!(
            f,
            "Repetition distance: {}",
            self.header.draw_state.repetition()
        )?;
        writeln!(
            f,
            "Material draw: {}",
            self.header.draw_state.is_material_draw()
        )?;
        Ok(())
    }
}
