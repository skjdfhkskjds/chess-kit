use chess_kit_primitives::Move;
use std::fmt::{self, Display};

/// PlayError is returned when attempting to play an illegal move
///
/// PlayError preserves the rejected move so callers of [`crate::PositionMoves::play`]
/// can inspect or report the failed operation
///
/// @type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayError {
    IllegalMove(Move),
}

impl Display for PlayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IllegalMove(mv) => write!(f, "illegal move: {mv}"),
        }
    }
}

impl std::error::Error for PlayError {}
