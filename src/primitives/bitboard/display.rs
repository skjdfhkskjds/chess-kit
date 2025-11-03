use crate::primitives::bitboard::Bitboard;
use std::fmt;

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const LAST_BIT: u64 = 63;
        for rank in 0..8 {
            for file in (0..8).rev() {
                let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
                let char = if self.0 & mask != 0 { '1' } else { '0' };
                write!(f, "{char} ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
