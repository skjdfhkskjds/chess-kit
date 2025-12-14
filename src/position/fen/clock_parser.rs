use crate::position::fen::{FENError, Parser};
use crate::state::Clock;

pub struct HalfmoveClockParser {
    pub clock: Clock,
}

impl Parser for HalfmoveClockParser {
    fn parse(segment: &str) -> Result<Self, FENError> {
        let halfmove_clock = segment.parse::<Clock>();
        match halfmove_clock {
            Ok(halfmove_clock) => Ok(Self { clock: halfmove_clock }),
            Err(_) => Err(FENError::InvalidHalfmoveCount),
        }
    }
}

pub struct FullmoveClockParser {
    pub clock: Clock,
}

impl Parser for FullmoveClockParser {
    fn parse(segment: &str) -> Result<Self, FENError> {
        let fullmove_clock = segment.parse::<Clock>();
        match fullmove_clock {
            Ok(fullmove_clock) => Ok(Self { clock: fullmove_clock }),
            Err(_) => Err(FENError::InvalidFullmoveCount),
        }
    }
}
