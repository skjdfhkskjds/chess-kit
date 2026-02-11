pub mod accumulator;
pub mod noop_eval;
pub mod psqt;

pub use accumulator::DefaultAccumulator;
pub use noop_eval::NoOpEvalState;
pub use psqt::PSQTEvalState;

use chess_kit_collections::Copyable;
use chess_kit_primitives::{Pieces, Side, Square};

pub type Score = i32;

pub trait EvalState: Copyable {
    /// new creates a new, empty eval state
    ///
    /// @return: new, empty eval state
    fn new() -> Self;

    /// score returns the evaluation score of this state
    ///
    /// @return: evaluation score of this state
    fn score(&mut self) -> Score;

    /// on_set_piece is the incremental update callback that fires when a piece
    /// is set on the board for the given side
    ///
    /// @marker: SideT - the side that the piece was set for
    /// @param: piece - piece that was set
    /// @param: square - square that the piece was set on
    /// @return: void
    /// @side-effects: modifies the eval state
    fn on_set_piece<SideT: Side>(&mut self, piece: Pieces, square: Square);

    /// on_remove_piece is the incremental update callback that fires when a piece
    /// is removed from the board for the given side
    ///
    /// @marker: SideT - the side that the piece was removed from
    /// @param: piece - piece that was removed
    /// @param: square - square that the piece was removed from
    /// @return: void
    /// @side-effects: modifies the eval state
    fn on_remove_piece<SideT: Side>(&mut self, piece: Pieces, square: Square);
}

/// `Accumulator` is a trait that defines a type that provides operations to
/// accumulate evaluation states
///
/// @trait
pub trait Accumulator<EvalStateT: EvalState> {
    /// new creates a new, uninitialized accumulator
    ///
    /// @return: new, uninitialized accumulator
    fn new() -> Self;

    /// reset resets the accumulator to a new initial state
    ///
    /// @return: void
    /// @side-effects: modifies the accumulator
    fn reset(&mut self);

    /// push pushes a new state onto the accumulator
    ///
    /// @param: state - state to push onto the accumulator
    /// @return: void
    /// @side-effects: modifies the accumulator
    fn push(&mut self, state: EvalStateT);

    /// push_next pushes a new state entry onto the accumulator by deriving it
    /// from the copy of the current eval state
    ///
    /// @return: mutable reference to the newly pushed state
    /// @side-effects: modifies the accumulator
    fn push_next(&mut self) -> &mut EvalStateT;

    /// pop pops the last state from the accumulator
    ///
    /// @return: void
    /// @side-effects: modifies the accumulator
    fn pop(&mut self);

    /// latest returns a reference to the latest state in the accumulator
    ///
    /// @return: reference to the latest state in the accumulator
    fn latest(&self) -> &EvalStateT;

    /// latest_mut returns a mutable reference to the latest state in the
    /// accumulator
    ///
    /// @return: mutable reference to the latest state in the accumulator
    fn latest_mut(&mut self) -> &mut EvalStateT;
}
