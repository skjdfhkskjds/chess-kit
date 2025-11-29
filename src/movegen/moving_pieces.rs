use crate::movegen::MoveGenerator;
use crate::primitives::{
    BITBOARD_FILES, BITBOARD_RANKS, BITBOARD_SQUARES, Bitboard, Files, Ranks, Side, Square, Squares,
};

impl MoveGenerator {
    // init_king_table initializes the king move table
    //
    // @param: self - mutable reference to the move generator
    // @return: void
    #[rustfmt::skip]
    pub fn init_king_table(&mut self) {
        for sq in Squares::ALL {
            let bitboard = BITBOARD_SQUARES[sq.unwrap()];
            let moves = ((bitboard & !BITBOARD_FILES[Files::A] & !BITBOARD_RANKS[Ranks::R8]) << 7u8)
                | ((bitboard & !BITBOARD_RANKS[Ranks::R8]) << 8u8)
                | ((bitboard & !BITBOARD_FILES[Files::H] & !BITBOARD_RANKS[Ranks::R8]) << 9u8)
                | ((bitboard & !BITBOARD_FILES[Files::H]) << 1u8)
                | ((bitboard & !BITBOARD_FILES[Files::H] & !BITBOARD_RANKS[Ranks::R1]) >> 7u8)
                | ((bitboard & !BITBOARD_RANKS[Ranks::R1]) >> 8u8)
                | ((bitboard & !BITBOARD_FILES[Files::A] & !BITBOARD_RANKS[Ranks::R1]) >> 9u8)
                | ((bitboard & !BITBOARD_FILES[Files::A]) >> 1u8);
            self.king_table[sq.unwrap()] = moves;
        }
    }

    // init_knight_table initializes the knight move table
    //
    // @param: self - mutable reference to the move generator
    // @return: void
    #[rustfmt::skip]
    pub fn init_knight_table(&mut self) {
        for sq in Squares::ALL {
            let bitboard = BITBOARD_SQUARES[sq.unwrap()];
            let moves =
                ((bitboard & !BITBOARD_RANKS[Ranks::R8] & !BITBOARD_RANKS[Ranks::R7] & !BITBOARD_FILES[Files::A]) << 15u8)
                | ((bitboard & !BITBOARD_RANKS[Ranks::R8] & !BITBOARD_RANKS[Ranks::R7] & !BITBOARD_FILES[Files::H]) << 17u8)
                | ((bitboard & !BITBOARD_FILES[Files::A] & !BITBOARD_FILES[Files::B] & !BITBOARD_RANKS[Ranks::R8]) << 6u8)
                | ((bitboard & !BITBOARD_FILES[Files::G] & !BITBOARD_FILES[Files::H] & !BITBOARD_RANKS[Ranks::R8]) << 10u8)
                | ((bitboard & !BITBOARD_RANKS[Ranks::R1] & !BITBOARD_RANKS[Ranks::R2] & !BITBOARD_FILES[Files::A]) >> 17u8)
                | ((bitboard & !BITBOARD_RANKS[Ranks::R1] & !BITBOARD_RANKS[Ranks::R2] & !BITBOARD_FILES[Files::H]) >> 15u8)
                | ((bitboard & !BITBOARD_FILES[Files::A] & !BITBOARD_FILES[Files::B] & !BITBOARD_RANKS[Ranks::R1]) >> 10u8)
                | ((bitboard & !BITBOARD_FILES[Files::G] & !BITBOARD_FILES[Files::H] & !BITBOARD_RANKS[Ranks::R1]) >> 6u8);
            self.knight_table[sq.unwrap()] = moves;
        }
    }

    // init_pawn_table initializes the pawn move table for each side
    //
    // @param: self - mutable reference to the move generator
    // @return: void
    #[rustfmt::skip]
    pub fn init_pawn_table(&mut self) {
        for sq in Squares::ALL {
            let bitboard = BITBOARD_SQUARES[sq.unwrap()];
            self.pawn_table[Side::White.idx()][sq.unwrap()] = ((bitboard & !BITBOARD_FILES[Files::A]) << 7u8)
                | ((bitboard & !BITBOARD_FILES[Files::H]) << 9u8);
            self.pawn_table[Side::Black.idx()][sq.unwrap()] = ((bitboard & !BITBOARD_FILES[Files::A]) >> 9u8)
                | ((bitboard & !BITBOARD_FILES[Files::H]) >> 7u8);
        }
    }

    // get_king_attacks returns the squares that the king attacks from the given
    // square
    //
    // @param: self - immutable reference to the move generator
    // @param: sq - square that the king is on
    // @return: king moves for the given square
    #[inline(always)]
    pub fn get_king_attacks(&self, sq: Square) -> Bitboard {
        self.king_table[sq.unwrap()]
    }

    // get_knight_attacks returns the squares that the knight attacks from the
    // given square
    //
    // @param: self - immutable reference to the move generator
    // @param: sq - square that the knight is on
    // @return: knight moves for the given square
    #[inline(always)]
    pub fn get_knight_attacks(&self, sq: Square) -> Bitboard {
        self.knight_table[sq.unwrap()]
    }

    // get_pawn_attacks returns the squares that the pawn attacks from the given
    // square for the given side
    //
    // @param: self - immutable reference to the move generator
    // @param: sq - square that the pawn is on
    // @param: side - side to get the pawn moves for
    // @return: pawn moves for the given square
    #[inline(always)]
    pub fn get_pawn_attacks(&self, sq: Square, side: Side) -> Bitboard {
        self.pawn_table[side.idx()][sq.unwrap()]
    }
}
