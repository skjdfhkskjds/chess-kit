use crate::attack_table::DefaultAttackTable;
use crate::primitives::{Bitboard, File, Rank, Sides, Square};

pub(crate) const NOT_A_FILE: Bitboard = Bitboard::new(!Bitboard::file(File::A).const_unwrap());
pub(crate) const NOT_H_FILE: Bitboard = Bitboard::new(!Bitboard::file(File::H).const_unwrap());
pub(crate) const NOT_R8_RANK: Bitboard = Bitboard::new(!Bitboard::rank(Rank::R8).const_unwrap());
pub(crate) const NOT_R1_RANK: Bitboard = Bitboard::new(!Bitboard::rank(Rank::R1).const_unwrap());

impl DefaultAttackTable {
    // init_king_table initializes the king move table
    //
    // @return: void
    #[rustfmt::skip]
    pub(crate) fn init_king_table(&mut self) {
        for sq in Square::ALL {
            let square = Bitboard::square(sq);
            let moves = ((square & NOT_A_FILE & NOT_R8_RANK) << 7u8)
                | ((square & NOT_R8_RANK) << 8u8)
                | ((square & NOT_H_FILE & NOT_R8_RANK) << 9u8)
                | ((square & NOT_H_FILE) << 1u8)
                | ((square & NOT_H_FILE & NOT_R1_RANK) >> 7u8)
                | ((square & NOT_R1_RANK) >> 8u8)
                | ((square & NOT_A_FILE & NOT_R1_RANK) >> 9u8)
                | ((square & NOT_A_FILE) >> 1u8);
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
                ((square & NOT_R8_RANK & !Bitboard::rank(Rank::R7) & NOT_A_FILE) << 15u8)
                | ((square & NOT_R8_RANK & !Bitboard::rank(Rank::R7) & NOT_H_FILE) << 17u8)
                | ((square & NOT_A_FILE & !Bitboard::file(File::B) & NOT_R8_RANK) << 6u8)
                | ((square & !Bitboard::file(File::G) & NOT_H_FILE & NOT_R8_RANK) << 10u8)
                | ((square & NOT_R1_RANK & !Bitboard::rank(Rank::R2) & NOT_A_FILE) >> 17u8)
                | ((square & NOT_R1_RANK & !Bitboard::rank(Rank::R2) & NOT_H_FILE) >> 15u8)
                | ((square & NOT_A_FILE & !Bitboard::file(File::B) & NOT_R1_RANK) >> 10u8)
                | ((square & !Bitboard::file(File::G) & NOT_H_FILE & NOT_R1_RANK) >> 6u8);
            self.knight_table[sq.idx()] = moves;
        }
    }

    // init_pawn_table initializes the pawn move table for each side
    //
    // @param: self - mutable reference to the move generator
    // @return: void
    #[rustfmt::skip]
    pub(crate) fn init_pawn_table(&mut self) {
        for sq in Square::ALL {
            let square = Bitboard::square(sq);
            // generate the pawn attacks
            self.pawn_table[Sides::White.idx()][sq.idx()] = ((square & NOT_A_FILE) << 7u8)
                | ((square & NOT_H_FILE) << 9u8);
            self.pawn_table[Sides::Black.idx()][sq.idx()] = ((square & NOT_A_FILE) >> 9u8)
                | ((square & NOT_H_FILE) >> 7u8);
        }
    }
}
