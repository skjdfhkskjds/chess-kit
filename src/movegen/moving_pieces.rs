use crate::movegen::MoveGenerator;
use crate::primitives::{
    BITBOARD_FILES, BITBOARD_RANKS, BITBOARD_SQUARES, Files, Ranks, Sides, Squares,
};

impl MoveGenerator {
    #[rustfmt::skip]
    pub fn init_king_moves(&mut self) {
        for sq in Squares::ALL {
            let bb_square = BITBOARD_SQUARES[sq.unwrap()];
            let bb_moves = ((bb_square & !BITBOARD_FILES[Files::A] & !BITBOARD_RANKS[Ranks::R8]) << 7)
                | ((bb_square & !BITBOARD_RANKS[Ranks::R8]) << 8)
                | ((bb_square & !BITBOARD_FILES[Files::H] & !BITBOARD_RANKS[Ranks::R8]) << 9)
                | ((bb_square & !BITBOARD_FILES[Files::H]) << 1)
                | ((bb_square & !BITBOARD_FILES[Files::H] & !BITBOARD_RANKS[Ranks::R1]) >> 7)
                | ((bb_square & !BITBOARD_RANKS[Ranks::R1]) >> 8)
                | ((bb_square & !BITBOARD_FILES[Files::A] & !BITBOARD_RANKS[Ranks::R1]) >> 9)
                | ((bb_square & !BITBOARD_FILES[Files::A]) >> 1);
            self.king_moves[sq.unwrap()] = bb_moves;
        }
    }

    #[rustfmt::skip]
    pub fn init_knight_moves(&mut self) {
        for sq in Squares::ALL {
            let bb_square = BITBOARD_SQUARES[sq.unwrap()];
            let bb_moves =
                ((bb_square & !BITBOARD_RANKS[Ranks::R8] & !BITBOARD_RANKS[Ranks::R7] & !BITBOARD_FILES[Files::A]) << 15)
                | ((bb_square & !BITBOARD_RANKS[Ranks::R8] & !BITBOARD_RANKS[Ranks::R7] & !BITBOARD_FILES[Files::H]) << 17)
                | ((bb_square & !BITBOARD_FILES[Files::A] & !BITBOARD_FILES[Files::B] & !BITBOARD_RANKS[Ranks::R8]) << 6)
                | ((bb_square & !BITBOARD_FILES[Files::G] & !BITBOARD_FILES[Files::H] & !BITBOARD_RANKS[Ranks::R8]) << 10)
                | ((bb_square & !BITBOARD_RANKS[Ranks::R1] & !BITBOARD_RANKS[Ranks::R2] & !BITBOARD_FILES[Files::A]) >> 17)
                | ((bb_square & !BITBOARD_RANKS[Ranks::R1] & !BITBOARD_RANKS[Ranks::R2] & !BITBOARD_FILES[Files::H]) >> 15)
                | ((bb_square & !BITBOARD_FILES[Files::A] & !BITBOARD_FILES[Files::B] & !BITBOARD_RANKS[Ranks::R1]) >> 10)
                | ((bb_square & !BITBOARD_FILES[Files::G] & !BITBOARD_FILES[Files::H] & !BITBOARD_RANKS[Ranks::R1]) >> 6);
            self.knight_moves[sq.unwrap()] = bb_moves;
        }
    }

    #[rustfmt::skip]
    pub fn init_pawn_moves(&mut self) {
        for sq in Squares::ALL {
            let bb_square = BITBOARD_SQUARES[sq.unwrap()];
            self.pawn_moves[Sides::WHITE][sq.unwrap()] = ((bb_square & !BITBOARD_FILES[Files::A]) << 7)
                | ((bb_square & !BITBOARD_FILES[Files::H]) << 9);
            self.pawn_moves[Sides::BLACK][sq.unwrap()] = ((bb_square & !BITBOARD_FILES[Files::A]) >> 9)
                | ((bb_square & !BITBOARD_FILES[Files::H]) >> 7);
        }
    }
}
