use crate::attack_table::AttackTable;
use crate::position::Position;
use crate::primitives::{Bitboard, GameStateExt, Pieces, Side, Square, State};

impl<AT, StateT> Position<AT, StateT>
where
    AT: AttackTable,
    StateT: State + GameStateExt,
{
    // is_attacked_by returns a bitboard containing the squares occupied by
    // SideT::Other's pieces that are attacking the given SideT at the given
    // square
    //
    // @param: square - square to check if is attacked by SideT::Other
    // @return: bitboard of the occupied squares of SideT::Other that are
    //          attacking the given square
    #[inline(always)]
    pub fn is_attacked_by<SideT: Side>(&self, square: Square) -> Bitboard {
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
        let occupancy = self.total_occupancy();
        let king_attacks = self.attack_table.king_targets(square);
        let rook_attacks = self.attack_table.rook_targets(square, occupancy);
        let bishop_attacks = self.attack_table.bishop_targets(square, occupancy);
        let knight_attacks = self.attack_table.knight_targets(square);
        let pawn_attacks = self.attack_table.pawn_targets::<SideT>(square);

        // check if there is an intersection between an attack board and that
        // piece's respective occupancy
        let queens = self.get_piece::<SideT::Other>(Pieces::Queen);
        king_attacks & self.get_piece::<SideT::Other>(Pieces::King)
            | rook_attacks & (self.get_piece::<SideT::Other>(Pieces::Rook) | queens)
            | bishop_attacks & (self.get_piece::<SideT::Other>(Pieces::Bishop) | queens)
            | knight_attacks & self.get_piece::<SideT::Other>(Pieces::Knight)
            | pawn_attacks & self.get_piece::<SideT::Other>(Pieces::Pawn)
    }

    // is_attacked returns true if the given square on SideT is attacked by
    // SideT::Other
    //
    // note: this is the same implementation as `attacked_by`, but we leverage
    //       early termination to improve performance
    //
    // @param: square - square to check if is attacked by SideT::Other
    // @param: occupancy - occupancy of the board
    // @return: true if the given square is attacked by SideT::Other, false otherwise
    #[inline(always)]
    pub fn is_attacked<SideT: Side>(&self, square: Square, occupancy: Bitboard) -> bool {
        // generate the attack boards for each piece
        let king_attacks = self.attack_table.king_targets(square);
        let rook_attacks = self.attack_table.rook_targets(square, occupancy);
        let bishop_attacks = self.attack_table.bishop_targets(square, occupancy);
        let knight_attacks = self.attack_table.knight_targets(square);
        let pawn_attacks = self.attack_table.pawn_targets::<SideT>(square);

        // check if there is an intersection between an attack board and that
        // piece's respective occupancy
        let queens = self.get_piece::<SideT::Other>(Pieces::Queen);
        (rook_attacks & (self.get_piece::<SideT::Other>(Pieces::Rook) | queens)).not_empty()
            || (bishop_attacks & (self.get_piece::<SideT::Other>(Pieces::Bishop) | queens))
                .not_empty()
            || (knight_attacks & self.get_piece::<SideT::Other>(Pieces::Knight)).not_empty()
            || (pawn_attacks & self.get_piece::<SideT::Other>(Pieces::Pawn)).not_empty()
            || (king_attacks & self.get_piece::<SideT::Other>(Pieces::King)).not_empty()
    }

    // is_attacked_by_sliders returns true if the given square is attacked by
    // SideT::Other's sliding pieces
    //
    // @param: square - square to check if is attacked by SideT::Other's sliders
    // @param: occupancy - occupancy of the board
    // @return: true if the given square is attacked by SideT::Other's sliders, false otherwise
    #[inline(always)]
    pub fn is_attacked_by_sliders<SideT: Side>(&self, square: Square, occupancy: Bitboard) -> bool {
        let queens = self.get_piece::<SideT::Other>(Pieces::Queen);

        (self.attack_table.rook_targets(square, occupancy)
            & (queens | self.get_piece::<SideT::Other>(Pieces::Rook)))
        .not_empty()
            || (self.attack_table.bishop_targets(square, occupancy)
                & (queens | self.get_piece::<SideT::Other>(Pieces::Bishop)))
            .not_empty()
    }

    // is_checked_by returns the bitboard of squares occupied by SideT::Other's
    // pieces that are delivering check to SideT
    //
    // @return: bitboard of squares that SideT is checked by
    #[inline(always)]
    pub fn is_checked_by<SideT: Side>(&self) -> Bitboard {
        self.is_attacked_by::<SideT>(self.king_square::<SideT>())
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

        // the snipers are the union of SideT::Other's rooks/queens that can
        // see the square on an empty board and SideT::Other's bishops/queens
        // that can see the square on an empty board
        (self.attack_table.rook_targets(square, Bitboard::empty())
            & (queens | self.get_piece::<SideT::Other>(Pieces::Rook)))
            | (self.attack_table.bishop_targets(square, Bitboard::empty())
                & (queens | self.get_piece::<SideT::Other>(Pieces::Bishop)))
    }
}
