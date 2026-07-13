use crate::table::BitboardTable;
use crate::table::{EMPTY_BISHOP_TABLE, EMPTY_ROOK_TABLE};
use chess_kit_primitives::{Bitboard, File, Rank, Square};

const RANK_EDGES: Bitboard = Bitboard::new(
    Bitboard::rank(Rank::R1).const_unwrap() | Bitboard::rank(Rank::R8).const_unwrap(),
);

const FILE_EDGES: Bitboard =
    Bitboard::new(Bitboard::file(File::A).const_unwrap() | Bitboard::file(File::H).const_unwrap());

/// new_empty_rook_table creates a new empty rook table
///
/// @return: new empty rook table
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

/// new_empty_bishop_table creates a new empty bishop table
///
/// @return: new empty bishop table
pub(crate) const fn new_empty_bishop_table() -> BitboardTable {
    let mut empty_bishop_table: BitboardTable = [Bitboard::empty(); Square::TOTAL];

    let mut sq = 0;
    while sq < Square::TOTAL {
        let square = Square::from_idx(sq);

        // the bishop attacks on empty squares are the diagonals excluding the
        // square itself
        let attacks = (Bitboard::diagonal(square).const_unwrap()
            | Bitboard::anti_diagonal(square).const_unwrap())
            ^ Bitboard::square(square).const_unwrap();

        empty_bishop_table[sq] = Bitboard::new(attacks);
        sq += 1;
    }

    empty_bishop_table
}

/// rook_mask returns the rook mask for the given square
///
/// @param: square - square to get the mask for
/// @return: masking bitboard for the given square
pub(crate) const fn rook_mask(square: Square) -> Bitboard {
    let edges = get_edges(square).const_unwrap();
    let attacks = EMPTY_ROOK_TABLE[square.idx()].const_unwrap();

    Bitboard::new(attacks & !edges)
}

/// bishop_mask returns the bishop mask for the given square
///
/// @param: square - square to get the mask for
/// @return: masking bitboard for the given square
pub(crate) const fn bishop_mask(square: Square) -> Bitboard {
    let edges = get_edges(square).const_unwrap();
    let attacks = EMPTY_BISHOP_TABLE[square.idx()].const_unwrap();

    Bitboard::new(attacks & !edges)
}

/// rook_attack_board returns the attack board associated with the given
/// square and blocker board
///
/// @param: square - square bitboard to get the attack board for
/// @param: file - precomputed file mask for the given square
/// @param: rank - precomputed rank mask for the given square
/// @param: occupancy - occupancy to use to generate the attack board
/// @return: attack board for the given square and occupancy
pub(crate) const fn rook_attack_board(
    square: Bitboard,
    file: Bitboard,
    rank: Bitboard,
    occupancy: Bitboard,
) -> Bitboard {
    let square_unwrapped = square.const_unwrap();
    let occupancy_unwrapped = occupancy.const_unwrap();

    Bitboard::new(
        fast_attack_ray(occupancy_unwrapped, square_unwrapped, file.const_unwrap())
            | fast_attack_ray(occupancy_unwrapped, square_unwrapped, rank.const_unwrap()),
    )
}

/// bishop_attack_board returns the attack board associated with the given
/// square and blocker board.
///
/// @param: square - square bitboard to get the attack board for
/// @param: diagonal - precomputed diagonal mask for the given square
/// @param: anti_diagonal - precomputed anti-diagonal mask for the given square
/// @param: occupancy - occupancy to use to generate the attack board
/// @return: attack board for the given square and occupancy
pub(crate) const fn bishop_attack_board(
    square: Bitboard,
    diagonal: Bitboard,
    anti_diagonal: Bitboard,
    occupancy: Bitboard,
) -> Bitboard {
    let square_unwrapped = square.const_unwrap();
    let occupancy_unwrapped = occupancy.const_unwrap();

    Bitboard::new(
        fast_attack_ray(
            occupancy_unwrapped,
            square_unwrapped,
            diagonal.const_unwrap(),
        ) | fast_attack_ray(
            occupancy_unwrapped,
            square_unwrapped,
            anti_diagonal.const_unwrap(),
        ),
    )
}

/// get_edges generates a bitboard of all the edges of the board excluding
/// the given square.
///
/// @param: exclude - square to exclude from the edges
/// @return: bitboard of all the edges of the board
/// TODO: think about moving this function elsewhere
const fn get_edges(exclude: Square) -> Bitboard {
    let exclude_file = Bitboard::file(exclude.file()).const_unwrap();
    let exclude_rank = Bitboard::rank(exclude.rank()).const_unwrap();

    Bitboard::new(
        (FILE_EDGES.const_unwrap() & !exclude_file) | (RANK_EDGES.const_unwrap() & !exclude_rank),
    )
}

/// attack_ray returns the attack ray from the current square in the given
/// line based on the given bitboard.
///
/// note: this function uses Hyperbola Quintessence to generate the ray, see
///       https://www.chessprogramming.org/Hyperbola_Quintessence for details
///
/// @param: occupancy - occupancy bitboard
/// @param: square - square to start the attack ray from
/// @param: line_mask - line mask to use to generate the attack ray
/// @return: attack ray bitboard
///
/// @requires: occupancy already excludes square
const fn fast_attack_ray(occupancy: u64, square: u64, line_mask: u64) -> u64 {
    let mut forward = occupancy & line_mask;
    let mut reverse = forward.reverse_bits(); // o'-s'
    forward = forward.wrapping_sub(square); // o -2s
    reverse = reverse.wrapping_sub(square.reverse_bits()); // o'-2s'
    forward ^= reverse.reverse_bits();

    forward & line_mask // mask the line again
}
