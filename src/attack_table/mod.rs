mod magics;
mod moving_pieces;
mod sliding_pieces;
mod table;

use std::sync::OnceLock;

pub(crate) use sliding_pieces::Direction;
pub use table::DefaultAttackTable;

use crate::position::Position;
use crate::primitives::{Bitboard, GameStateExt, Side, Square, State};

static DEFAULT_ATTACK_TABLE: OnceLock<DefaultAttackTable> = OnceLock::new();

pub fn default_attack_table() -> &'static DefaultAttackTable {
    DEFAULT_ATTACK_TABLE.get_or_init(DefaultAttackTable::new)
}

// AttackTable is a table that provides information about targeting/targetted
// squares for the board.
pub trait AttackTable: PieceTargetsTable + Sized + 'static {
    // new creates and initializes a new attack table
    //
    // @return: a new, initialized attack table
    fn new() -> Self;

    // attacked_by returns a bitboard containing the squares that the given side
    // is attacked by at the given square
    //
    // @param: position - immutable reference to the position
    // @param: square - square to check if is attacked
    // @return: a bitboard of the squares of the opposing side that attack `square`
    fn attacked_by<SideT: Side, StateT: State + GameStateExt>(
        &self,
        position: &Position<Self, StateT>,
        square: Square,
    ) -> Bitboard;

    // is_attacked returns true if the given square on the given side is attacked
    // by the opponent.
    //
    // @param: position - immutable reference to the position
    // @param: square - square to check if is attacked
    // @return: true if the square is attacked, false otherwise
    fn is_attacked<SideT: Side, StateT: State + GameStateExt>(
        &self,
        position: &Position<Self, StateT>,
        square: Square,
    ) -> bool;

    // is_checked returns true if the given side is checked
    //
    // @param: position - immutable reference to the position
    // @return: true if the side is checked, false otherwise
    fn is_checked<SideT: Side, StateT: State + GameStateExt>(
        &self,
        position: &Position<Self, StateT>,
    ) -> bool;

    // sniped_by returns a bitboard containing the squares of the opposing
    // side that can "snipe" (a sliding piece that can theoretically, without
    // being blocked, attack a given square) the given square
    //
    // @param: position - immutable reference to the position
    // @param: square - square to check if can sniper
    // @return: a bitboard of the squares that the given side can sniper at the given square
    fn sniped_by<SideT: Side, StateT: State + GameStateExt>(
        &self,
        position: &Position<Self, StateT>,
        square: Square,
    ) -> Bitboard;
}

// PieceTargetsTable is a table that provides information about the squares that
// a piece targets from the given square.
pub trait PieceTargetsTable {
    // king_targets returns the squares that the king targets from the given
    // square
    //
    // @param: sq - square that the king is on
    // @return: a bitboard of the king's targets from the given square
    fn king_targets(&self, square: Square) -> Bitboard;

    // knight_targets returns the squares that the knight targets from the given
    // square
    //
    // @param: sq - square that the knight is on
    // @return: a bitboard of the knight's targets from the given square
    fn knight_targets(&self, square: Square) -> Bitboard;

    // pawn_targets returns the squares that the pawn targets from the given
    // square from the given side
    //
    // note: these targets do NOT include en passant targets
    //
    // @param: sq - square that the pawn is on
    // @return: a bitboard of the pawn's targets from the given square
    fn pawn_targets<SideT: Side>(&self, square: Square) -> Bitboard;

    // rook_targets returns the squares that the rook targets from the given
    // square
    //
    // @param: sq - square that the rook is on
    // @return: a bitboard of the rook's targets from the given square
    fn rook_targets(&self, square: Square, occupancy: Bitboard) -> Bitboard;

    // bishop_targets returns the squares that the bishop targets from the given
    // square
    //
    // @param: sq - square that the bishop is on
    // @return: a bitboard of the bishop's targets from the given square
    fn bishop_targets(&self, square: Square, occupancy: Bitboard) -> Bitboard;

    // queen_targets returns the squares that the queen targets from the given
    // square
    //
    // @param: sq - square that the queen is on
    // @return: a bitboard of the queen's targets from the given square
    fn queen_targets(&self, square: Square, occupancy: Bitboard) -> Bitboard;
}
