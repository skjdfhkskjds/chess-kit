use chess_kit_collections::Stack;

use crate::{Accumulator, EvalState};

/// DefaultAccumulator is the default implementation of the Accumulator trait
///
/// @type
pub struct DefaultAccumulator<EvalStateT: EvalState> {
    stack: Stack<EvalStateT>,
}

impl<EvalStateT: EvalState> Accumulator<EvalStateT> for DefaultAccumulator<EvalStateT> {
    /// new creates a new, uninitialized accumulator
    ///
    /// @impl: Accumulator::new
    #[inline(always)]
    fn new() -> Self {
        Self {
            stack: Stack::new(),
        }
    }

    /// reset resets the accumulator to a new initial state
    ///
    /// @impl: Accumulator::reset
    #[inline(always)]
    fn reset(&mut self) {
        self.stack.clear();
    }

    /// push pushes a new state onto the accumulator
    ///
    /// @impl: Accumulator::push
    #[inline(always)]
    fn push(&mut self, state: EvalStateT) {
        self.stack.push(state);
    }

    /// push_next pushes a new state entry onto the accumulator by deriving it
    /// from the copy of the current eval state
    ///
    /// @impl: Accumulator::push_next
    #[inline(always)]
    fn push_next(&mut self) -> &mut EvalStateT {
        self.stack.push_next()
    }

    /// pop pops the last state from the accumulator
    ///
    /// @impl: Accumulator::pop
    #[inline(always)]
    fn pop(&mut self) {
        self.stack.pop();
    }

    /// latest returns a reference to the latest state in the accumulator
    ///
    /// @impl: Accumulator::latest
    #[inline(always)]
    fn latest(&self) -> &EvalStateT {
        self.stack.top()
    }

    /// latest_mut returns a mutable reference to the latest state in the
    /// accumulator
    ///
    /// @impl: Accumulator::latest_mut
    #[inline(always)]
    fn latest_mut(&mut self) -> &mut EvalStateT {
        self.stack.top_mut()
    }
}

impl<EvalStateT: EvalState> Default for DefaultAccumulator<EvalStateT> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
