mod generate;
mod movegen;
mod move_type;

pub use move_type::MoveType;
pub use movegen::MoveGenerator;

use crate::position::SideCastlingSquares;
use crate::primitives::{Black, SideRanks, White};

// `SideToMove` is a trait that defines the contract required for a side to be
// able to move pieces
//
// @trait
pub trait SideToMove: SideCastlingSquares + SideRanks {
    // PAWN_PUSH_OFFSET is the offset to add to the current square of a pawn to
    // get the destination square of a pawn push
    const PAWN_PUSH_OFFSET: i8;

    // PAWN_RIGHT_TARGET_OFFSET is the offset to add to the current square of a
    // pawn to get the destination square of a pawn target on the relative right
    const PAWN_RIGHT_TARGET_OFFSET: i8;

    // PAWN_LEFT_TARGET_OFFSET is the offset to add to the current square of a
    // pawn to get the destination square of a pawn target on the relative left
    const PAWN_LEFT_TARGET_OFFSET: i8;
}

impl SideToMove for White {
    const PAWN_PUSH_OFFSET: i8 = 8;
    const PAWN_RIGHT_TARGET_OFFSET: i8 = 9;
    const PAWN_LEFT_TARGET_OFFSET: i8 = 7;
}

impl SideToMove for Black {
    const PAWN_PUSH_OFFSET: i8 = -8;
    const PAWN_RIGHT_TARGET_OFFSET: i8 = -9;
    const PAWN_LEFT_TARGET_OFFSET: i8 = -7;
}
