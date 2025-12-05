use crate::attack_table::AttackTable;
use crate::position::Position;
use crate::primitives::{Bitboard, GameStateExt, Pieces, Side, Square, State};

impl<AT, StateT> Position<AT, StateT>
where
    AT: AttackTable,
    StateT: State + GameStateExt,
{
    // attacked_by returns a bitboard containing the squares that the given side
    // is attacked by at the given square
    //
    // @impl: AttackTable::attacked_by
    #[inline(always)]
    pub fn attacked_by<SideT: Side>(&self, square: Square) -> Bitboard {
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

    // is_attacked returns true if the given square on the given side is attacked
    // by the opponent
    //
    // note: this is the same implementation as `attacked_by`, but we leverage
    //       early termination to improve performance
    //
    // @impl: AttackTable::is_attacked
    #[inline(always)]
    pub fn is_attacked<SideT: Side>(&self, square: Square) -> bool {
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
        !(rook_attacks & (self.get_piece::<SideT::Other>(Pieces::Rook) | queens)).is_empty()
            || !(bishop_attacks & (self.get_piece::<SideT::Other>(Pieces::Bishop) | queens))
                .is_empty()
            || !(knight_attacks & self.get_piece::<SideT::Other>(Pieces::Knight)).is_empty()
            || !(pawn_attacks & self.get_piece::<SideT::Other>(Pieces::Pawn)).is_empty()
            || !(king_attacks & self.get_piece::<SideT::Other>(Pieces::King)).is_empty()
    }

    // is_checked returns true if the given side is checked
    //
    // @impl: AttackTable::is_checked
    #[inline(always)]
    pub fn is_checked<SideT: Side>(&self) -> bool {
        self.is_attacked::<SideT>(self.king_square::<SideT>())
    }

    // sniped_by returns a bitboard containing the squares of the opposing
    // side that can "snipe" the given square
    //
    // @impl: AttackTable::sniped_by
    #[inline(always)]
    pub fn sniped_by<SideT: Side>(&self, square: Square) -> Bitboard {
        let queens = self.get_piece::<SideT::Other>(Pieces::Queen);

        // the snipers are the union of the opponent's rooks/queens that can
        // see the square on an empty board and the opponent's bishops/queens
        // that can see the square on an empty board
        (self.attack_table.rook_targets(square, Bitboard::empty())
            & (queens | self.get_piece::<SideT::Other>(Pieces::Rook)))
            | (self.attack_table.bishop_targets(square, Bitboard::empty())
                & (queens | self.get_piece::<SideT::Other>(Pieces::Bishop)))
    }
}
