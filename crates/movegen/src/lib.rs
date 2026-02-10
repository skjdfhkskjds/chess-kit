mod generate;
mod movegen;
mod splat;

pub use movegen::DefaultMoveGenerator;

use chess_kit_position::{PositionAttacks, PositionMoves, PositionState, SideCastlingSquares};
use chess_kit_primitives::{Black, MoveList, Rank, White};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MoveType {
    Quiet,
    Capture,
    Evasions,
    NonEvasions,
}

// `MoveGenerator` is a trait that defines the contract which defines the move
// generation service
//
// @trait
pub trait MoveGenerator {
    // new creates a new move generator
    //
    // @return: new move generator
    fn new() -> Self;

    // generate_moves generates all the pseudo-legal moves of the given move type
    // from the current position and pushes them to the move list
    //
    // @param: position - immutable reference to the position
    // @param: list - mutable reference to the move list
    // @param: move_type - move type to generate moves for
    // @return: void
    // @side-effects: modifies the `move list`
    fn generate_moves<PositionT: PositionState + PositionAttacks>(
        &self,
        position: &PositionT,
        list: &mut MoveList,
        move_type: MoveType,
    );

    // generate_legal_moves generates all the legal moves from the current position
    // and pushes them to the move list
    //
    // @param: position - immutable reference to the position
    // @param: list - mutable reference to the move list
    // @return: void
    // @side-effects: modifies the `move list`
    fn generate_legal_moves<PositionT: PositionState + PositionAttacks + PositionMoves>(
        &self,
        position: &PositionT,
        list: &mut MoveList,
    );
}

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
