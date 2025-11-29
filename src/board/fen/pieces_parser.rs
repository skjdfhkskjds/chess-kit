use crate::board::fen::{FENError, Parser};
use crate::primitives::{BITBOARD_SQUARES, Bitboard, File, Pieces, Rank, Side};

const VALID_PIECES: &str = "kqrbnpKQRBNP";
const DELIMITTER: char = '/';

pub struct PiecesParser {
    pub bitboards: [[Bitboard; Pieces::TOTAL]; Side::TOTAL],
}

impl Parser for PiecesParser {
    fn parse(segment: &str) -> Result<Self, FENError> {
        let mut rank = Rank::R8.idx();
        let mut file = File::A.idx();
        let mut bitboards = [[Bitboard::empty(); Pieces::TOTAL]; Side::TOTAL];

        for c in segment.chars() {
            let square = (rank * 8) + file;
            match c {
                'k' => bitboards[Side::Black.idx()][Pieces::KING.unwrap()] |= BITBOARD_SQUARES[square],
                'q' => bitboards[Side::Black.idx()][Pieces::QUEEN.unwrap()] |= BITBOARD_SQUARES[square],
                'r' => bitboards[Side::Black.idx()][Pieces::ROOK.unwrap()] |= BITBOARD_SQUARES[square],
                'b' => bitboards[Side::Black.idx()][Pieces::BISHOP.unwrap()] |= BITBOARD_SQUARES[square],
                'n' => bitboards[Side::Black.idx()][Pieces::KNIGHT.unwrap()] |= BITBOARD_SQUARES[square],
                'p' => bitboards[Side::Black.idx()][Pieces::PAWN.unwrap()] |= BITBOARD_SQUARES[square],
                'K' => bitboards[Side::White.idx()][Pieces::KING.unwrap()] |= BITBOARD_SQUARES[square],
                'Q' => bitboards[Side::White.idx()][Pieces::QUEEN.unwrap()] |= BITBOARD_SQUARES[square],
                'R' => bitboards[Side::White.idx()][Pieces::ROOK.unwrap()] |= BITBOARD_SQUARES[square],
                'B' => bitboards[Side::White.idx()][Pieces::BISHOP.unwrap()] |= BITBOARD_SQUARES[square],
                'N' => bitboards[Side::White.idx()][Pieces::KNIGHT.unwrap()] |= BITBOARD_SQUARES[square],
                'P' => bitboards[Side::White.idx()][Pieces::PAWN.unwrap()] |= BITBOARD_SQUARES[square],
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
