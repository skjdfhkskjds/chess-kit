use crate::fen::{FENError, Parser};
use chess_kit_primitives::{Bitboard, File, Pieces, Rank, Sides, Square};

const VALID_PIECES: &str = "kqrbnpKQRBNP";
const DELIMITTER: char = '/';

pub struct PiecesParser {
    pub bitboards: [[Bitboard; Pieces::TOTAL]; Sides::TOTAL],
}

impl Parser for PiecesParser {
    fn parse(segment: &str) -> Result<Self, FENError> {
        let mut rank = Rank::R8.idx();
        let mut file = File::A.idx();
        let mut bitboards = [[Bitboard::empty(); Pieces::TOTAL]; Sides::TOTAL];

        for c in segment.chars() {
            match c {
                'k' => {
                    bitboards[Sides::Black][Pieces::King] |=
                        Bitboard::square(Square::new(File::from_idx(file), Rank::from_idx(rank)))
                }
                'q' => {
                    bitboards[Sides::Black][Pieces::Queen] |=
                        Bitboard::square(Square::new(File::from_idx(file), Rank::from_idx(rank)))
                }
                'r' => {
                    bitboards[Sides::Black][Pieces::Rook] |=
                        Bitboard::square(Square::new(File::from_idx(file), Rank::from_idx(rank)))
                }
                'b' => {
                    bitboards[Sides::Black][Pieces::Bishop] |=
                        Bitboard::square(Square::new(File::from_idx(file), Rank::from_idx(rank)))
                }
                'n' => {
                    bitboards[Sides::Black][Pieces::Knight] |=
                        Bitboard::square(Square::new(File::from_idx(file), Rank::from_idx(rank)))
                }
                'p' => {
                    bitboards[Sides::Black][Pieces::Pawn] |=
                        Bitboard::square(Square::new(File::from_idx(file), Rank::from_idx(rank)))
                }
                'K' => {
                    bitboards[Sides::White][Pieces::King] |=
                        Bitboard::square(Square::new(File::from_idx(file), Rank::from_idx(rank)))
                }
                'Q' => {
                    bitboards[Sides::White][Pieces::Queen] |=
                        Bitboard::square(Square::new(File::from_idx(file), Rank::from_idx(rank)))
                }
                'R' => {
                    bitboards[Sides::White][Pieces::Rook] |=
                        Bitboard::square(Square::new(File::from_idx(file), Rank::from_idx(rank)))
                }
                'B' => {
                    bitboards[Sides::White][Pieces::Bishop] |=
                        Bitboard::square(Square::new(File::from_idx(file), Rank::from_idx(rank)))
                }
                'N' => {
                    bitboards[Sides::White][Pieces::Knight] |=
                        Bitboard::square(Square::new(File::from_idx(file), Rank::from_idx(rank)))
                }
                'P' => {
                    bitboards[Sides::White][Pieces::Pawn] |=
                        Bitboard::square(Square::new(File::from_idx(file), Rank::from_idx(rank)))
                }
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
