use crate::primitives::{Bitboard, Squares, Sides, Pieces};
use crate::movegen::magics::{Magic, ROOK_TABLE_SIZE, BISHOP_TABLE_SIZE};

pub struct MoveGenerator {
    pub(crate) king_table: [Bitboard; Squares::TOTAL],
    pub(crate) knight_table: [Bitboard; Squares::TOTAL],
    pub(crate) pawn_table: [[Bitboard; Squares::TOTAL]; Sides::TOTAL],
    pub(crate) bishop_table: Vec<Bitboard>,
    pub(crate) rook_table: Vec<Bitboard>,
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
            king_table: [Bitboard::empty(); Squares::TOTAL],
            knight_table: [Bitboard::empty(); Squares::TOTAL],
            pawn_table: [[Bitboard::empty(); Squares::TOTAL]; Sides::TOTAL],
            bishop_table: vec![Bitboard::empty(); BISHOP_TABLE_SIZE],
            rook_table: vec![Bitboard::empty(); ROOK_TABLE_SIZE],
            rook_magics: [magics; Squares::TOTAL],
            bishop_magics: [magics; Squares::TOTAL],
        };
        mg.init_king_table();
        mg.init_knight_table();
        mg.init_pawn_table();
        mg.init_magics(Pieces::ROOK);
        mg.init_magics(Pieces::BISHOP);
        mg
    }
}
