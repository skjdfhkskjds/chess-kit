use std::fmt::{self, Display};
use std::str::FromStr;

use super::ParseError;

/// `UciMove` is a type that represents a move encoded using UCI long algebraic
/// notation
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UciMove(String);

impl UciMove {
    /// null returns the UCI null move used when there is no legal move
    ///
    /// @return: UCI null move
    pub fn null() -> Self {
        Self("0000".to_owned())
    }

    /// as_str returns the normalized long algebraic move notation
    ///
    /// @return: normalized UCI move string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for UciMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for UciMove {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        // UCI move notation is case-insensitive, but normalized moves use a
        // lowercase promotion suffix
        let value = value.to_ascii_lowercase();
        if value == "0000" {
            return Ok(Self(value));
        }

        // normal moves contain source and destination squares followed by an
        // optional queen, rook, bishop, or knight promotion suffix
        let bytes = value.as_bytes();
        let valid_square =
            |file: u8, rank: u8| (b'a'..=b'h').contains(&file) && (b'1'..=b'8').contains(&rank);
        let valid = matches!(bytes.len(), 4 | 5)
            && valid_square(bytes[0], bytes[1])
            && valid_square(bytes[2], bytes[3])
            && (bytes.len() == 4 || matches!(bytes[4], b'q' | b'r' | b'b' | b'n'));

        if valid {
            Ok(Self(value))
        } else {
            Err(ParseError::InvalidMove(value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_and_normalizes_uci_moves() {
        assert_eq!("e7e8Q".parse::<UciMove>().unwrap().as_str(), "e7e8q");
        assert!("e2e9".parse::<UciMove>().is_err());
        assert!("e7e8k".parse::<UciMove>().is_err());
    }
}
