use crate::table::BitboardTable;
use chess_kit_primitives::{Bitboard, File, Rank, Sides, Square};

pub(crate) const NOT_A_FILE: Bitboard = Bitboard::new(!Bitboard::file(File::A).const_unwrap());
pub(crate) const NOT_H_FILE: Bitboard = Bitboard::new(!Bitboard::file(File::H).const_unwrap());
pub(crate) const NOT_R8_RANK: Bitboard = Bitboard::new(!Bitboard::rank(Rank::R8).const_unwrap());
pub(crate) const NOT_R1_RANK: Bitboard = Bitboard::new(!Bitboard::rank(Rank::R1).const_unwrap());

/// new_king_table creates a new king move table
///
/// @return: new king move table
#[rustfmt::skip]
pub(crate) const fn new_king_table() -> BitboardTable {
    let mut king_table: BitboardTable = [Bitboard::empty(); Square::TOTAL];

    let not_a_file = NOT_A_FILE.const_unwrap();
    let not_h_file = NOT_H_FILE.const_unwrap();
    let not_r1_rank = NOT_R1_RANK.const_unwrap();
    let not_r8_rank = NOT_R8_RANK.const_unwrap();

    let mut sq = 0;
    while sq < Square::TOTAL {
        let square = Bitboard::square(Square::from_idx(sq)).const_unwrap();
        let moves = ((square & not_a_file & not_r8_rank) << 7u8)
            | ((square & not_r8_rank) << 8u8)
            | ((square & not_h_file & not_r8_rank) << 9u8)
            | ((square & not_h_file) << 1u8)
            | ((square & not_h_file & not_r1_rank) >> 7u8)
            | ((square & not_r1_rank) >> 8u8)
            | ((square & not_a_file & not_r1_rank) >> 9u8)
            | ((square & not_a_file) >> 1u8);

        
        king_table[sq] = Bitboard::new(moves);
        sq += 1;
    }

    king_table
}

/// new_knight_table creates a new knight move table
///
/// @return: new knight move table
#[rustfmt::skip]
pub(crate) const fn new_knight_table() -> BitboardTable {
    let mut knight_table: BitboardTable = [Bitboard::empty(); Square::TOTAL];

    // variables to "try" (unsuccessfully) to improve readability
    let not_r8_rank = NOT_R8_RANK.const_unwrap();
    let not_r7_rank = !Bitboard::rank(Rank::R7).const_unwrap();
    let not_r2_rank = !Bitboard::rank(Rank::R2).const_unwrap();
    let not_r1_rank = NOT_R1_RANK.const_unwrap();
    let not_a_file = NOT_A_FILE.const_unwrap();
    let not_b_file = !Bitboard::file(File::B).const_unwrap();
    let not_g_file = !Bitboard::file(File::G).const_unwrap();
    let not_h_file = NOT_H_FILE.const_unwrap();
    
    let mut sq = 0;
    while sq < Square::TOTAL {
        let square = Bitboard::square(Square::from_idx(sq)).const_unwrap();
        let moves =
            ((square & not_r8_rank & not_r7_rank & not_a_file) << 15u8)
            | ((square & not_r8_rank & not_r7_rank & not_h_file) << 17u8)
            | ((square & not_a_file & not_b_file & not_r8_rank) << 6u8)
            | ((square & not_g_file & not_h_file & not_r8_rank) << 10u8)
            | ((square & not_r1_rank & not_r2_rank & not_a_file) >> 17u8)
            | ((square & not_r1_rank & not_r2_rank & not_h_file) >> 15u8)
            | ((square & not_a_file & not_b_file & not_r1_rank) >> 10u8)
            | ((square & not_g_file & not_h_file & not_r1_rank) >> 6u8);
        
        knight_table[sq] = Bitboard::new(moves);
        sq += 1;
    }

    knight_table
}

/// new_pawn_table creates a new pawn move table for each side
///
/// @return: new pawn move table for each side
#[rustfmt::skip]
pub(crate) const fn new_pawn_table() -> [BitboardTable; Sides::TOTAL] {
    let mut pawn_table: [BitboardTable; Sides::TOTAL] = [[Bitboard::empty(); Square::TOTAL]; Sides::TOTAL];

    let not_a_file = NOT_A_FILE.const_unwrap();
    let not_h_file = NOT_H_FILE.const_unwrap();

    let mut sq = 0;
    while sq < Square::TOTAL {
        let square = Bitboard::square(Square::from_idx(sq)).const_unwrap();
        pawn_table[Sides::White.idx()][sq] = Bitboard::new(((square & not_a_file) << 7u8)
            | ((square & not_h_file) << 9u8));
        pawn_table[Sides::Black.idx()][sq] = Bitboard::new(((square & not_a_file) >> 9u8)
            | ((square & not_h_file) >> 7u8));
        sq += 1;
    }

    pawn_table
}
