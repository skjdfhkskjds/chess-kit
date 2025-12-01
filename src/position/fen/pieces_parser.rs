use crate::position::fen::{FENError, Parser};
use crate::primitives::{BITBOARD_SQUARES, Bitboard, File, Pieces, Rank, Sides};

const VALID_PIECES: &str = "kqrbnpKQRBNP";
const DELIMITTER: char = '/';

pub struct PiecesParser {
    pub bitboards: [[Bitboard; Pieces::TOTAL]; Sides::TOTAL],
}

#[rustfmt::skip]
impl Parser for PiecesParser {
    fn parse(segment: &str) -> Result<Self, FENError> {
        let mut rank = Rank::R8.idx();
        let mut file = File::A.idx();
        let mut bitboards = [[Bitboard::empty(); Pieces::TOTAL]; Sides::TOTAL];

        for c in segment.chars() {
            let square = (rank * 8) + file;
            match c {
                'k' => bitboards[Sides::Black.idx()][Pieces::King.idx()] |= BITBOARD_SQUARES[square],
                'q' => bitboards[Sides::Black.idx()][Pieces::Queen.idx()] |= BITBOARD_SQUARES[square],
                'r' => bitboards[Sides::Black.idx()][Pieces::Rook.idx()] |= BITBOARD_SQUARES[square],
                'b' => bitboards[Sides::Black.idx()][Pieces::Bishop.idx()] |= BITBOARD_SQUARES[square],
                'n' => bitboards[Sides::Black.idx()][Pieces::Knight.idx()] |= BITBOARD_SQUARES[square],
                'p' => bitboards[Sides::Black.idx()][Pieces::Pawn.idx()] |= BITBOARD_SQUARES[square],
                'K' => bitboards[Sides::White.idx()][Pieces::King.idx()] |= BITBOARD_SQUARES[square],
                'Q' => bitboards[Sides::White.idx()][Pieces::Queen.idx()] |= BITBOARD_SQUARES[square],
                'R' => bitboards[Sides::White.idx()][Pieces::Rook.idx()] |= BITBOARD_SQUARES[square],
                'B' => bitboards[Sides::White.idx()][Pieces::Bishop.idx()] |= BITBOARD_SQUARES[square],
                'N' => bitboards[Sides::White.idx()][Pieces::Knight.idx()] |= BITBOARD_SQUARES[square],
                'P' => bitboards[Sides::White.idx()][Pieces::Pawn.idx()] |= BITBOARD_SQUARES[square],
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
