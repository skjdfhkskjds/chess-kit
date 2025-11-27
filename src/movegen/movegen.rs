use crate::primitives::{Bitboard, Squares, Sides, Pieces};
use crate::movegen::magics::{Magic, ROOK_TABLE_SIZE, BISHOP_TABLE_SIZE};

pub struct MoveGenerator {
    pub(crate) king_moves: [Bitboard; Squares::TOTAL],
    pub(crate) knight_moves: [Bitboard; Squares::TOTAL],
    pub(crate) pawn_moves: [[Bitboard; Squares::TOTAL]; Sides::TOTAL],
    pub(crate) bishop_moves: Vec<Bitboard>,
    pub(crate) rook_moves: Vec<Bitboard>,
    pub(crate) bishop_magics: [Magic; Squares::TOTAL],
    pub(crate) rook_magics: [Magic; Squares::TOTAL],
}

impl Default for MoveGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl MoveGenerator {
    pub fn new() -> Self {
        let magics: Magic = Default::default();
        let mut mg = Self {
            king_moves: [Bitboard::empty(); Squares::TOTAL],
            knight_moves: [Bitboard::empty(); Squares::TOTAL],
            pawn_moves: [[Bitboard::empty(); Squares::TOTAL]; Sides::TOTAL],
            bishop_moves: vec![Bitboard::empty(); BISHOP_TABLE_SIZE],
            rook_moves: vec![Bitboard::empty(); ROOK_TABLE_SIZE],
            rook_magics: [magics; Squares::TOTAL],
            bishop_magics: [magics; Squares::TOTAL],
        };
        mg.init_king_moves();
        mg.init_knight_moves();
        mg.init_pawn_moves();
        mg.init_magics(Pieces::ROOK);
        mg.init_magics(Pieces::BISHOP);
        mg
    }
}
