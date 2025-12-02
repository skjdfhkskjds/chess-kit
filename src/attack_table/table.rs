use crate::attack_table::magics::{BISHOP_TABLE_SIZE, Magic, ROOK_TABLE_SIZE};
use crate::attack_table::{AttackTable, PieceTargetsTable};
use crate::position::Position;
use crate::primitives::{
    Bitboard, BitboardVec, GameStateExt, Pieces, Side, Sides, Square, State, White,
};

type BitboardTable = [Bitboard; Square::TOTAL];
type MagicTable = [Magic; Square::TOTAL];

pub struct DefaultAttackTable {
    // king targets from each square
    pub(crate) king_table: BitboardTable,

    // knight targets from each square
    pub(crate) knight_table: BitboardTable,

    // pawn targets from each square for each side
    pub(crate) pawn_table: [BitboardTable; Sides::TOTAL],

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
            pawn_table: [[Bitboard::empty(); Square::TOTAL]; Sides::TOTAL],
            bishop_table: vec![Bitboard::empty(); BISHOP_TABLE_SIZE],
            rook_table: vec![Bitboard::empty(); ROOK_TABLE_SIZE],
            rook_magics: [Magic::default(); Square::TOTAL],
            bishop_magics: [Magic::default(); Square::TOTAL],
        };

        // initialize the attack table
        attack_table.init_king_table();
        attack_table.init_knight_table();
        attack_table.init_pawn_table::<White>();
        attack_table.init_magics(Pieces::Rook);
        attack_table.init_magics(Pieces::Bishop);

        attack_table
    }

    // attacked_by returns a bitboard containing the squares that the given side
    // is attacked by at the given square
    //
    // @impl: AttackTable::attacked_by
    #[inline(always)]
    fn attacked_by<SideT: Side, StateT: State + GameStateExt>(
        &self,
        position: &Position<StateT>,
        square: Square,
    ) -> Bitboard {
        // idea: our square `T` is attacked iff the opponent has at least one
        //       piece in square `S` such that attack board generated from `T`
        //       includes `S`
        //
        // effectively relies on this idea of, if i can see you, you can see me
        //
        // the nuance not covered above is pawn attacks are not symmetric, so we
        // reconcile this by checking the pawn attacks for our side instead of the
        // opponent's

        // generate the attack boards for each piece
        let occupancy = position.total_occupancy();
        let king_attacks = self.king_targets(square);
        let rook_attacks = self.rook_targets(square, occupancy);
        let bishop_attacks = self.bishop_targets(square, occupancy);
        let knight_attacks = self.knight_targets(square);
        let pawn_attacks = self.pawn_targets::<SideT>(square);
        // @opt: union of rook and bishop attacks instead of routine call
        let queen_attacks = rook_attacks | bishop_attacks;

        // check if there is an intersection between an attack board and that
        // piece's respective occupancy
        let opponent = position.bitboards[SideT::Other::INDEX];
        king_attacks & opponent[Pieces::King.idx()]
            | rook_attacks & opponent[Pieces::Rook.idx()]
            | queen_attacks & opponent[Pieces::Queen.idx()]
            | bishop_attacks & opponent[Pieces::Bishop.idx()]
            | knight_attacks & opponent[Pieces::Knight.idx()]
            | pawn_attacks & opponent[Pieces::Pawn.idx()]
    }

    // is_attacked returns true if the given square on the given side is attacked
    // by the opponent
    //
    // @impl: AttackTable::is_attacked
    #[inline(always)]
    fn is_attacked<SideT: Side, StateT: State + GameStateExt>(
        &self,
        position: &Position<StateT>,
        square: Square,
    ) -> bool {
        !self
            .attacked_by::<SideT, StateT>(position, square)
            .is_empty()
    }

    // is_checked returns true if the given side is checked
    //
    // @impl: AttackTable::is_checked
    #[inline(always)]
    fn is_checked<SideT: Side, StateT: State + GameStateExt>(
        &self,
        position: &Position<StateT>,
    ) -> bool {
        self.is_attacked::<SideT, StateT>(position, position.king_square::<SideT>())
    }

    // sniped_by returns a bitboard containing the squares of the opposing
    // side that can "snipe" the given square
    //
    // @impl: AttackTable::sniped_by
    #[inline(always)]
    fn sniped_by<SideT: Side, StateT: State + GameStateExt>(
        &self,
        position: &Position<StateT>,
        square: Square,
    ) -> Bitboard {
        let queens = position.get_piece::<SideT::Other>(Pieces::Queen);

        // the snipers are the union of the opponent's rooks/queens that can
        // see the square on an empty board and the opponent's bishops/queens
        // that can see the square on an empty board
        (self.rook_targets(square, Bitboard::empty())
            & (queens | position.get_piece::<SideT::Other>(Pieces::Rook)))
            | (self.bishop_targets(square, Bitboard::empty())
                & (queens | position.get_piece::<SideT::Other>(Pieces::Bishop)))
    }
}

impl PieceTargetsTable for DefaultAttackTable {
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

    // pawn_targets returns the squares that the pawn targets from the given
    // square for the given side
    //
    // @impl: PieceTargetsTable::pawn_targets
    #[inline(always)]
    fn pawn_targets<SideT: Side>(&self, sq: Square) -> Bitboard {
        self.pawn_table[SideT::INDEX][sq.idx()]
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
