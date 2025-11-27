use crate::movegen::{
    BISHOP_MAGIC_NUMS, BISHOP_TABLE_SIZE, Magic, MoveGenerator, ROOK_MAGIC_NUMS, ROOK_TABLE_SIZE,
};
use crate::primitives::{
    BITBOARD_FILES, BITBOARD_RANKS, BITBOARD_SQUARES, Bitboard, Files, Piece, Pieces, Ranks,
    Square, Squares, BitboardVec,
};

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    UpLeft,
    UpRight,
    DownRight,
    DownLeft,
}

impl MoveGenerator {
    pub fn init_magics(&mut self, piece: Piece) {
        let ok = piece == Pieces::ROOK || piece == Pieces::BISHOP;
        assert!(ok, "Illegal piece: {piece}");

        let is_rook = piece == Pieces::ROOK;
        let mut offset = 0;

        for sq in Squares::ALL {
            let r_mask = MoveGenerator::rook_mask(sq);
            let b_mask = MoveGenerator::bishop_mask(sq);
            let mask = if is_rook { r_mask } else { b_mask };

            let bits = mask.count_ones(); // Number of set bits in the mask
            let permutations = 2u64.pow(bits); // Number of blocker boards to be indexed.
            let end = offset + permutations - 1; // End point in the attack table.
            let blocker_boards = MoveGenerator::blocker_boards(mask);

            let r_ab = MoveGenerator::rook_attack_boards(sq, &blocker_boards);
            let b_ab = MoveGenerator::bishop_attack_boards(sq, &blocker_boards);
            let attack_boards = if is_rook { r_ab } else { b_ab };

            let mut magic: Magic = Default::default();
            let r_magic_nr = ROOK_MAGIC_NUMS[sq.unwrap()];
            let b_magic_nr = BISHOP_MAGIC_NUMS[sq.unwrap()];

            magic.mask = mask;
            magic.shift = (64 - bits) as u8;
            magic.offset = offset;
            magic.num = if is_rook { r_magic_nr } else { b_magic_nr };

            for i in 0..permutations {
                let next = i as usize;
                let index = magic.get_index(blocker_boards[next]);
                let rook_table = &mut self.rook_moves[..];
                let bishop_table = &mut self.bishop_moves[..];
                let table = if is_rook { rook_table } else { bishop_table };

                if table[index].is_empty() {
                    let fail_low = index < offset as usize;
                    let fail_high = index > end as usize;
                    assert!(!fail_low && !fail_high, "Indexing error. Error in Magics.");
                    table[index] = attack_boards[next];
                } else {
                    panic!("Attack table index not empty. Error in Magics.");
                }
            }

            // No failures during indexing. Store this magic.
            if is_rook {
                self.rook_magics[sq.unwrap()] = magic;
            } else {
                self.bishop_magics[sq.unwrap()] = magic;
            }

            // Do the next magic.
            offset += permutations;
        }

        // All permutations (blocker boards) should have been indexed.
        let r_ts = ROOK_TABLE_SIZE;
        let b_ts = BISHOP_TABLE_SIZE;
        let expectation = if is_rook { r_ts } else { b_ts };
        const ERROR: &str = "Initializing magics failed. Check magic numbers.";

        assert!(offset == expectation as u64, "{}", ERROR);
    }

    // rook_mask returns the rook mask for the given square
    //
    // @param: square - square to get the mask for
    // @return: masking bitboard for the given square
    pub fn rook_mask(square: Square) -> Bitboard {
        let bb_rook_square = BITBOARD_SQUARES[square.unwrap()];
        let bb_edges = MoveGenerator::get_edges(square);
        let bb_mask = BITBOARD_FILES[square.file()] | BITBOARD_RANKS[square.rank()];

        bb_mask & !bb_edges & !bb_rook_square
    }

    // bishop_mask returns the bishop mask for the given square
    //
    // @param: square - square to get the mask for
    // @return: masking bitboard for the given square
    #[inline(always)]
    pub fn bishop_mask(square: Square) -> Bitboard {
        let bitboard = Bitboard::empty();
        let bb_bishop_square = BITBOARD_SQUARES[square.unwrap()];
        let bb_edges = MoveGenerator::get_edges(square);
        let bb_up_left = MoveGenerator::bb_ray(&bitboard, square, Direction::UpLeft);
        let bb_up_right = MoveGenerator::bb_ray(&bitboard, square, Direction::UpRight);
        let bb_down_right = MoveGenerator::bb_ray(&bitboard, square, Direction::DownRight);
        let bb_down_left = MoveGenerator::bb_ray(&bitboard, square, Direction::DownLeft);

        (bb_up_left | bb_up_right | bb_down_right | bb_down_left) & !bb_edges & !bb_bishop_square
    }

    // TODO: think about moving this function elsewhere
    fn get_edges(exclude: Square) -> Bitboard {
        let exclude_file = BITBOARD_FILES[exclude.file()];
        let exclude_rank = BITBOARD_RANKS[exclude.rank()];

        (BITBOARD_FILES[Files::A] & !exclude_file)
            | (BITBOARD_FILES[Files::H] & !exclude_file)
            | (BITBOARD_RANKS[Ranks::R1] & !exclude_rank)
            | (BITBOARD_RANKS[Ranks::R8] & !exclude_rank)
    }

    // This function takes a square, and all the blocker boards belonging
    // to that square. Then it'll iterate through those blocker boards, and
    // generate the attack board belonging to that blocker board.
    pub fn rook_attack_boards(square: Square, blockers: &[Bitboard]) -> BitboardVec {
        let mut attacks: BitboardVec = Vec::new();

        for bitboard in blockers.iter() {
            let attacking = MoveGenerator::bb_ray(bitboard, square, Direction::Up)
                | MoveGenerator::bb_ray(bitboard, square, Direction::Right)
                | MoveGenerator::bb_ray(bitboard, square, Direction::Down)
                | MoveGenerator::bb_ray(bitboard, square, Direction::Left);
            attacks.push(attacking);
        }

        attacks
    }

    // Same as the function above, but for the bishop.
    pub fn bishop_attack_boards(square: Square, blockers: &[Bitboard]) -> BitboardVec {
        let mut bb_attack_boards: BitboardVec = Vec::new();

        for b in blockers.iter() {
            let bb_attacks = MoveGenerator::bb_ray(b, square, Direction::UpLeft)
                | MoveGenerator::bb_ray(b, square, Direction::UpRight)
                | MoveGenerator::bb_ray(b, square, Direction::DownRight)
                | MoveGenerator::bb_ray(b, square, Direction::DownLeft);
            bb_attack_boards.push(bb_attacks);
        }

        bb_attack_boards
    }

    // blocker_boards() takes a piece mask. This is a bitboard in which all
    // the bits are set for a square where a slider can move to, without
    // the edges. (As generated by the functions in the mask.rs file.)
    // blocker_boards() generates all possible permutations for the given
    // mask, using the Carry Rippler method. See the given link, or
    // http://rustic-chess.org for more information.
    pub fn blocker_boards(mask: Bitboard) -> BitboardVec {
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

    // This is a long function, but fortunately it's easy to understand. It
    // creates a ray for a sliding piece, in one of 8 directions: up, left,
    // right, down, up left, up right, down left, down right. (Some
    // programs call it N, E, S, W, NW, NE, SE, SW.) The function starts at
    // the given square, in a given direction, and then it keeps iterating
    // in that direction until it either hits a piece, or the edge of the
    // board. Therefore, in each call, only one of the eight blocks of this
    // function will be executed.
    pub fn bb_ray(bb_in: &Bitboard, square: Square, direction: Direction) -> Bitboard {
        let mut file = square.file();
        let mut rank = square.rank();
        let mut bb_square = BITBOARD_SQUARES[square.unwrap()];
        let mut bb_ray = Bitboard::empty();
        let mut done = false;
        while !done {
            done = true;
            match direction {
                Direction::Up => {
                    if rank != Ranks::R8 {
                        bb_square <<= 8u8;
                        bb_ray |= bb_square;
                        rank += 1;
                        done = !(bb_square & bb_in).is_empty();
                    }
                }
                Direction::Right => {
                    if file != Files::H {
                        bb_square <<= 1u8;
                        bb_ray |= bb_square;
                        file += 1;
                        done = !(bb_square & bb_in).is_empty();
                    }
                }
                Direction::Down => {
                    if rank != Ranks::R1 {
                        bb_square >>= 8u8;
                        bb_ray |= bb_square;
                        rank -= 1;
                        done = !(bb_square & bb_in).is_empty();
                    }
                }
                Direction::Left => {
                    if file != Files::A {
                        bb_square >>= 1u8;
                        bb_ray |= bb_square;
                        file -= 1;
                        done = !(bb_square & bb_in).is_empty();
                    }
                }
                Direction::UpLeft => {
                    if (rank != Ranks::R8) && (file != Files::A) {
                        bb_square <<= 7u8;
                        bb_ray |= bb_square;
                        rank += 1;
                        file -= 1;
                        done = !(bb_square & bb_in).is_empty();
                    }
                }
                Direction::UpRight => {
                    if (rank != Ranks::R8) && (file != Files::H) {
                        bb_square <<= 9u8;
                        bb_ray |= bb_square;
                        rank += 1;
                        file += 1;
                        done = !(bb_square & bb_in).is_empty();
                    }
                }
                Direction::DownRight => {
                    if (rank != Ranks::R1) && (file != Files::H) {
                        bb_square >>= 7u8;
                        bb_ray |= bb_square;
                        rank -= 1;
                        file += 1;
                        done = !(bb_square & bb_in).is_empty();
                    }
                }
                Direction::DownLeft => {
                    if (rank != Ranks::R1) && (file != Files::A) {
                        bb_square >>= 9u8;
                        bb_ray |= bb_square;
                        rank -= 1;
                        file -= 1;
                        done = !(bb_square & bb_in).is_empty();
                    }
                }
            };
        }
        bb_ray
    }
}
