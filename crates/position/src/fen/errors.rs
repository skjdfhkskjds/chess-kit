use std::fmt::{self, Display};

/// `FENError` is an enum that represents the errors that can occur when parsing a FEN string
///
/// @type
#[derive(Debug)]
pub enum FENError {
    InvalidFormat,        // the FEN string must be composed of 6 segments
    InvalidPieces,        // the first segment must contain the pieces and squares
    InvalidTurn,          // the second segment must contain the turn to move
    InvalidCastling,      // the third segment must contain the castling rights
    InvalidEnPassant,     // the fourth segment must contain the en passant square
    InvalidHalfmoveCount, // the fifth segment must contain the halfmove count
    InvalidFullmoveCount, // the sixth segment must contain the fullmove count
}

impl Display for FENError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = match self {
            Self::InvalidFormat => "Must be 6 segments",
            Self::InvalidPieces => "Invalid pieces or squares in the first segment",
            Self::InvalidTurn => "Invalid turn to move in the second segment",
            Self::InvalidCastling => "Invalid castling rights in the third segment",
            Self::InvalidEnPassant => "Invalid en passant square in the fourth segment",
            Self::InvalidHalfmoveCount => "Invalid halfmove count in the fifth segment",
            Self::InvalidFullmoveCount => "Invalid fullmove count in the sixth segment",
        };
        write!(f, "Error in FEN string: {error}")
    }
}
