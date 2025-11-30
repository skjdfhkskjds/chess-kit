use crate::primitives::{File, Rank, Square};
use std::fmt;

impl TryFrom<&str> for Square {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() != 2 {
            return Err("invalid square");
        }

        let file_str = &s[0..1];
        let rank_str = &s[1..2];

        let file = File::try_from(file_str)?;
        let rank = Rank::try_from(rank_str)?;

        Ok(Square::new(file, rank))
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}
