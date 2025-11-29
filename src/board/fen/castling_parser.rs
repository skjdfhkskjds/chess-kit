use crate::board::fen::{FENError, Parser};
use crate::primitives::{Castling, Sides};

pub struct CastlingParser {
    pub castling: Castling,
}

impl Parser for CastlingParser {
    fn parse(segment: &str) -> Result<Self, FENError> {
        if !(1..=4).contains(&segment.len()) {
            return Err(FENError::InvalidCastling);
        }

        let mut castling = Castling::none();
        for c in segment.chars() {
            match c {
                'K' => castling = castling.with_kingside(Sides::WHITE),
                'Q' => castling = castling.with_queenside(Sides::WHITE),
                'k' => castling = castling.with_kingside(Sides::BLACK),
                'q' => castling = castling.with_queenside(Sides::BLACK),
                '-' => (),
                _ => return Err(FENError::InvalidCastling),
            }
        }

        Ok(Self { castling })
    }
}
