use crate::board::fen::{FENError, Parser};
use crate::primitives::{Sides, Side};

pub struct TurnParser {
    pub turn: Side,
}

impl Parser for TurnParser {
    fn parse(segment: &str) -> Result<Self, FENError> {
        if segment.len() != 1 {
            return Err(FENError::InvalidTurn);
        }

        let turn = match segment.chars().next().unwrap() {
            'w' => Sides::WHITE,
            'b' => Sides::BLACK,
            _ => return Err(FENError::InvalidTurn),
        };

        return Ok(Self { turn });
    }
}
