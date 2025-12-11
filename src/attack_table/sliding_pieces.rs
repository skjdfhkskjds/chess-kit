use crate::attack_table::{DefaultAttackTable, Direction};
use crate::primitives::{Bitboard, BitboardVec, File, Rank, Square};

impl DefaultAttackTable {
    // init_empty_tables initializes the empty tables for the rook and bishop
    // tables
    //
    // @return: void
    pub(crate) const fn init_empty_tables(&mut self) {
        let mut sq = 0;
        while sq < Square::TOTAL {
            let square = Square::from_idx(sq);
            // the rook attacks on empty squares are just the file and ranks
            // excluding the square itself
            self.empty_rook_table[square.idx()] = Bitboard::new(
                (Bitboard::file(square.file()).const_unwrap()
                    | Bitboard::rank(square.rank()).const_unwrap())
                    ^ Bitboard::square(square).const_unwrap(),
            );

            // the bishop attacks on empty squares are the attack rays in all
            // four directions
            //
            // note: attack_ray excludes the source square already
            let bitboard = Bitboard::empty();
            self.empty_bishop_table[square.idx()] = Bitboard::new(
                DefaultAttackTable::attack_ray(&bitboard, square, Direction::NorthWest)
                    .const_unwrap()
                    | DefaultAttackTable::attack_ray(&bitboard, square, Direction::NorthEast)
                        .const_unwrap()
                    | DefaultAttackTable::attack_ray(&bitboard, square, Direction::SouthEast)
                        .const_unwrap()
                    | DefaultAttackTable::attack_ray(&bitboard, square, Direction::SouthWest)
                        .const_unwrap(),
            );

            sq += 1;
        }
    }

    // rook_mask returns the rook mask for the given square
    //
    // @param: square - square to get the mask for
    // @return: masking bitboard for the given square
    pub(crate) const fn rook_mask(square: Square) -> Bitboard {
        let rook_at = Bitboard::square(square).const_unwrap();
        let edges = DefaultAttackTable::get_edges(square).const_unwrap();
        let line_of_sight = Bitboard::file(square.file()).const_unwrap()
            | Bitboard::rank(square.rank()).const_unwrap();

        Bitboard::new(line_of_sight & !edges & !rook_at)
    }

    // bishop_mask returns the bishop mask for the given square
    //
    // @param: square - square to get the mask for
    // @return: masking bitboard for the given square
    pub(crate) const fn bishop_mask(square: Square) -> Bitboard {
        let bishop_at = Bitboard::square(square).const_unwrap();
        let edges = DefaultAttackTable::get_edges(square).const_unwrap();
        let bitboard = Bitboard::empty();
        let line_of_sight = DefaultAttackTable::attack_ray(&bitboard, square, Direction::NorthWest)
            .const_unwrap()
            | DefaultAttackTable::attack_ray(&bitboard, square, Direction::NorthEast)
                .const_unwrap()
            | DefaultAttackTable::attack_ray(&bitboard, square, Direction::SouthEast)
                .const_unwrap()
            | DefaultAttackTable::attack_ray(&bitboard, square, Direction::SouthWest)
                .const_unwrap();

        Bitboard::new(line_of_sight & !edges & !bishop_at)
    }

    // rook_attack_boards returns the attack boards for the given square and
    // blockers.
    //
    // @param: square - square to get the attack boards for
    // @param: blockers - blockers to use to generate the attack boards
    // @return: attack boards for the given square and blockers
    pub(crate) fn rook_attack_boards(square: Square, blockers: &[Bitboard]) -> BitboardVec {
        let mut attacks: BitboardVec = Vec::new();

        for bitboard in blockers.iter() {
            let attacking = DefaultAttackTable::attack_ray(bitboard, square, Direction::North)
                | DefaultAttackTable::attack_ray(bitboard, square, Direction::East)
                | DefaultAttackTable::attack_ray(bitboard, square, Direction::South)
                | DefaultAttackTable::attack_ray(bitboard, square, Direction::West);

            attacks.push(attacking);
        }

        attacks
    }

    // bishop_attack_boards returns the attack boards for the given square and
    // blockers.
    //
    // @param: square - square to get the attack boards for
    // @param: blockers - blockers to use to generate the attack boards
    // @return: attack boards for the given square and blockers
    pub(crate) fn bishop_attack_boards(square: Square, blockers: &[Bitboard]) -> BitboardVec {
        let mut attacks: BitboardVec = Vec::new();

        for bitboard in blockers.iter() {
            let attacking = DefaultAttackTable::attack_ray(bitboard, square, Direction::NorthWest)
                | DefaultAttackTable::attack_ray(bitboard, square, Direction::NorthEast)
                | DefaultAttackTable::attack_ray(bitboard, square, Direction::SouthEast)
                | DefaultAttackTable::attack_ray(bitboard, square, Direction::SouthWest);

            attacks.push(attacking);
        }

        attacks
    }

    // bishop_attack_board returns the attack board associated with the given
    // square and blocker board.
    //
    // @param: square - square to get the attack board for
    // @param: blocker - blocker to use to generate the attack board
    // @return: attack board for the given square and blocker
    pub(crate) const fn rook_attack_board(square: Square, blocker: &Bitboard) -> Bitboard {
        Bitboard::new(
            DefaultAttackTable::attack_ray(blocker, square, Direction::North).const_unwrap()
                | DefaultAttackTable::attack_ray(blocker, square, Direction::East).const_unwrap()
                | DefaultAttackTable::attack_ray(blocker, square, Direction::South).const_unwrap()
                | DefaultAttackTable::attack_ray(blocker, square, Direction::West).const_unwrap(),
        )
    }

    // bishop_attack_board returns the attack board associated with the given
    // square and blocker board.
    //
    // @param: square - square to get the attack board for
    // @param: blocker - blocker to use to generate the attack board
    // @return: attack board for the given square and blocker
    pub(crate) const fn bishop_attack_board(square: Square, blocker: &Bitboard) -> Bitboard {
        Bitboard::new(
            DefaultAttackTable::attack_ray(blocker, square, Direction::NorthWest).const_unwrap()
                | DefaultAttackTable::attack_ray(blocker, square, Direction::NorthEast)
                    .const_unwrap()
                | DefaultAttackTable::attack_ray(blocker, square, Direction::SouthEast)
                    .const_unwrap()
                | DefaultAttackTable::attack_ray(blocker, square, Direction::SouthWest)
                    .const_unwrap(),
        )
    }

    // blocker_boards() takes a piece mask. This is a bitboard in which all
    // the bits are set for a square where a slider can move to, without
    // the edges. (As generated by the functions in the mask.rs file.)
    // blocker_boards() generates all possible permutations for the given
    // mask, using the Carry Rippler method. See the given link, or
    // http://rustic-chess.org for more information.
    // TODO: revisit this function later
    pub(crate) fn blocker_boards(mask: Bitboard) -> BitboardVec {
        let mut bb_blocker_boards: BitboardVec = Vec::new();
        let mut n: Bitboard = Bitboard::empty();

        // Carry-Rippler
        // https://www.chessprogramming.org/Traversing_Subsets_of_a_Set
        loop {
            bb_blocker_boards.push(n);
            n = n.wrapping_sub(mask) & mask;
            if n.is_empty() {
                break;
            }
        }

        bb_blocker_boards
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
    // @param: bitboard - bitboard to use as the base for the attack ray
    // @param: square - square to start the attack ray from
    // @param: direction - direction to attack in
    // @return: attack ray bitboard
    pub const fn attack_ray(bitboard: &Bitboard, square: Square, direction: Direction) -> Bitboard {
        // get the file and rank and the square to analyze
        let mut file = square.file();
        let mut rank = square.rank();
        let mut square = Bitboard::square(square).const_unwrap();

        // build the ray bitboard in the given direction
        let mut ray = 0u64;
        let occupancy = bitboard.const_unwrap();
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
}
