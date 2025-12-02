use crate::primitives::bitboard::Bitboard;
use crate::primitives::{Rank, File, Square};
use crate::attack_table::{DefaultAttackTable, Direction};

// BITBOARD_RANKS is a constant array of bitboards, where each bitboard is has
// the bits for that rank set to 1
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

// BITBOARD_FILES is a constant array of bitboards, where each bitboard is has
// the bits for that file set to 1
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

// BITBOARD_SQUARES is a constant array of bitboards, where each bitboard is has
// the bits for that square set to 1
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

pub const BITBOARD_BETWEEN: [[Bitboard; Square::TOTAL]; Square::TOTAL] = {
    let mut between = [[Bitboard::empty(); Square::TOTAL]; Square::TOTAL];
    let mut i = 0;

    // for each square pair [i, j], set the between bitboard to be the result
    // of the attack ray from j with the occupancy bitboard set to the bitboard
    // square for i
    while i < Square::TOTAL {
        let source = Square::from_idx(i);
        let source_rank = source.rank().idx();
        let source_file = source.file().idx();

        let mut j = 0;
        while j < Square::TOTAL {
            let target = Square::from_idx(j);
            let target_rank = target.rank().idx();
            let target_file = target.file().idx();

            let same_rank = target_rank == source_rank;
            let same_file = target_file == source_file;
            let same_diagonal = (target_file as isize - source_file as isize).abs()
                == (target_rank as isize - source_rank as isize).abs();

            // we only want to compute the between bitboard for pairs of squares
            // that are
            //
            // 1. not the same square
            // 2. on the same rank, file, or diagonal
            //
            // otherwise, the entry should just contain the end square's bitboard
            if i == j || !(same_rank || same_file || same_diagonal) {
                between[i][j] = BITBOARD_SQUARES[j];
                j += 1;
                continue;
            }

            // determine the direction of the attack ray
            let direction = if target_rank < source_rank && target_file < source_file {
                Direction::DownLeft
            } else if target_rank < source_rank && target_file > source_file {
                Direction::DownRight
            } else if target_rank > source_rank && target_file < source_file {
                Direction::UpLeft
            } else if target_rank > source_rank && target_file > source_file {
                Direction::UpRight
            } else if target_rank < source_rank {
                Direction::Down
            } else if target_rank > source_rank {
                Direction::Up
            } else if target_file < source_file {
                Direction::Left
            } else {
                Direction::Right
            };

            // get the attack ray from the source square to the occupancy board
            // which contains our "target" square
            let occupancy = BITBOARD_SQUARES[j];
            between[i][j] = DefaultAttackTable::attack_ray(&occupancy, source, direction);

            j += 1;
        }

        i += 1;
    }

    between
};

#[cfg(test)]
mod tests {
    use super::BITBOARD_BETWEEN;
    use crate::primitives::{BITBOARD_SQUARES, Square};

    #[test]
    fn print_bitboard_between_samples() {
        let samples = [
            (Square::D4, Square::D7),
            (Square::A1, Square::H8),
            (Square::B2, Square::F2),
            (Square::H1, Square::A8),
            (Square::E4, Square::B8),
        ];

        for (target, source) in samples {
            let bitboard = BITBOARD_BETWEEN[target.idx()][source.idx()];
            println!("start {target} end {source}");
            println!("start bitboard:\n{}", BITBOARD_SQUARES[target.idx()]);
            println!("end bitboard:\n{}", BITBOARD_SQUARES[source.idx()]);
            println!("bitboard:\n{bitboard}");
        }
    }
}
