use crate::{EvalState, Score};
use chess_kit_collections::Copyable;
use chess_kit_position::PositionView;
use chess_kit_primitives::MoveDelta;

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
    #[inline]
    fn from_position<P: PositionView>(_: &P) -> Self {
        Self
    }

    #[inline]
    fn apply(&mut self, _: MoveDelta) {}

    /// score returns the evaluation score of this state
    ///
    /// @impl: EvalState::score
    #[inline]
    fn score(&mut self) -> Score {
        0
    }
}

impl Copyable for NoOpEvalState {
    /// copy_from copies the contents of another NoOpEvalState into this one
    ///
    /// @impl: Copyable::copy_from
    #[inline]
    fn copy_from(&mut self, other: &Self) {
        *self = *other;
    }
}
