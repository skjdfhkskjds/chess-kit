use crate::board::fen::{FENError, Parser};

pub struct HalfmoveCountParser {
    pub halfmove_count: u16,
}

impl Parser for HalfmoveCountParser {
    fn parse(segment: &str) -> Result<Self, FENError> {
        let halfmove_count = segment.parse::<u16>();
        match halfmove_count {
            Ok(halfmove_count) => Ok(Self { halfmove_count }),
            Err(_) => Err(FENError::InvalidHalfmoveCount),
        }
    }
}

pub struct FullmoveCountParser {
    pub fullmove_count: u8,
}

impl Parser for FullmoveCountParser {
    fn parse(segment: &str) -> Result<Self, FENError> {
        let fullmove_count = segment.parse::<u8>();
        match fullmove_count {
            Ok(fullmove_count) => Ok(Self { fullmove_count }),
            Err(_) => Err(FENError::InvalidFullmoveCount),
        }
    }
}
