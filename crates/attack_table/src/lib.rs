mod magics;
mod moving_pieces;
mod sliding_pieces;
mod table;

pub(crate) use moving_pieces::{NOT_A_FILE, NOT_H_FILE};
pub use table::DefaultAttackTable;

use chess_kit_primitives::{Bitboard, Side, Square};

/// `PawnDirections` is a type that enumerates the three possible directions that a
/// pawn can move
///
/// @type
pub enum PawnDirections {
    Up,
    Right,
    Left,
}

impl PawnDirections {
    pub const TOTAL: usize = 3;
}

/// `AttackTable` is a table that provides information about targeting/targetted
/// squares for the board
///
/// note: the usage pattern of this trait is a phantom marker type that is used
///       to allow *constant* resolution of the attack table
///
/// @trait
pub trait AttackTable {
    /// king_targets returns the squares that the king targets from the given
    /// square
    ///
    /// @param: square - square that the king is on
    /// @return: a bitboard of the king's targets from the given square
    fn king_targets(square: Square) -> Bitboard;

    /// knight_targets returns the squares that the knight targets from the given
    /// square
    ///
    /// @param: square - square that the knight is on
    /// @return: a bitboard of the knight's targets from the given square
    fn knight_targets(square: Square) -> Bitboard;

    /// pawn_targets returns the squares that the pawn targets from the given
    /// square from the given side
    ///
    /// note: these targets do NOT include en passant targets
    ///
    /// @param: square - square that the pawn is on
    /// @return: a bitboard of the pawn's targets from the given square
    fn pawn_targets<SideT: Side>(square: Square) -> Bitboard;

    /// pawn_pushes returns the squares that the pawn pushes to from the given
    /// square
    ///
    /// @param: square - square that the pawn is on
    /// @return: a bitboard of the pawn's pushes  from the given square
    fn pawn_pushes<SideT: Side>(square: Square) -> Bitboard;

    /// all_pawn_targets returns the squares that the all the pawns on the given
    /// squares target from the given side in the given direction
    ///
    /// @param: squares - bitboard of the squares that the pawns are on
    /// @param: direction - direction of the pawns
    /// @return: a bitboard of the squares that the all the pawns on the given
    ///          squares target from the given side in the given direction
    fn all_pawn_targets<SideT: Side>(squares: Bitboard, direction: PawnDirections) -> Bitboard;

    /// empty_rook_targets returns the squares that the rook targets from the given
    /// square on an empty board
    ///
    /// @param: square - square that the rook is on
    /// @return: a bitboard of the rook's targets from the given square
    fn empty_rook_targets(square: Square) -> Bitboard;

    /// rook_targets returns the squares that the rook targets from the given
    /// square
    ///
    /// @param: square - square that the rook is on
    /// @return: a bitboard of the rook's targets from the given square
    fn rook_targets(square: Square, occupancy: Bitboard) -> Bitboard;

    /// empty_bishop_targets returns the squares that the bishop targets from the given
    /// square on an empty board
    ///
    /// @param: square - square that the bishop is on
    /// @return: a bitboard of the bishop's targets from the given square
    fn empty_bishop_targets(square: Square) -> Bitboard;

    /// bishop_targets returns the squares that the bishop targets from the given
    /// square
    ///
    /// @param: square - square that the bishop is on
    /// @return: a bitboard of the bishop's targets from the given square
    fn bishop_targets(square: Square, occupancy: Bitboard) -> Bitboard;

    /// queen_targets returns the squares that the queen targets from the given
    /// square
    ///
    /// @param: square - square that the queen is on
    /// @return: a bitboard of the queen's targets from the given square
    fn queen_targets(square: Square, occupancy: Bitboard) -> Bitboard;
}
