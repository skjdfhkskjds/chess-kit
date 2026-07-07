use crate::{File, Square, bitboard::Bitboard};
use std::fmt;

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for square in Square::ALL {
            let char = if self.has_square(square) { '1' } else { '0' };
            write!(f, "{char} ")?;

            if square.file() == File::H {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
