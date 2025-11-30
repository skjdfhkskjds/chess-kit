use crate::board::fen::{FENError, Parser};
use crate::primitives::{Castling, White, Black};

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
                'K' => castling = castling.with_kingside::<White>(),
                'Q' => castling = castling.with_queenside::<White>(),
                'k' => castling = castling.with_kingside::<Black>(),
                'q' => castling = castling.with_queenside::<Black>(),
                '-' => (),
                _ => return Err(FENError::InvalidCastling),
            }
        }

        Ok(Self { castling })
    }
}
