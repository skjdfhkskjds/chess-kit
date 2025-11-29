use crate::primitives::{Bitboard, Square, Side, Pieces};
use crate::movegen::magics::{Magic, ROOK_TABLE_SIZE, BISHOP_TABLE_SIZE};

pub struct MoveGenerator {
    pub(crate) king_table: [Bitboard; Square::TOTAL],
    pub(crate) knight_table: [Bitboard; Square::TOTAL],
    pub(crate) pawn_table: [[Bitboard; Square::TOTAL]; Side::TOTAL],
    pub(crate) bishop_table: Vec<Bitboard>,
    pub(crate) rook_table: Vec<Bitboard>,
    pub(crate) bishop_magics: [Magic; Square::TOTAL],
    pub(crate) rook_magics: [Magic; Square::TOTAL],
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
            king_table: [Bitboard::empty(); Square::TOTAL],
            knight_table: [Bitboard::empty(); Square::TOTAL],
            pawn_table: [[Bitboard::empty(); Square::TOTAL]; Side::TOTAL],
            bishop_table: vec![Bitboard::empty(); BISHOP_TABLE_SIZE],
            rook_table: vec![Bitboard::empty(); ROOK_TABLE_SIZE],
            rook_magics: [magics; Square::TOTAL],
            bishop_magics: [magics; Square::TOTAL],
        };
        mg.init_king_table();
        mg.init_knight_table();
        mg.init_pawn_table();
        mg.init_magics(Pieces::ROOK);
        mg.init_magics(Pieces::BISHOP);
        mg
    }
}
