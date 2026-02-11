mod generate;
mod movegen;
mod splat;

pub use movegen::DefaultMoveGenerator;

use chess_kit_primitives::{MoveList, Rank};

use chess_kit_position::{PositionAttacks, PositionMoves, PositionState};

/// `MoveType` is a type that represents the variation of moves that can be generated
///
/// @type
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MoveType {
    Quiet,
    Capture,
    Evasions,
    NonEvasions,
}

/// `MoveGenerator` is a trait that defines the contract which defines the move
/// generation service
///
/// @trait
pub trait MoveGenerator {
    /// new creates a new move generator
    ///
    /// @return: new move generator
    fn new() -> Self;

    /// generate_moves generates all the pseudo-legal moves of the given move type
    /// from the current position and pushes them to the move list
    ///
    /// @param: position - immutable reference to the position
    /// @param: list - mutable reference to the move list
    /// @param: move_type - move type to generate moves for
    /// @return: void
    /// @side-effects: modifies the `move list`
    fn generate_moves<PositionT: PositionState + PositionAttacks>(
        &self,
        position: &PositionT,
        list: &mut MoveList,
        move_type: MoveType,
    );

    /// generate_legal_moves generates all the legal moves from the current position
    /// and pushes them to the move list
    ///
    /// @param: position - immutable reference to the position
    /// @param: list - mutable reference to the move list
    /// @return: void
    /// @side-effects: modifies the `move list`
    fn generate_legal_moves<PositionT: PositionState + PositionAttacks + PositionMoves>(
        &self,
        position: &PositionT,
        list: &mut MoveList,
    );
}

// PawnOffsets is a per-side table of pawn movement offsets
chess_kit_primitives::define_sides! {
    PawnOffsets: i8 {
        PUSH => (8, -8),
        RIGHT_TARGET => (9, -9),
        LEFT_TARGET => (7, -7),
    }
}

// PawnRanks is a per-side table of pawn ranks
chess_kit_primitives::define_sides! {
    PawnRanks: Rank {
        SINGLE_STEP => (Rank::R3, Rank::R6),
        PROMOTABLE => (Rank::R7, Rank::R2),
    }
}
