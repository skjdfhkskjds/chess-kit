use crate::board::fen::{FENError, Parser};
use crate::primitives::{CastleFlags, Castling};

pub struct CastlingParser {
    pub castling: Castling,
}

impl Parser for CastlingParser {
    fn parse(segment: &str) -> Result<Self, FENError> {
        let mut castling = Castling::none();
        if (1..=4).contains(&segment.len()) {
            for c in segment.chars() {
                match c {
                    'K' => castling |= Castling::from(CastleFlags::WHITE_KING),
                    'Q' => castling |= Castling::from(CastleFlags::WHITE_QUEEN),
                    'k' => castling |= Castling::from(CastleFlags::BLACK_KING),
                    'q' => castling |= Castling::from(CastleFlags::BLACK_QUEEN),
                    '-' => (),
                    _ => return Err(FENError::InvalidCastling),
                }
            }
            return Ok(Self { castling });
        }

        Err(FENError::InvalidCastling)
    }
}
