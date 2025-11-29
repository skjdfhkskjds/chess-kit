use crate::movegen::MoveGenerator;
use crate::primitives::{
    BITBOARD_FILES, BITBOARD_RANKS, BITBOARD_SQUARES, Bitboard, File, Rank, Side, Square,
};

impl MoveGenerator {
    // init_king_table initializes the king move table
    //
    // @param: self - mutable reference to the move generator
    // @return: void
    #[rustfmt::skip]
    pub(crate) fn init_king_table(&mut self) {
        for sq in Square::ALL {
            let bitboard = BITBOARD_SQUARES[sq.idx()];
            let moves = ((bitboard & !BITBOARD_FILES[File::A.idx()] & !BITBOARD_RANKS[Rank::R8.idx()]) << 7u8)
                | ((bitboard & !BITBOARD_RANKS[Rank::R8.idx()]) << 8u8)
                | ((bitboard & !BITBOARD_FILES[File::H.idx()] & !BITBOARD_RANKS[Rank::R8.idx()]) << 9u8)
                | ((bitboard & !BITBOARD_FILES[File::H.idx()]) << 1u8)
                | ((bitboard & !BITBOARD_FILES[File::H.idx()] & !BITBOARD_RANKS[Rank::R1.idx()]) >> 7u8)
                | ((bitboard & !BITBOARD_RANKS[Rank::R1.idx()]) >> 8u8)
                | ((bitboard & !BITBOARD_FILES[File::A.idx()] & !BITBOARD_RANKS[Rank::R1.idx()]) >> 9u8)
                | ((bitboard & !BITBOARD_FILES[File::A.idx()]) >> 1u8);
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
            let bitboard = BITBOARD_SQUARES[sq.idx()];
            let moves =
                ((bitboard & !BITBOARD_RANKS[Rank::R8.idx()] & !BITBOARD_RANKS[Rank::R7.idx()] & !BITBOARD_FILES[File::A.idx()]) << 15u8)
                | ((bitboard & !BITBOARD_RANKS[Rank::R8.idx()] & !BITBOARD_RANKS[Rank::R7.idx()] & !BITBOARD_FILES[File::H.idx()]) << 17u8)
                | ((bitboard & !BITBOARD_FILES[File::A.idx()] & !BITBOARD_FILES[File::B.idx()] & !BITBOARD_RANKS[Rank::R8.idx()]) << 6u8)
                | ((bitboard & !BITBOARD_FILES[File::G.idx()] & !BITBOARD_FILES[File::H.idx()] & !BITBOARD_RANKS[Rank::R8.idx()]) << 10u8)
                | ((bitboard & !BITBOARD_RANKS[Rank::R1.idx()] & !BITBOARD_RANKS[Rank::R2.idx()] & !BITBOARD_FILES[File::A.idx()]) >> 17u8)
                | ((bitboard & !BITBOARD_RANKS[Rank::R1.idx()] & !BITBOARD_RANKS[Rank::R2.idx()] & !BITBOARD_FILES[File::H.idx()]) >> 15u8)
                | ((bitboard & !BITBOARD_FILES[File::A.idx()] & !BITBOARD_FILES[File::B.idx()] & !BITBOARD_RANKS[Rank::R1.idx()]) >> 10u8)
                | ((bitboard & !BITBOARD_FILES[File::G.idx()] & !BITBOARD_FILES[File::H.idx()] & !BITBOARD_RANKS[Rank::R1.idx()]) >> 6u8);
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
            let bitboard = BITBOARD_SQUARES[sq.idx()];
            self.pawn_table[Side::White.idx()][sq.idx()] = ((bitboard & !BITBOARD_FILES[File::A.idx()]) << 7u8)
                | ((bitboard & !BITBOARD_FILES[File::H.idx()]) << 9u8);
            self.pawn_table[Side::Black.idx()][sq.idx()] = ((bitboard & !BITBOARD_FILES[File::A.idx()]) >> 9u8)
                | ((bitboard & !BITBOARD_FILES[File::H.idx()]) >> 7u8);
        }
    }

    // get_king_targets returns the squares that the king targets from the given
    // square
    //
    // @param: self - immutable reference to the move generator
    // @param: sq - square that the king is on
    // @return: king targets from the given square
    #[inline(always)]
    pub(crate) fn get_king_targets(&self, sq: Square) -> Bitboard {
        self.king_table[sq.idx()]
    }

    // get_knight_targets returns the squares that the knight targets from the
    // given square
    //
    // @param: self - immutable reference to the move generator
    // @param: sq - square that the knight is on
    // @return: knight targets from the given square
    #[inline(always)]
    pub(crate) fn get_knight_targets(&self, sq: Square) -> Bitboard {
        self.knight_table[sq.idx()]
    }

    // get_pawn_targets returns the squares that the pawn targets from the given
    // square for the given side
    //
    // @param: self - immutable reference to the move generator
    // @param: sq - square that the pawn is on
    // @param: side - side to get the pawn moves for
    // @return: pawn targets from the given square
    #[inline(always)]
    pub(crate) fn get_pawn_targets(&self, sq: Square, side: Side) -> Bitboard {
        self.pawn_table[side.idx()][sq.idx()]
    }
}
