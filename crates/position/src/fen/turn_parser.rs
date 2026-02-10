use crate::fen::{FENError, Parser};
use chess_kit_primitives::Sides;

pub struct TurnParser {
    pub turn: Sides,
}

impl Parser for TurnParser {
    fn parse(segment: &str) -> Result<Self, FENError> {
        if segment.len() != 1 {
            return Err(FENError::InvalidTurn);
        }

        let turn = match segment.chars().next().unwrap() {
            'w' => Sides::White,
            'b' => Sides::Black,
            _ => return Err(FENError::InvalidTurn),
        };

        return Ok(Self { turn });
    }
}
