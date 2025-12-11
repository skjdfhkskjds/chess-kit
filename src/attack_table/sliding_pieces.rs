use crate::attack_table::{Direction, table::BitboardTable};
use crate::primitives::{Bitboard, File, Rank, Square};

// new_empty_rook_table creates a new empty rook table
//
// @return: new empty rook table
pub(crate) const fn new_empty_rook_table() -> BitboardTable {
    let mut empty_rook_table: BitboardTable = [Bitboard::empty(); Square::TOTAL];

    let mut sq = 0;
    while sq < Square::TOTAL {
        let square = Square::from_idx(sq);

        // the rook attacks on empty squares are just the file and ranks
        // excluding the square itself
        let attacks = (Bitboard::file(square.file()).const_unwrap()
            | Bitboard::rank(square.rank()).const_unwrap())
            ^ Bitboard::square(square).const_unwrap();

        empty_rook_table[sq] = Bitboard::new(attacks);
        sq += 1;
    }

    empty_rook_table
}

// new_empty_bishop_table creates a new empty bishop table
//
// @return: new empty bishop table
pub(crate) const fn new_empty_bishop_table() -> BitboardTable {
    let mut empty_bishop_table: BitboardTable = [Bitboard::empty(); Square::TOTAL];

    let mut sq = 0;
    while sq < Square::TOTAL {
        let square = Square::from_idx(sq);

        // the bishop attacks on empty squares are the attack rays in all
        // four directions
        //
        // note: attack_ray excludes the source square already
        let attacks = attack_ray(Bitboard::empty(), square, Direction::NorthWest).const_unwrap()
            | attack_ray(Bitboard::empty(), square, Direction::NorthEast).const_unwrap()
            | attack_ray(Bitboard::empty(), square, Direction::SouthEast).const_unwrap()
            | attack_ray(Bitboard::empty(), square, Direction::SouthWest).const_unwrap();

        empty_bishop_table[sq] = Bitboard::new(attacks);
        sq += 1;
    }

    empty_bishop_table
}

// rook_mask returns the rook mask for the given square
//
// @param: square - square to get the mask for
// @return: masking bitboard for the given square
pub(crate) const fn rook_mask(square: Square) -> Bitboard {
    let rook_at = Bitboard::square(square).const_unwrap();
    let edges = get_edges(square).const_unwrap();
    let line_of_sight =
        Bitboard::file(square.file()).const_unwrap() | Bitboard::rank(square.rank()).const_unwrap();

    Bitboard::new(line_of_sight & !edges & !rook_at)
}

// bishop_mask returns the bishop mask for the given square
//
// @param: square - square to get the mask for
// @return: masking bitboard for the given square
pub(crate) const fn bishop_mask(square: Square) -> Bitboard {
    let bishop_at = Bitboard::square(square).const_unwrap();
    let edges = get_edges(square).const_unwrap();
    let line_of_sight = attack_ray(Bitboard::empty(), square, Direction::NorthWest).const_unwrap()
        | attack_ray(Bitboard::empty(), square, Direction::NorthEast).const_unwrap()
        | attack_ray(Bitboard::empty(), square, Direction::SouthEast).const_unwrap()
        | attack_ray(Bitboard::empty(), square, Direction::SouthWest).const_unwrap();

    Bitboard::new(line_of_sight & !edges & !bishop_at)
}

// bishop_attack_board returns the attack board associated with the given
// square and blocker board.
//
// @param: square - square to get the attack board for
// @param: blocker - blocker to use to generate the attack board
// @return: attack board for the given square and blocker
pub(crate) const fn rook_attack_board(square: Square, blocker: Bitboard) -> Bitboard {
    Bitboard::new(
        attack_ray(blocker, square, Direction::North).const_unwrap()
            | attack_ray(blocker, square, Direction::East).const_unwrap()
            | attack_ray(blocker, square, Direction::South).const_unwrap()
            | attack_ray(blocker, square, Direction::West).const_unwrap(),
    )
}

// bishop_attack_board returns the attack board associated with the given
// square and blocker board.
//
// @param: square - square to get the attack board for
// @param: blocker - blocker to use to generate the attack board
// @return: attack board for the given square and blocker
pub(crate) const fn bishop_attack_board(square: Square, blocker: Bitboard) -> Bitboard {
    Bitboard::new(
        attack_ray(blocker, square, Direction::NorthWest).const_unwrap()
            | attack_ray(blocker, square, Direction::NorthEast).const_unwrap()
            | attack_ray(blocker, square, Direction::SouthEast).const_unwrap()
            | attack_ray(blocker, square, Direction::SouthWest).const_unwrap(),
    )
}

// get_edges generates a bitboard of all the edges of the board excluding
// the given square.
//
// @param: exclude - square to exclude from the edges
// @return: bitboard of all the edges of the board
// TODO: think about moving this function elsewhere
const fn get_edges(exclude: Square) -> Bitboard {
    let exclude_file = Bitboard::file(exclude.file()).const_unwrap();
    let exclude_rank = Bitboard::rank(exclude.rank()).const_unwrap();

    Bitboard::new(
        (Bitboard::file(File::A).const_unwrap() & !exclude_file)
            | (Bitboard::file(File::H).const_unwrap() & !exclude_file)
            | (Bitboard::rank(Rank::R1).const_unwrap() & !exclude_rank)
            | (Bitboard::rank(Rank::R8).const_unwrap() & !exclude_rank),
    )
}

// attack_ray returns the attack ray from the current square in the given
// direction based on the given bitboard.
//
// note: we unwrap the bitboards to u64 as a hack to enable const bitwise
//       operations
//
// @param: occupancy - occupancy bitboard to use as the base for the attack ray
// @param: square - square to start the attack ray from
// @param: direction - direction to attack in
// @return: attack ray bitboard
pub const fn attack_ray(occupancy: Bitboard, square: Square, direction: Direction) -> Bitboard {
    // get the file and rank and the square to analyze
    let mut file = square.file();
    let mut rank = square.rank();
    let mut square = Bitboard::square(square).const_unwrap();

    // build the ray bitboard in the given direction
    let mut ray = Bitboard::empty().const_unwrap();
    let occupancy = occupancy.const_unwrap();
    loop {
        match direction {
            Direction::North => {
                if rank.const_eq(Rank::R8) {
                    break;
                }

                square <<= 8u8;
                ray |= square;
                rank.inc();
            }
            Direction::East => {
                if file.const_eq(File::H) {
                    break;
                }

                square <<= 1u8;
                ray |= square;
                file.inc();
            }
            Direction::South => {
                if rank.const_eq(Rank::R1) {
                    break;
                }

                square >>= 8u8;
                ray |= square;
                rank.dec();
            }
            Direction::West => {
                if file.const_eq(File::A) {
                    break;
                }

                square >>= 1u8;
                ray |= square;
                file.dec();
            }
            Direction::NorthWest => {
                if rank.const_eq(Rank::R8) || file.const_eq(File::A) {
                    break;
                }

                square <<= 7u8;
                ray |= square;
                rank.inc();
                file.dec();
            }
            Direction::NorthEast => {
                if rank.const_eq(Rank::R8) || file.const_eq(File::H) {
                    break;
                }

                square <<= 9u8;
                ray |= square;
                rank.inc();
                file.inc();
            }
            Direction::SouthEast => {
                if rank.const_eq(Rank::R1) || file.const_eq(File::H) {
                    break;
                }

                square >>= 7u8;
                ray |= square;
                rank.dec();
                file.inc();
            }
            Direction::SouthWest => {
                if rank.const_eq(Rank::R1) || file.const_eq(File::A) {
                    break;
                }

                square >>= 9u8;
                ray |= square;
                rank.dec();
                file.dec();
            }
        };

        // if the square is blocked, we have built the full ray in this
        // direction, so we can stop
        if square & occupancy != 0 {
            break;
        }
    }

    Bitboard::new(ray)
}

