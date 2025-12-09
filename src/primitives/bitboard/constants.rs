use crate::attack_table::{DefaultAttackTable, Direction};
use crate::primitives::bitboard::Bitboard;
use crate::primitives::{File, Rank, Square};

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
                between[i][j] = Bitboard::square(target);
                j += 1;
                continue;
            }

            // determine the direction of the attack ray
            let direction = if target_rank < source_rank && target_file < source_file {
                Direction::SouthWest
            } else if target_rank < source_rank && target_file > source_file {
                Direction::SouthEast
            } else if target_rank > source_rank && target_file < source_file {
                Direction::NorthWest
            } else if target_rank > source_rank && target_file > source_file {
                Direction::NorthEast
            } else if target_rank < source_rank {
                Direction::South
            } else if target_rank > source_rank {
                Direction::North
            } else if target_file < source_file {
                Direction::West
            } else {
                Direction::East
            };

            // get the attack ray from the source square to the occupancy board
            // which contains our "target" square
            let occupancy = Bitboard::square(target);
            between[i][j] = DefaultAttackTable::attack_ray(&occupancy, source, direction);

            j += 1;
        }

        i += 1;
    }

    between
};

// BITBOARD_LINES is a constant array of bitboards, where each bitboard is has
// the bits for that line from edge to edge intersecting the given squares set
// to 1
pub const BITBOARD_LINES: [[Bitboard; Square::TOTAL]; Square::TOTAL] = {
    let mut lines = [[Bitboard::empty(); Square::TOTAL]; Square::TOTAL];
    let mut i = 0;

    // for each ordered pair of squares [i, j], if they are on the same line
    // (rank, file, or diagonal), build the edge‑to‑edge ray that passes through
    // them; otherwise leave the entry empty
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

            // only build a line if the squares are distinct and lie on a
            // straight / diagonal line; otherwise keep the entry empty
            if i == j || !(same_rank || same_file || same_diagonal) {
                j += 1;
                continue;
            }

            // determine the primary direction from source -> target and its
            // opposite counterpart
            let (forward_dir, backward_dir) =
                if target_rank < source_rank && target_file < source_file {
                    (Direction::SouthWest, Direction::NorthEast)
                } else if target_rank < source_rank && target_file > source_file {
                    (Direction::SouthEast, Direction::NorthWest)
                } else if target_rank > source_rank && target_file < source_file {
                    (Direction::NorthWest, Direction::SouthEast)
                } else if target_rank > source_rank && target_file > source_file {
                    (Direction::NorthEast, Direction::SouthWest)
                } else if target_rank < source_rank {
                    (Direction::South, Direction::North)
                } else if target_rank > source_rank {
                    (Direction::North, Direction::South)
                } else if target_file < source_file {
                    (Direction::West, Direction::East)
                } else {
                    (Direction::East, Direction::West)
                };

            // build the full edge‑to‑edge line by casting rays in both the
            // forward and opposite directions on an empty board, then include
            // the source square itself
            let empty = Bitboard::empty();
            let forward = DefaultAttackTable::attack_ray(&empty, source, forward_dir);
            let backward = DefaultAttackTable::attack_ray(&empty, source, backward_dir);

            lines[i][j] = Bitboard::new(
                forward.const_unwrap()
                    | backward.const_unwrap()
                    | Bitboard::square(source).const_unwrap(),
            );

            j += 1;
        }

        i += 1;
    }

    lines
};

#[cfg(test)]
mod tests {
    use crate::primitives::{Bitboard, File, Rank, Square};

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
            let bitboard = Bitboard::between(target, source);
            println!("start {target} end {source}");
            println!("start bitboard:\n{}", Bitboard::square(target));
            println!("end bitboard:\n{}", Bitboard::square(source));
            println!("bitboard:\n{bitboard}");
        }
    }

    #[test]
    fn line_on_file_a_contains_all_a_file_squares() {
        let s1 = Square::A1;
        let s2 = Square::A8;

        let line = Bitboard::line(s1, s2);
        let expected = Bitboard::file(File::A);

        assert_eq!(
            line, expected,
            "line(A1, A8) should contain all squares on file A"
        );
    }

    #[test]
    fn line_on_rank_1_contains_all_rank_1_squares() {
        let s1 = Square::A1;
        let s2 = Square::H1;

        let line = Bitboard::line(s1, s2);
        let expected = Bitboard::rank(Rank::R1);

        assert_eq!(
            line, expected,
            "line(A1, H1) should contain all squares on rank 1"
        );
    }

    #[test]
    fn line_on_main_diagonal_contains_all_diagonal_squares() {
        let s1 = Square::C3;
        let s2 = Square::F6;

        let line = Bitboard::line(s1, s2);

        let diagonal = [
            Square::A1,
            Square::B2,
            Square::C3,
            Square::D4,
            Square::E5,
            Square::F6,
            Square::G7,
            Square::H8,
        ];

        let mut expected = Bitboard::empty();
        for sq in diagonal {
            expected |= Bitboard::square(sq);
        }

        assert_eq!(
            line, expected,
            "line(C3, F6) should contain all squares on the a1-h8 diagonal"
        );
    }

    #[test]
    fn line_for_non_collinear_squares_is_empty() {
        let s1 = Square::A1;
        let s2 = Square::B3;

        let line = Bitboard::line(s1, s2);

        assert!(
            line.is_empty(),
            "line(A1, B3) should be empty for non-collinear squares"
        );
    }
}
