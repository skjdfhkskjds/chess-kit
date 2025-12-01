use crate::position::fen::FENError;
use crate::position::fen::{
    CastlingParser, EnPassantParser, FullmoveClockParser, HalfmoveClockParser, PiecesParser,
    TurnParser,
};

pub trait Parser: Sized {
    fn parse(segment: &str) -> Result<Self, FENError>;
}

pub struct FENParser {
    pub pieces: PiecesParser,
    pub turn: TurnParser,
    pub castling: CastlingParser,
    pub en_passant: EnPassantParser,
    pub halfmove_parser: HalfmoveClockParser,
    pub fullmove_parser: FullmoveClockParser,
}

impl Parser for FENParser {
    fn parse(segment: &str) -> Result<Self, FENError> {
        let segments = segment.split(' ').collect::<Vec<&str>>();
        if segments.len() != 6 {
            return Err(FENError::InvalidFormat);
        }

        let pieces = PiecesParser::parse(segments[0])?;
        let turn = TurnParser::parse(segments[1])?;
        let castling = CastlingParser::parse(segments[2])?;
        let en_passant = EnPassantParser::parse(segments[3])?;
        let halfmove_parser = HalfmoveClockParser::parse(segments[4])?;
        let fullmove_parser = FullmoveClockParser::parse(segments[5])?;

        Ok(Self {
            pieces,
            turn,
            castling,
            en_passant,
            halfmove_parser,
            fullmove_parser,
        })
    }
}
