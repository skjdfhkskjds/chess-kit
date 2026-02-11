use crate::{EvalState, Score};
use chess_kit_collections::Copyable;
use chess_kit_primitives::{Pieces, Side, Square};

/// NoOpEvalState is a no-op implementation of the EvalState trait that does not
/// perform any evaluation
///
/// note: this is useful for testing and benchmarking and all use cases where
///       evaluation accumulation can be skipped when making a move
#[derive(Copy, Clone, Default)]
pub struct NoOpEvalState;

impl EvalState for NoOpEvalState {
    /// new creates a new, empty eval state
    ///
    /// @impl: EvalState::new
    #[inline(always)]
    fn new() -> Self {
        Self
    }

    /// score returns the evaluation score of this state
    ///
    /// @impl: EvalState::score
    #[inline(always)]
    fn score(&mut self) -> Score {
        0
    }

    /// on_set_piece is the incremental update callback that fires when a piece
    /// is set on the board for the given side
    ///
    /// @impl: EvalState::on_set_piece
    #[inline(always)]
    fn on_set_piece<SideT: Side>(&mut self, _: Pieces, _: Square) {}

    /// on_remove_piece is the incremental update callback that fires when a piece
    /// is removed from the board for the given side
    ///
    /// @impl: EvalState::on_remove_piece
    #[inline(always)]
    fn on_remove_piece<SideT: Side>(&mut self, _: Pieces, _: Square) {}
}

impl Copyable for NoOpEvalState {
    /// copy_from copies the contents of another NoOpEvalState into this one
    ///
    /// @impl: Copyable::copy_from
    #[inline(always)]
    fn copy_from(&mut self, other: &Self) {
        *self = *other;
    }
}
