use crate::board::fen::{FENError, Parser};
use crate::primitives::{Square, Squares};
use std::ops::RangeInclusive;

const EP_SQUARES_WHITE: RangeInclusive<Square> = Squares::A3..=Squares::H3;
const EP_SQUARES_BLACK: RangeInclusive<Square> = Squares::A6..=Squares::H6;

pub struct EnPassantParser {
    pub square: Option<Square>,
}

impl Parser for EnPassantParser {
    fn parse(segment: &str) -> Result<Self, FENError> {
        match segment.len() {
            1 => match segment.chars().next().unwrap() {
                '-' => Ok(Self { square: None }),
                _ => Err(FENError::InvalidEnPassant),
            },
            2 => {
                let square = Square::try_from(segment);
                match square {
                    Ok(square) => {
                        if EP_SQUARES_WHITE.contains(&square) || EP_SQUARES_BLACK.contains(&square)
                        {
                            Ok(Self {
                                square: Some(square),
                            })
                        } else {
                            Err(FENError::InvalidEnPassant)
                        }
                    }
                    Err(_) => Err(FENError::InvalidEnPassant),
                }
            }
            _ => Err(FENError::InvalidEnPassant),
        }
    }
}
