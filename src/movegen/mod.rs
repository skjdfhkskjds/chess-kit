pub mod generate;
pub mod magics;
pub mod movegen;
pub mod moving_pieces;
pub mod sliding_pieces;

pub use movegen::MoveGenerator;

use crate::position::SideCastlingSquares;
use crate::primitives::{Black, SideRanks, Square, White};

// `SideToMove` is a trait that defines the contract required for a side to be
// able to move pieces
//
// @trait
pub trait SideToMove: SideCastlingSquares + SideRanks {
    // PAWN_PUSH_OFFSET is the offset to add to the current square of a pawn to
    // get the destination square of a pawn push
    const PAWN_PUSH_OFFSET: i8;

    // PAWN_DOUBLE_STEP_OFFSET is the offset to add to the current square of a
    // pawn to get the destination square of a pawn double step
    const PAWN_DOUBLE_STEP_OFFSET: u32;
}

impl SideToMove for White {
    const PAWN_PUSH_OFFSET: i8 = 8;
    const PAWN_DOUBLE_STEP_OFFSET: u32 = (Square::TOTAL as i8 + Self::PAWN_PUSH_OFFSET) as u32;
}

impl SideToMove for Black {
    const PAWN_PUSH_OFFSET: i8 = -8;
    const PAWN_DOUBLE_STEP_OFFSET: u32 = (Square::TOTAL as i8 + Self::PAWN_PUSH_OFFSET) as u32;
}
