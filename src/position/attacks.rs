use crate::attack_table::AttackTable;
use crate::position::{DefaultPosition, GameStateExt, PositionAttacks, PositionState, State};
use crate::primitives::{Bitboard, Pieces, Side, Square};

impl<AT, StateT> PositionAttacks for DefaultPosition<AT, StateT>
where
    AT: AttackTable,
    StateT: State + GameStateExt,
{
    // is_attacked_by returns a bitboard containing the squares occupied by
    // SideT::Other's pieces that are attacking the given SideT at the given
    // square
    //
    // @impl: PositionAttacks::is_attacked_by
    #[inline]
    fn is_attacked_by<SideT: Side>(&self, square: Square, occupancy: Bitboard) -> Bitboard {
        // idea: our square `T` is attacked iff SideT::Other has at least one
        //       piece in square `S` such that attack board generated from `T`
        //       includes `S`
        //
        // effectively relies on this idea of, if i can see you, you can see me
        //
        // the nuance not covered above is that pawn attacks are not symmetric,
        // so we reconcile this by checking the pawn attacks for SideT instead
        // of SideT::Other

        // generate the attack boards for each piece
        let king_attacks = AT::king_targets(square);
        let rook_attacks = AT::rook_targets(square, occupancy);
        let bishop_attacks = AT::bishop_targets(square, occupancy);
        let knight_attacks = AT::knight_targets(square);
        let pawn_attacks = AT::pawn_targets::<SideT>(square);

        let queens = self.get_piece::<SideT::Other>(Pieces::Queen);
        let rooks_and_queens = self.get_piece::<SideT::Other>(Pieces::Rook) | queens;
        let bishops_and_queens = self.get_piece::<SideT::Other>(Pieces::Bishop) | queens;
        let knights = self.get_piece::<SideT::Other>(Pieces::Knight);
        let pawns = self.get_piece::<SideT::Other>(Pieces::Pawn);
        let kings = self.get_piece::<SideT::Other>(Pieces::King);

        // check if there is an intersection between an attack board and that
        // piece's respective occupancy
        king_attacks & kings
            | rook_attacks & rooks_and_queens
            | bishop_attacks & bishops_and_queens
            | knight_attacks & knights
            | pawn_attacks & pawns
    }

    // is_attacked returns true if the given square on SideT is attacked by
    // SideT::Other
    //
    // note: this is the same implementation as `attacked_by`, but we leverage
    //       early termination to improve performance
    //
    // @impl: PositionAttacks::is_attacked
    #[inline(always)]
    fn is_attacked<SideT: Side>(&self, square: Square, occupancy: Bitboard) -> bool {
        // generate the attack boards for each piece
        let king_attacks = AT::king_targets(square);
        let rook_attacks = AT::rook_targets(square, occupancy);
        let bishop_attacks = AT::bishop_targets(square, occupancy);
        let knight_attacks = AT::knight_targets(square);
        let pawn_attacks = AT::pawn_targets::<SideT>(square);

        // check if there is an intersection between an attack board and that
        // piece's respective occupancy
        let queens = self.get_piece::<SideT::Other>(Pieces::Queen);
        rook_attacks.intersects(self.get_piece::<SideT::Other>(Pieces::Rook) | queens)
            || bishop_attacks.intersects(self.get_piece::<SideT::Other>(Pieces::Bishop) | queens)
            || knight_attacks.intersects(self.get_piece::<SideT::Other>(Pieces::Knight))
            || pawn_attacks.intersects(self.get_piece::<SideT::Other>(Pieces::Pawn))
            || king_attacks.intersects(self.get_piece::<SideT::Other>(Pieces::King))
    }

    // checkers returns the bitboard of pieces that are checking the side-to-
    // move's king on the board
    //
    // @impl: PositionAttacks::checkers
    #[inline(always)]
    fn checkers(&self) -> Bitboard {
        self.state().checkers()
    }

    // king_blocker_pieces gets the occupancy bitboard of all pieces that are
    // blocking SideT's king from being in check
    //
    // @impl: PositionAttacks::king_blocker_pieces
    #[inline(always)]
    fn king_blocker_pieces<SideT: Side>(&self) -> Bitboard {
        self.state().king_blocker_pieces::<SideT>()
    }

    // pinning_pieces gets the occupancy bitboard of SideT's pieces that are
    // attacking SideT::Other's king-blocker pieces
    //
    // @impl: PositionAttacks::pinning_pieces
    #[inline(always)]
    fn pinning_pieces<SideT: Side>(&self) -> Bitboard {
        self.state().pinning_pieces::<SideT>()
    }

    // check_squares returns the bitboard of squares that a given piece on SideT
    // would have to be on to deliver check to SideT::Other's king
    //
    // @impl: PositionAttacks::check_squares
    #[inline(always)]
    fn check_squares<SideT: Side>(&self, piece: Pieces) -> Bitboard {
        self.state().check_squares::<SideT>(piece)
    }
}

impl<AT, StateT> DefaultPosition<AT, StateT>
where
    AT: AttackTable,
    StateT: State + GameStateExt,
{
    // is_attacked_by_sliders returns true if the given square is attacked by
    // SideT::Other's sliding pieces
    //
    // @param: square - square to check if is attacked by SideT::Other's sliders
    // @param: occupancy - occupancy of the board
    // @return: true if the given square is attacked by SideT::Other's sliders, false otherwise
    #[inline(always)]
    pub fn is_attacked_by_sliders<SideT: Side>(&self, square: Square, occupancy: Bitboard) -> bool {
        let queens = self.get_piece::<SideT::Other>(Pieces::Queen);

        AT::rook_targets(square, occupancy)
            .intersects(queens | self.get_piece::<SideT::Other>(Pieces::Rook))
            || AT::bishop_targets(square, occupancy)
                .intersects(queens | self.get_piece::<SideT::Other>(Pieces::Bishop))
    }

    // is_checked_by returns the bitboard of squares occupied by SideT::Other's
    // pieces that are delivering check to SideT
    //
    // @return: bitboard of squares that SideT is checked by
    #[inline(always)]
    pub fn is_checked_by<SideT: Side>(&self) -> Bitboard {
        self.is_attacked_by::<SideT>(self.king_square::<SideT>(), self.total_occupancy())
    }

    // is_checked returns true if SideT is currently in check
    //
    // @return: true if SideT is checked, false otherwise
    #[inline(always)]
    pub fn is_checked<SideT: Side>(&self) -> bool {
        self.is_attacked::<SideT>(self.king_square::<SideT>(), self.total_occupancy())
    }

    // is_sniped_by returns a bitboard containing the squares occupied by
    // SideT::Other's pieces that can "snipe" the given SideT at the given
    // square
    //
    // note: we define "sniping" as a sliding piece that would be able to attack
    //       SideT at the given square assuming the ray is empty
    //
    // @param: square - square to check if is sniped by SideT::Other
    // @return: true if the given square is sniped by SideT::Other, false otherwise
    #[inline(always)]
    pub fn is_sniped_by<SideT: Side>(&self, square: Square) -> Bitboard {
        let queens = self.get_piece::<SideT::Other>(Pieces::Queen);
        let rooks_and_queens = self.get_piece::<SideT::Other>(Pieces::Rook) | queens;
        let bishops_and_queens = self.get_piece::<SideT::Other>(Pieces::Bishop) | queens;

        // the snipers are the union of SideT::Other's rooks/queens that can
        // see the square on an empty board and SideT::Other's bishops/queens
        // that can see the square on an empty board
        (AT::empty_rook_targets(square) & rooks_and_queens)
            | (AT::empty_bishop_targets(square) & bishops_and_queens)
    }
}
