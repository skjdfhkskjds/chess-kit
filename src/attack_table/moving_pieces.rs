use crate::attack_table::DefaultAttackTable;
use crate::primitives::{Bitboard, File, Rank, Side, Square};

impl DefaultAttackTable {
    // init_king_table initializes the king move table
    //
    // @return: void
    #[rustfmt::skip]
    pub(crate) fn init_king_table(&mut self) {
        for sq in Square::ALL {
            let square = Bitboard::square(sq);
            let moves = ((square & !Bitboard::file(File::A) & !Bitboard::rank(Rank::R8)) << 7u8)
                | ((square & !Bitboard::rank(Rank::R8)) << 8u8)
                | ((square & !Bitboard::file(File::H) & !Bitboard::rank(Rank::R8)) << 9u8)
                | ((square & !Bitboard::file(File::H)) << 1u8)
                | ((square & !Bitboard::file(File::H) & !Bitboard::rank(Rank::R1)) >> 7u8)
                | ((square & !Bitboard::rank(Rank::R1)) >> 8u8)
                | ((square & !Bitboard::file(File::A) & !Bitboard::rank(Rank::R1)) >> 9u8)
                | ((square & !Bitboard::file(File::A)) >> 1u8);
            self.king_table[sq.idx()] = moves;
        }
    }

    // init_knight_table initializes the knight move table
    //
    // @param: self - mutable reference to the move generator
    // @return: void
    #[rustfmt::skip]
    pub(crate) fn init_knight_table(&mut self) {
        for sq in Square::ALL {
            let square = Bitboard::square(sq);
            let moves =
                ((square & !Bitboard::rank(Rank::R8) & !Bitboard::rank(Rank::R7) & !Bitboard::file(File::A)) << 15u8)
                | ((square & !Bitboard::rank(Rank::R8) & !Bitboard::rank(Rank::R7) & !Bitboard::file(File::H)) << 17u8)
                | ((square & !Bitboard::file(File::A) & !Bitboard::file(File::B) & !Bitboard::rank(Rank::R8)) << 6u8)
                | ((square & !Bitboard::file(File::G) & !Bitboard::file(File::H) & !Bitboard::rank(Rank::R8)) << 10u8)
                | ((square & !Bitboard::rank(Rank::R1) & !Bitboard::rank(Rank::R2) & !Bitboard::file(File::A)) >> 17u8)
                | ((square & !Bitboard::rank(Rank::R1) & !Bitboard::rank(Rank::R2) & !Bitboard::file(File::H)) >> 15u8)
                | ((square & !Bitboard::file(File::A) & !Bitboard::file(File::B) & !Bitboard::rank(Rank::R1)) >> 10u8)
                | ((square & !Bitboard::file(File::G) & !Bitboard::file(File::H) & !Bitboard::rank(Rank::R1)) >> 6u8);
            self.knight_table[sq.idx()] = moves;
        }
    }

    // init_pawn_table initializes the pawn move table for each side
    //
    // @param: self - mutable reference to the move generator
    // @return: void
    #[rustfmt::skip]
    pub(crate) fn init_pawn_table<S: Side>(&mut self) {
        for sq in Square::ALL {
            let square = Bitboard::square(sq);
            self.pawn_table[S::INDEX][sq.idx()] = ((square & !Bitboard::file(File::A)) << 7u8)
                | ((square & !Bitboard::file(File::H)) << 9u8);
            self.pawn_table[S::Other::INDEX][sq.idx()] = ((square & !Bitboard::file(File::A)) >> 9u8)
                | ((square & !Bitboard::file(File::H)) >> 7u8);
        }
    }
}
