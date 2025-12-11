use crate::attack_table::magics::{BISHOP_TABLE_SIZE, Magic, ROOK_TABLE_SIZE, new_magics};
use crate::attack_table::moving_pieces::{new_king_table, new_knight_table, new_pawn_table};
use crate::attack_table::sliding_pieces::{new_empty_bishop_table, new_empty_rook_table};
use crate::attack_table::{AttackTable, NOT_A_FILE, NOT_H_FILE, PawnDirections};
use crate::primitives::{Bitboard, Pieces, Side, Sides, Square};

pub(crate) type BitboardTable = [Bitboard; Square::TOTAL];
pub(crate) type MagicTable = [Magic; Square::TOTAL];

pub struct DefaultAttackTable {}

static KING_TABLE: BitboardTable = new_king_table();
static KNIGHT_TABLE: BitboardTable = new_knight_table();
static PAWN_TABLE: [BitboardTable; Sides::TOTAL] = new_pawn_table();
static EMPTY_BISHOP_TABLE: BitboardTable = new_empty_bishop_table();
static EMPTY_ROOK_TABLE: BitboardTable = new_empty_rook_table();
static BISHOP_MAGICS_TABLE: BishopMagicsTable = BishopMagicsTable::new();

#[cfg_attr(true, allow(long_running_const_eval))]
static ROOK_MAGICS_TABLE: RookMagicsTable = RookMagicsTable::new();

struct BishopMagicsTable {
    table: [Bitboard; BISHOP_TABLE_SIZE],
    magics: MagicTable,
}

struct RookMagicsTable {
    table: [Bitboard; ROOK_TABLE_SIZE],
    magics: MagicTable,
}

impl BishopMagicsTable {
    // new creates and initializes a new attack table
    //
    // @impl: AttackTable::new
    const fn new() -> Self {
        // create a new, empty table
        let mut attack_table = Self {
            table: [Bitboard::empty(); BISHOP_TABLE_SIZE],
            magics: [Magic::default(); Square::TOTAL],
        };

        // initialize the attack table
        attack_table.magics = new_magics(Pieces::Bishop, &mut attack_table.table);

        attack_table
    }
}

impl RookMagicsTable {
    // new creates and initializes a new attack table
    //
    // @impl: AttackTable::new
    const fn new() -> Self {
        // create a new, empty table
        let mut attack_table = Self {
            table: [Bitboard::empty(); ROOK_TABLE_SIZE],
            magics: [Magic::default(); Square::TOTAL],
        };

        // initialize the attack table
        attack_table.magics = new_magics(Pieces::Rook, &mut attack_table.table);

        attack_table
    }
}

impl AttackTable for DefaultAttackTable {
    // king_targets returns the squares that the king targets from the given
    // square
    //
    // @impl: PieceTargetsTable::king_targets
    #[inline(always)]
    fn king_targets(sq: Square) -> Bitboard {
        KING_TABLE[sq.idx()]
    }

    // knight_targets returns the squares that the knight targets from the
    // given square
    //
    // @impl: PieceTargetsTable::knight_targets
    #[inline(always)]
    fn knight_targets(sq: Square) -> Bitboard {
        KNIGHT_TABLE[sq.idx()]
    }

    // pawn_pushes returns the squares that the pawn pushes to from the given
    // square
    //
    // @impl: PieceTargetsTable::pawn_pushes
    #[inline(always)]
    fn pawn_pushes<SideT: Side>(sq: Square) -> Bitboard {
        match SideT::SIDE {
            Sides::White => Bitboard::square(sq) << 8u8,
            Sides::Black => Bitboard::square(sq) >> 8u8,
        }
    }

    // pawn_targets returns the squares that the pawn targets from the given
    // square for the given side
    //
    // @impl: PieceTargetsTable::pawn_targets
    #[inline(always)]
    fn pawn_targets<SideT: Side>(sq: Square) -> Bitboard {
        PAWN_TABLE[SideT::INDEX][sq.idx()]
    }

    // all_pawn_targets returns the squares that the all the pawns on the given
    // squares target from the given side in the given direction
    //
    // @impl: PieceTargetsTable::all_pawn_targets
    #[inline(always)]
    fn all_pawn_targets<SideT: Side>(squares: Bitboard, direction: PawnDirections) -> Bitboard {
        match SideT::SIDE {
            Sides::White => match direction {
                PawnDirections::Up => squares << 8u8,
                PawnDirections::Right => (squares & NOT_H_FILE) << 9u8,
                PawnDirections::Left => (squares & NOT_A_FILE) << 7u8,
            },
            Sides::Black => match direction {
                PawnDirections::Up => squares >> 8u8,
                PawnDirections::Right => (squares & NOT_A_FILE) >> 9u8,
                PawnDirections::Left => (squares & NOT_H_FILE) >> 7u8,
            },
        }
    }

    // empty_rook_targets returns the squares that the rook targets from the given
    // square on an empty board
    //
    // @impl: PieceTargetsTable::empty_rook_targets
    #[inline(always)]
    fn empty_rook_targets(square: Square) -> Bitboard {
        EMPTY_ROOK_TABLE[square.idx()]
    }

    // rook_targets returns the attacks for the given square and bitboard.
    //
    // @impl: PieceTargetsTable::rook_targets
    #[inline(always)]
    fn rook_targets(square: Square, bitboard: Bitboard) -> Bitboard {
        debug_assert!(
            ROOK_MAGICS_TABLE.magics[square.idx()].idx(bitboard) < ROOK_MAGICS_TABLE.table.len(),
            "Invalid index for square {square}. Error in Magics. occupancy:\n{bitboard}"
        );
        ROOK_MAGICS_TABLE.table[ROOK_MAGICS_TABLE.magics[square.idx()].idx(bitboard)]
    }

    // empty_bishop_targets returns the squares that the bishop targets from the given
    // square on an empty board
    //
    // @impl: PieceTargetsTable::empty_bishop_targets
    #[inline(always)]
    fn empty_bishop_targets(square: Square) -> Bitboard {
        EMPTY_BISHOP_TABLE[square.idx()]
    }

    // bishop_targets returns the attacks for the given square and bitboard.
    //
    // @impl: PieceTargetsTable::bishop_targets
    #[inline(always)]
    fn bishop_targets(square: Square, bitboard: Bitboard) -> Bitboard {
        BISHOP_MAGICS_TABLE.table[BISHOP_MAGICS_TABLE.magics[square.idx()].idx(bitboard)]
    }

    // queen_targets returns the attacks for the given square and bitboard.
    //
    // @impl: PieceTargetsTable::queen_targets
    #[inline(always)]
    fn queen_targets(square: Square, bitboard: Bitboard) -> Bitboard {
        Self::rook_targets(square, bitboard) | Self::bishop_targets(square, bitboard)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attack_table::AttackTable;

    #[test]
    fn empty_rook_targets_returns_correct_value() {
        let square = Square::A1;
        let expected = (Bitboard::file(square.file()) | Bitboard::rank(square.rank()))
            ^ Bitboard::square(square);
        let actual = DefaultAttackTable::empty_rook_targets(square);
        assert_eq!(actual, expected, "Expected {expected}, got {actual}");
    }

    #[test]
    fn empty_bishop_targets_returns_correct_value() {
        let square = Square::A1;
        let expected = Bitboard::between(square, Square::H8);
        let actual = DefaultAttackTable::empty_bishop_targets(square);
        assert_eq!(actual, expected, "Expected {expected}, got {actual}");
    }
}
