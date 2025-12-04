mod magics;
mod moving_pieces;
mod sliding_pieces;
mod table;

pub(crate) use sliding_pieces::Direction;
pub use table::{default_attack_table, DefaultAttackTable};

use crate::primitives::{Bitboard, Side, Square};

// AttackTable is a table that provides information about targeting/targetted
// squares for the board.
// 
// @trait
pub trait AttackTable: Sized + 'static {
    // new creates and initializes a new attack table
    //
    // @return: a new, initialized attack table
    fn new() -> Self;

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
