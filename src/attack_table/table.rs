use crate::attack_table::magics::{BISHOP_TABLE_SIZE, Magic, ROOK_TABLE_SIZE};
use crate::attack_table::{AttackTable, NOT_A_FILE, NOT_H_FILE, PawnDirections};
use crate::primitives::{Bitboard, BitboardVec, Pieces, Side, Sides, Square};
use std::sync::OnceLock;

pub(crate) type BitboardTable = [Bitboard; Square::TOTAL];
pub(crate) type MagicTable = [Magic; Square::TOTAL];

static DEFAULT_ATTACK_TABLE: OnceLock<DefaultAttackTable> = OnceLock::new();

pub fn default_attack_table() -> &'static DefaultAttackTable {
    DEFAULT_ATTACK_TABLE.get_or_init(DefaultAttackTable::new)
}

pub struct DefaultAttackTable {
    // king targets from each square
    pub(crate) king_table: BitboardTable,

    // knight targets from each square
    pub(crate) knight_table: BitboardTable,

    // pawn attacks from each square for each side
    pub(crate) pawn_table: [BitboardTable; Sides::TOTAL],

    // bishop targets from each square for an empty board
    pub(crate) empty_bishop_table: BitboardTable,

    // bishop targets from each square for each occupancy
    pub(crate) bishop_table: [Bitboard; BISHOP_TABLE_SIZE],

    // rook targets from each square for an empty board
    pub(crate) empty_rook_table: BitboardTable,

    // rook targets from each square for each occupancy
    pub(crate) rook_table: [Bitboard; ROOK_TABLE_SIZE],

    // magics for the bishop table
    pub(crate) bishop_magics: MagicTable,

    // magics for the rook table
    pub(crate) rook_magics: MagicTable,
}

impl AttackTable for DefaultAttackTable {
    // new creates and initializes a new attack table
    //
    // @impl: AttackTable::new
    fn new() -> Self {
        // create a new, empty table
        let mut attack_table = Self {
            king_table: DefaultAttackTable::new_king_table(),
            knight_table: DefaultAttackTable::new_knight_table(),
            pawn_table: DefaultAttackTable::new_pawn_table(),
            empty_bishop_table: DefaultAttackTable::new_empty_bishop_table(),
            bishop_table: [Bitboard::empty(); BISHOP_TABLE_SIZE],
            empty_rook_table: DefaultAttackTable::new_empty_rook_table(),
            rook_table: [Bitboard::empty(); ROOK_TABLE_SIZE],
            rook_magics: [Magic::default(); Square::TOTAL],
            bishop_magics: [Magic::default(); Square::TOTAL],
        };

        // initialize the attack table
        attack_table.rook_magics =
            DefaultAttackTable::new_magics(Pieces::Rook, &mut attack_table.rook_table);
        attack_table.bishop_magics =
            DefaultAttackTable::new_magics(Pieces::Bishop, &mut attack_table.bishop_table);

        attack_table
    }

    // king_targets returns the squares that the king targets from the given
    // square
    //
    // @impl: PieceTargetsTable::king_targets
    #[inline(always)]
    fn king_targets(&self, sq: Square) -> Bitboard {
        self.king_table[sq.idx()]
    }

    // knight_targets returns the squares that the knight targets from the
    // given square
    //
    // @impl: PieceTargetsTable::knight_targets
    #[inline(always)]
    fn knight_targets(&self, sq: Square) -> Bitboard {
        self.knight_table[sq.idx()]
    }

    // pawn_pushes returns the squares that the pawn pushes to from the given
    // square
    //
    // @impl: PieceTargetsTable::pawn_pushes
    #[inline(always)]
    fn pawn_pushes<SideT: Side>(&self, sq: Square) -> Bitboard {
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
    fn pawn_targets<SideT: Side>(&self, sq: Square) -> Bitboard {
        self.pawn_table[SideT::INDEX][sq.idx()]
    }

    // all_pawn_targets returns the squares that the all the pawns on the given
    // squares target from the given side in the given direction
    //
    // @impl: PieceTargetsTable::all_pawn_targets
    #[inline(always)]
    fn all_pawn_targets<SideT: Side>(
        &self,
        squares: Bitboard,
        direction: PawnDirections,
    ) -> Bitboard {
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
    fn empty_rook_targets(&self, square: Square) -> Bitboard {
        self.empty_rook_table[square.idx()]
    }

    // rook_targets returns the attacks for the given square and bitboard.
    //
    // @impl: PieceTargetsTable::rook_targets
    #[inline(always)]
    fn rook_targets(&self, square: Square, bitboard: Bitboard) -> Bitboard {
        debug_assert!(
            self.rook_magics[square.idx()].idx(bitboard) < self.rook_table.len(),
            "Invalid index for square {square}. Error in Magics. occupancy:\n{bitboard}"
        );
        self.rook_table[self.rook_magics[square.idx()].idx(bitboard)]
    }

    // empty_bishop_targets returns the squares that the bishop targets from the given
    // square on an empty board
    //
    // @impl: PieceTargetsTable::empty_bishop_targets
    #[inline(always)]
    fn empty_bishop_targets(&self, square: Square) -> Bitboard {
        self.empty_bishop_table[square.idx()]
    }

    // bishop_targets returns the attacks for the given square and bitboard.
    //
    // @impl: PieceTargetsTable::bishop_targets
    #[inline(always)]
    fn bishop_targets(&self, square: Square, bitboard: Bitboard) -> Bitboard {
        self.bishop_table[self.bishop_magics[square.idx()].idx(bitboard)]
    }

    // queen_targets returns the attacks for the given square and bitboard.
    //
    // @impl: PieceTargetsTable::queen_targets
    #[inline(always)]
    fn queen_targets(&self, square: Square, bitboard: Bitboard) -> Bitboard {
        self.rook_targets(square, bitboard) | self.bishop_targets(square, bitboard)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_rook_targets_returns_correct_value() {
        let attack_table = DefaultAttackTable::new();
        let square = Square::A1;
        let expected = (Bitboard::file(square.file()) | Bitboard::rank(square.rank()))
            ^ Bitboard::square(square);
        let actual = attack_table.empty_rook_targets(square);
        assert_eq!(actual, expected, "Expected {expected}, got {actual}");
    }

    #[test]
    fn empty_bishop_targets_returns_correct_value() {
        let attack_table = DefaultAttackTable::new();
        let square = Square::A1;
        let expected = Bitboard::between(square, Square::H8);
        let actual = attack_table.empty_bishop_targets(square);
        assert_eq!(actual, expected, "Expected {expected}, got {actual}");
    }
}
