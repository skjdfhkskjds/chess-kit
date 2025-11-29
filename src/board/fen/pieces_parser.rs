use crate::board::fen::{FENError, Parser};
use crate::primitives::{BITBOARD_SQUARES, Bitboard, Files, Pieces, Ranks, Sides};

const VALID_PIECES: &str = "kqrbnpKQRBNP";
const DELIMITTER: char = '/';

pub struct PiecesParser {
    pub bitboards: [[Bitboard; Pieces::TOTAL]; Sides::TOTAL],
}

impl Parser for PiecesParser {
    fn parse(segment: &str) -> Result<Self, FENError> {
        let mut rank = Ranks::R8;
        let mut file = Files::A;
        let mut bitboards = [[Bitboard::empty(); Pieces::TOTAL]; Sides::TOTAL];

        for c in segment.chars() {
            let square = (rank * 8) + file;
            match c {
                'k' => bitboards[Sides::BLACK][Pieces::KING.unwrap()] |= BITBOARD_SQUARES[square],
                'q' => bitboards[Sides::BLACK][Pieces::QUEEN.unwrap()] |= BITBOARD_SQUARES[square],
                'r' => bitboards[Sides::BLACK][Pieces::ROOK.unwrap()] |= BITBOARD_SQUARES[square],
                'b' => bitboards[Sides::BLACK][Pieces::BISHOP.unwrap()] |= BITBOARD_SQUARES[square],
                'n' => bitboards[Sides::BLACK][Pieces::KNIGHT.unwrap()] |= BITBOARD_SQUARES[square],
                'p' => bitboards[Sides::BLACK][Pieces::PAWN.unwrap()] |= BITBOARD_SQUARES[square],
                'K' => bitboards[Sides::WHITE][Pieces::KING.unwrap()] |= BITBOARD_SQUARES[square],
                'Q' => bitboards[Sides::WHITE][Pieces::QUEEN.unwrap()] |= BITBOARD_SQUARES[square],
                'R' => bitboards[Sides::WHITE][Pieces::ROOK.unwrap()] |= BITBOARD_SQUARES[square],
                'B' => bitboards[Sides::WHITE][Pieces::BISHOP.unwrap()] |= BITBOARD_SQUARES[square],
                'N' => bitboards[Sides::WHITE][Pieces::KNIGHT.unwrap()] |= BITBOARD_SQUARES[square],
                'P' => bitboards[Sides::WHITE][Pieces::PAWN.unwrap()] |= BITBOARD_SQUARES[square],
                '1'..='8' => {
                    if let Some(offset) = c.to_digit(10) {
                        file += offset as usize;
                    }
                }
                DELIMITTER => {
                    if file != 8 {
                        return Err(FENError::InvalidPieces);
                    }
                    rank -= 1;
                    file = 0;
                }
                _ => return Err(FENError::InvalidPieces),
            }

            // in the default case of a piece being found, advance to the next file
            if VALID_PIECES.contains(c) {
                file += 1;
            }
        }

        Ok(Self { bitboards })
    }
}
