mod generate;
mod move_type;
mod movegen;

pub use move_type::MoveType;
pub use movegen::MoveGenerator;

use crate::position::SideCastlingSquares;
use crate::primitives::{Black, Rank, White};

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

// `SideRanks` is a trait that provides specific rank information with respect
// to a given side
//
// @trait
pub trait SideRanks {
    // SINGLE_STEP_RANK is the rank that a pawn can single step to
    const SINGLE_STEP_RANK: Rank;

    // PROMOTABLE_RANK is the rank that a pawn is on when it promotes on its
    // next move
    const PROMOTABLE_RANK: Rank;
}

impl SideRanks for White {
    const SINGLE_STEP_RANK: Rank = Rank::R3;
    const PROMOTABLE_RANK: Rank = Rank::R7;
}

impl SideRanks for Black {
    const SINGLE_STEP_RANK: Rank = Rank::R6;
    const PROMOTABLE_RANK: Rank = Rank::R2;
}
