use crate::board::fen::{FENError, Parser};
use crate::primitives::{BITBOARD_SQUARES, Bitboard, File, Piece, Rank, Side};

const VALID_PIECES: &str = "kqrbnpKQRBNP";
const DELIMITTER: char = '/';

pub struct PiecesParser {
    pub bitboards: [[Bitboard; Piece::TOTAL]; Side::TOTAL],
}

#[rustfmt::skip]
impl Parser for PiecesParser {
    fn parse(segment: &str) -> Result<Self, FENError> {
        let mut rank = Rank::R8.idx();
        let mut file = File::A.idx();
        let mut bitboards = [[Bitboard::empty(); Piece::TOTAL]; Side::TOTAL];

        for c in segment.chars() {
            let square = (rank * 8) + file;
            match c {
                'k' => bitboards[Side::Black.idx()][Piece::King.idx()] |= BITBOARD_SQUARES[square],
                'q' => bitboards[Side::Black.idx()][Piece::Queen.idx()] |= BITBOARD_SQUARES[square],
                'r' => bitboards[Side::Black.idx()][Piece::Rook.idx()] |= BITBOARD_SQUARES[square],
                'b' => bitboards[Side::Black.idx()][Piece::Bishop.idx()] |= BITBOARD_SQUARES[square],
                'n' => bitboards[Side::Black.idx()][Piece::Knight.idx()] |= BITBOARD_SQUARES[square],
                'p' => bitboards[Side::Black.idx()][Piece::Pawn.idx()] |= BITBOARD_SQUARES[square],
                'K' => bitboards[Side::White.idx()][Piece::King.idx()] |= BITBOARD_SQUARES[square],
                'Q' => bitboards[Side::White.idx()][Piece::Queen.idx()] |= BITBOARD_SQUARES[square],
                'R' => bitboards[Side::White.idx()][Piece::Rook.idx()] |= BITBOARD_SQUARES[square],
                'B' => bitboards[Side::White.idx()][Piece::Bishop.idx()] |= BITBOARD_SQUARES[square],
                'N' => bitboards[Side::White.idx()][Piece::Knight.idx()] |= BITBOARD_SQUARES[square],
                'P' => bitboards[Side::White.idx()][Piece::Pawn.idx()] |= BITBOARD_SQUARES[square],
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
