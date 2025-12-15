use crate::eval::{EvalState, Score};
use crate::position::Position;
use crate::primitives::{Pieces, Side, Square};
use chess_kit_collections::Copyable;

// NoOpEvalState is a no-op implementation of the EvalState trait that does not
// perform any evaluation
//
// note: this is useful for testing and benchmarking and all use cases where
//       evaluation accumulation can be skipped when making a move
#[derive(Copy, Clone, Default)]
pub struct NoOpEvalState;

impl EvalState for NoOpEvalState {
    #[inline(always)]
    fn new() -> Self {
        Self
    }

    #[inline(always)]
    fn init<PositionT: Position>(&mut self, _: &PositionT) {}

    #[inline(always)]
    fn score(&mut self) -> Score {
        0
    }

    #[inline(always)]
    fn on_set_piece<SideT: Side>(&mut self, _: Pieces, _: Square) {}

    #[inline(always)]
    fn on_remove_piece<SideT: Side>(&mut self, _: Pieces, _: Square) {}
}

impl Copyable for NoOpEvalState {
    #[inline(always)]
    fn copy_from(&mut self, other: &Self) {
        *self = *other;
    }
}
