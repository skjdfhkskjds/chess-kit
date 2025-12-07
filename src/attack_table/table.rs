use crate::attack_table::magics::{BISHOP_TABLE_SIZE, Magic, ROOK_TABLE_SIZE};
use crate::attack_table::{AttackTable, PawnDirections};
use crate::primitives::{Bitboard, BitboardVec, File, Pieces, Side, Sides, Square};
use std::sync::OnceLock;

type BitboardTable = [Bitboard; Square::TOTAL];
type MagicTable = [Magic; Square::TOTAL];

static DEFAULT_ATTACK_TABLE: OnceLock<DefaultAttackTable> = OnceLock::new();

pub fn default_attack_table() -> &'static DefaultAttackTable {
    DEFAULT_ATTACK_TABLE.get_or_init(DefaultAttackTable::new)
}

pub struct DefaultAttackTable {
    // king targets from each square
    pub(crate) king_table: BitboardTable,

    // knight targets from each square
    pub(crate) knight_table: BitboardTable,

    // pawn pushes from each square for each side
    pub(crate) pawn_push_table: [BitboardTable; Sides::TOTAL],

    // pawn attacks from each square for each side
    pub(crate) pawn_attack_table: [BitboardTable; Sides::TOTAL],

    // bishop targets from each square for each occupancy
    pub(crate) bishop_table: BitboardVec,

    // rook targets from each square for each occupancy
    pub(crate) rook_table: BitboardVec,

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
            king_table: [Bitboard::empty(); Square::TOTAL],
            knight_table: [Bitboard::empty(); Square::TOTAL],
            pawn_push_table: [[Bitboard::empty(); Square::TOTAL]; Sides::TOTAL],
            pawn_attack_table: [[Bitboard::empty(); Square::TOTAL]; Sides::TOTAL],
            bishop_table: vec![Bitboard::empty(); BISHOP_TABLE_SIZE],
            rook_table: vec![Bitboard::empty(); ROOK_TABLE_SIZE],
            rook_magics: [Magic::default(); Square::TOTAL],
            bishop_magics: [Magic::default(); Square::TOTAL],
        };

        // initialize the attack table
        attack_table.init_king_table();
        attack_table.init_knight_table();
        attack_table.init_pawn_table();
        attack_table.init_magics(Pieces::Rook);
        attack_table.init_magics(Pieces::Bishop);

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
        self.pawn_push_table[SideT::INDEX][sq.idx()]
    }

    // pawn_targets returns the squares that the pawn targets from the given
    // square for the given side
    //
    // @impl: PieceTargetsTable::pawn_targets
    #[inline(always)]
    fn pawn_targets<SideT: Side>(&self, sq: Square) -> Bitboard {
        self.pawn_attack_table[SideT::INDEX][sq.idx()]
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
        if SideT::SIDE == Sides::White {
            match direction {
                PawnDirections::Up => squares << 8u8,
                PawnDirections::Right => (squares & !Bitboard::file(File::H)) << 9u8,
                PawnDirections::Left => (squares & !Bitboard::file(File::A)) << 7u8,
            }
        } else {
            match direction {
                PawnDirections::Up => squares >> 8u8,
                PawnDirections::Right => (squares & !Bitboard::file(File::A)) >> 9u8,
                PawnDirections::Left => (squares & !Bitboard::file(File::H)) >> 7u8,
            }
        }
    }

    // rook_targets returns the attacks for the given square and bitboard.
    //
    // @impl: PieceTargetsTable::rook_targets
    #[inline(always)]
    fn rook_targets(&self, square: Square, bitboard: Bitboard) -> Bitboard {
        self.rook_table[self.rook_magics[square.idx()].idx(bitboard)]
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
        self.rook_targets(square, bitboard) ^ self.bishop_targets(square, bitboard)
    }
}
