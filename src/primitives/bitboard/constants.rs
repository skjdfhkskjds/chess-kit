use crate::primitives::bitboard::Bitboard;
use crate::primitives::{Rank, File, Square};

pub const BITBOARD_RANKS: [Bitboard; Rank::TOTAL] = {
    const RANK_1: u64 = 0xFF;
    let mut ranks = [Bitboard::empty(); Rank::TOTAL];
    let mut i = 0;

    // Note: while loop hack to get around const fn loop limitations
    while i < Rank::TOTAL {
        ranks[i] = Bitboard::new(RANK_1 << (i * 8));
        i += 1;
    }

    ranks
};

pub const BITBOARD_FILES: [Bitboard; File::TOTAL] = {
    const FILE_A: u64 = 0x0101_0101_0101_0101;
    let mut files = [Bitboard::empty(); File::TOTAL];
    let mut i = 0;

    // Note: while loop hack to get around const fn loop limitations
    while i < File::TOTAL {
        files[i] = Bitboard::new(FILE_A << i);
        i += 1;
    }

    files
};

pub const BITBOARD_SQUARES: [Bitboard; Square::TOTAL] = {
    let mut squares = [Bitboard::empty(); Square::TOTAL];
    let mut i = 0;

    // Note: while loop hack to get around const fn loop limitations
    while i < Square::TOTAL {
        squares[i] = Bitboard::new(1 << i);
        i += 1;
    }

    squares
};
