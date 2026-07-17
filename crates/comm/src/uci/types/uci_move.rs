use std::fmt::{self, Display};
use std::str::FromStr;

use chess_kit_primitives::{Move, Pieces, Square};

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

impl TryFrom<&UciMove> for Move {
    type Error = ParseError;

    fn try_from(value: &UciMove) -> Result<Self, Self::Error> {
        let bytes = value.0.as_bytes();
        if bytes == b"0000" {
            return Err(ParseError::InvalidMove(value.0.clone()));
        }

        let from = Square::try_from(&value.0[0..2])
            .expect("UciMove validation guarantees an on-board source square");
        let to = Square::try_from(&value.0[2..4])
            .expect("UciMove validation guarantees an on-board destination square");
        let mut mv = Move::new(from, to);
        if let Some(suffix) = bytes.get(4) {
            let promoted = match suffix {
                b'n' => Pieces::Knight,
                b'b' => Pieces::Bishop,
                b'r' => Pieces::Rook,
                b'q' => Pieces::Queen,
                _ => unreachable!("UciMove validation guarantees a promotion piece"),
            };
            mv = mv.with_promotion(promoted);
        }
        Ok(mv)
    }
}

impl From<Move> for UciMove {
    fn from(mv: Move) -> Self {
        Self(mv.to_string().to_ascii_lowercase())
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

    #[test]
    fn converts_to_and_from_engine_moves() {
        let uci = "a7a8Q".parse::<UciMove>().unwrap();
        let engine_move = Move::try_from(&uci).unwrap();

        assert_eq!(engine_move.promoted_to(), Pieces::Queen);
        assert_eq!(UciMove::from(engine_move).as_str(), "a7a8q");
        assert!(Move::try_from(&UciMove::null()).is_err());
    }
}
