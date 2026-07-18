use std::fmt::{self, Display};
use std::str::FromStr;

use chess_kit_primitives::{Move, Pieces, Square};

use super::ParseError;

/// `UciMove` is a type that represents a move encoded using UCI long algebraic
/// notation
///
/// @type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UciMove {
    /// UCI's protocol-level null move representation.
    Null,
    /// A parsed primitive move.
    Move(Move),
}

impl UciMove {
    /// null returns the UCI null move used when there is no legal move
    ///
    /// @return: UCI null move
    pub const fn null() -> Self {
        Self::Null
    }
}

impl Display for UciMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => f.write_str("0000"),
            Self::Move(mv) => {
                write!(f, "{}{}", mv.from(), mv.to())?;
                if mv.type_of() == chess_kit_primitives::MoveType::Promotion {
                    let suffix = match mv.promoted_to() {
                        Pieces::Knight => 'n',
                        Pieces::Bishop => 'b',
                        Pieces::Rook => 'r',
                        Pieces::Queen => 'q',
                        _ => unreachable!("primitive promotion moves use promotable pieces"),
                    };
                    write!(f, "{suffix}")?;
                }
                Ok(())
            }
        }
    }
}

impl FromStr for UciMove {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        // UCI move notation is case-insensitive, while Display normalizes the
        // promotion suffix to lowercase.
        if value == "0000" {
            return Ok(Self::Null);
        }

        // Byte slicing is safe after the ASCII and length checks.
        let bytes = value.as_bytes();
        if !value.is_ascii() || !matches!(bytes.len(), 4 | 5) {
            return Err(ParseError::InvalidMove(value.to_owned()));
        }

        let from = Square::try_from(&value[0..2])
            .map_err(|_| ParseError::InvalidMove(value.to_owned()))?;
        let to = Square::try_from(&value[2..4])
            .map_err(|_| ParseError::InvalidMove(value.to_owned()))?;
        let mut mv = Move::new(from, to);
        if let Some(suffix) = bytes.get(4).map(u8::to_ascii_lowercase) {
            let promoted = match suffix {
                b'n' => Pieces::Knight,
                b'b' => Pieces::Bishop,
                b'r' => Pieces::Rook,
                b'q' => Pieces::Queen,
                _ => return Err(ParseError::InvalidMove(value.to_owned())),
            };
            mv = mv.with_promotion(promoted);
        }
        Ok(Self::Move(mv))
    }
}

impl TryFrom<&UciMove> for Move {
    type Error = ParseError;

    fn try_from(value: &UciMove) -> Result<Self, Self::Error> {
        match value {
            UciMove::Null => Err(ParseError::InvalidMove("0000".to_owned())),
            UciMove::Move(mv) => Ok(*mv),
        }
    }
}

impl From<Move> for UciMove {
    fn from(mv: Move) -> Self {
        Self::Move(mv)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_and_normalizes_uci_moves() {
        assert_eq!("e7e8Q".parse::<UciMove>().unwrap().to_string(), "e7e8q");
        assert!("e2e9".parse::<UciMove>().is_err());
        assert!("e7e8k".parse::<UciMove>().is_err());
    }

    #[test]
    fn converts_to_and_from_engine_moves() {
        let uci = "a7a8Q".parse::<UciMove>().unwrap();
        let engine_move = Move::try_from(&uci).unwrap();

        assert_eq!(engine_move.promoted_to(), Pieces::Queen);
        assert_eq!(UciMove::from(engine_move).to_string(), "a7a8q");
        assert!(Move::try_from(&UciMove::null()).is_err());
    }
}
