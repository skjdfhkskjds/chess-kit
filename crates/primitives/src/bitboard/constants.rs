use crate::bitboard::Bitboard;
use crate::{Direction, File, Rank, Square};

/// BITBOARD_RANKS is a constant array of bitboards, where each bitboard has
/// the bits for that rank set to 1
pub const BITBOARD_RANKS: [Bitboard; Rank::TOTAL] = {
    const RANK_1: u64 = 0xFF;
    let mut ranks = [Bitboard::empty(); Rank::TOTAL];
    let mut i = 0;

    // note: while loop hack to get around const fn loop limitations
    while i < Rank::TOTAL {
        ranks[i] = Bitboard::new(RANK_1 << (i * 8));
        i += 1;
    }

    ranks
};

/// BITBOARD_FILES is a constant array of bitboards, where each bitboard has
/// the bits for that file set to 1
pub const BITBOARD_FILES: [Bitboard; File::TOTAL] = {
    const FILE_A: u64 = 0x0101_0101_0101_0101;
    let mut files = [Bitboard::empty(); File::TOTAL];
    let mut i = 0;

    // note: while loop hack to get around const fn loop limitations
    while i < File::TOTAL {
        files[i] = Bitboard::new(FILE_A << i);
        i += 1;
    }

    files
};

/// diagonal_mask returns the mask for the diagonal of the given square
///
/// @param: sq - square to get the diagonal mask for
/// @return: diagonal mask for the given square
#[inline(always)]
const fn diagonal_mask(sq: Square) -> Bitboard {
    let f0 = sq.file() as i8;
    let r0 = sq.rank() as i8;
    let d0 = f0 - r0;

    let mut mask = 0u64;
    let mut rank = 0u8;
    while rank < 8 {
        let mut file = 0u8;
        while file < 8 {
            if (file as i8 - rank as i8) == d0 {
                mask |= 1u64 << ((rank as u64) * 8 + (file as u64));
            }
            file += 1;
        }
        rank += 1;
    }

    Bitboard::new(mask)
}

/// anti_diagonal_mask returns the mask for the anti-diagonal of the given square
///
/// @param: sq - square to get the anti-diagonal mask for
/// @return: anti-diagonal mask for the given square
#[inline(always)]
const fn anti_diagonal_mask(sq: Square) -> Bitboard {
    let f0 = sq.file() as i8;
    let r0 = sq.rank() as i8;
    let s0 = f0 + r0;

    let mut mask = 0u64;
    let mut rank = 0u8;
    while rank < 8 {
        let mut file = 0u8;
        while file < 8 {
            if (file as i8 + rank as i8) == s0 {
                mask |= 1u64 << ((rank as u64) * 8 + (file as u64));
            }
            file += 1;
        }
        rank += 1;
    }

    Bitboard::new(mask)
}

/// BITBOARD_DIAGONALS is a constant array of bitboards, where each bitboard has
/// the bits for that diagonal set to 1
pub const BITBOARD_DIAGONALS: [Bitboard; Square::TOTAL] = {
    let mut diagonals = [Bitboard::empty(); Square::TOTAL];
    let mut i = 0;
    while i < Square::TOTAL {
        diagonals[i] = diagonal_mask(Square::from_idx(i));
        i += 1;
    }
    diagonals
};

/// BITBOARD_ANTI_DIAGONALS is a constant array of bitboards, where each bitboard
/// has the bits for that anti-diagonal set to 1
pub const BITBOARD_ANTI_DIAGONALS: [Bitboard; Square::TOTAL] = {
    let mut anti_diagonals = [Bitboard::empty(); Square::TOTAL];
    let mut i = 0;
    while i < Square::TOTAL {
        anti_diagonals[i] = anti_diagonal_mask(Square::from_idx(i));
        i += 1;
    }
    anti_diagonals
};

/// BITBOARD_BETWEEN is a constant array of bitboards, where each bitboard has
/// the bits for that between set to 1
///
/// @note: we define the between bitboard as the bitboard that contains the bits
///        for the squares that are on the line between the two given squares,
///        excluding the start square and including the end square
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

        let mut j = i;
        while j < Square::TOTAL {
            let target = Square::from_idx(j);
            let target_rank = target.rank().idx();
            let target_file = target.file().idx();

            let same_rank = target_rank == source_rank;
            let same_file = target_file == source_file;
            let same_diagonal = target_file > source_file
                && (target_file - source_file) == (target_rank - source_rank);
            let same_anti_diagonal = target_file < source_file
                && (source_file - target_file) == (target_rank - source_rank);

            // we only want to compute the between bitboard for pairs of squares
            // that are
            //
            // 1. not the same square
            // 2. on the same rank, file, or diagonal
            //
            // otherwise, the entry should just contain the end square's bitboard
            if i == j || !(same_rank || same_file || same_diagonal || same_anti_diagonal) {
                between[i][j] = Bitboard::square(target);
                between[j][i] = Bitboard::square(source);
                j += 1;
                continue;
            }

            // determine the direction of the attack ray
            //
            // note: since we have the invariant that target > source, we only
            //       need to check for north(west/east) or east directions
            let (distance, direction) = if same_rank {
                (target_file - source_file, Direction::East)
            } else if same_file {
                (target_rank - source_rank, Direction::North)
            } else if same_diagonal {
                (target_rank - source_rank, Direction::NorthEast)
            } else {
                (target_rank - source_rank, Direction::NorthWest)
            };

            // aggregate the attack rays in the given direction
            let mut ray = 0u64;
            let mut s = Bitboard::square(source);
            let mut count = 0;
            while count < distance {
                s = s.shift(direction);
                ray |= s.const_unwrap();
                count += 1;
            }

            // set the attack ray for the source and target squares
            //
            // note: for the inverse direction, exclude the target and include
            //       the source square to get the inverse between bitboard
            between[i][j] = Bitboard::new(ray);
            between[j][i] = Bitboard::new(
                ray ^ Bitboard::square(source).const_unwrap()
                    ^ Bitboard::square(target).const_unwrap(),
            );

            j += 1;
        }

        i += 1;
    }

    between
};

/// BITBOARD_LINES is a constant array of bitboards, where each bitboard is has
/// the bits for that line from edge to edge intersecting the given squares set
/// to 1
pub const BITBOARD_LINES: [[Bitboard; Square::TOTAL]; Square::TOTAL] = {
    let mut lines = [[Bitboard::empty(); Square::TOTAL]; Square::TOTAL];

    // for each ordered pair of squares [i, j], if they are on the same line
    // (rank, file, or diagonal), build the edge‑to‑edge ray that passes through
    // them; otherwise leave the entry empty
    let mut i = 0;
    while i < Square::TOTAL {
        let source = Square::from_idx(i);
        let source_rank = source.rank().idx();
        let source_file = source.file().idx();

        let mut j = i + 1;
        while j < Square::TOTAL {
            let target = Square::from_idx(j);
            let target_rank = target.rank().idx();
            let target_file = target.file().idx();

            let same_rank = target_rank == source_rank;
            let same_file = target_file == source_file;
            let same_diagonal = target_file > source_file
                && (target_file - source_file) == (target_rank - source_rank);
            let same_anti_diagonal = target_file < source_file
                && (source_file - target_file) == (target_rank - source_rank);

            // only build a line if the squares are distinct and lie on a
            // straight / diagonal line; otherwise keep the entry empty
            if !(same_rank || same_file || same_diagonal || same_anti_diagonal) {
                j += 1;
                continue;
            }

            let line = if same_rank {
                Bitboard::rank(source.rank())
            } else if same_file {
                Bitboard::file(source.file())
            } else if same_diagonal {
                Bitboard::diagonal(source)
            } else {
                Bitboard::anti_diagonal(source)
            };

            lines[i][j] = line;
            lines[j][i] = line;

            j += 1;
        }

        i += 1;
    }

    lines
};

#[cfg(test)]
mod tests {
    use crate::{Bitboard, File, Rank, Square};

    fn bb(squares: &[Square]) -> Bitboard {
        squares
            .iter()
            .fold(Bitboard::empty(), |acc, &sq| acc | Bitboard::square(sq))
    }

    #[test]
    fn between_d4_to_d7_includes_vertical_segment() {
        let expected = bb(&[Square::D5, Square::D6, Square::D7]);
        assert_eq!(
            Bitboard::between(Square::D4, Square::D7),
            expected,
            "between(D4, D7) should include D5, D6, D7"
        );
    }

    #[test]
    fn between_a1_to_h8_includes_main_diagonal() {
        let expected = bb(&[
            Square::B2,
            Square::C3,
            Square::D4,
            Square::E5,
            Square::F6,
            Square::G7,
            Square::H8,
        ]);
        assert_eq!(
            Bitboard::between(Square::A1, Square::H8),
            expected,
            "between(A1, H8) should include all main-diagonal squares"
        );
    }

    #[test]
    fn between_b2_to_f2_includes_rank_segment() {
        let expected = bb(&[Square::C2, Square::D2, Square::E2, Square::F2]);
        assert_eq!(
            Bitboard::between(Square::B2, Square::F2),
            expected,
            "between(B2, F2) should include C2, D2, E2, F2"
        );
    }

    #[test]
    fn between_h1_to_a8_includes_anti_diagonal() {
        let expected = bb(&[
            Square::G2,
            Square::F3,
            Square::E4,
            Square::D5,
            Square::C6,
            Square::B7,
            Square::A8,
        ]);
        assert_eq!(
            Bitboard::between(Square::H1, Square::A8),
            expected,
            "between(H1, A8) should include all anti-diagonal squares"
        );
    }

    #[test]
    fn between_e4_to_b8_returns_target_when_not_aligned() {
        let expected = bb(&[Square::B8]);
        assert_eq!(
            Bitboard::between(Square::E4, Square::B8),
            expected,
            "between(E4, B8) should fall back to the target square"
        );
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
