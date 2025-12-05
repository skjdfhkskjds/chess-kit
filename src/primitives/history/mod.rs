mod iterator;

use crate::primitives::State;

const MAX_FULLMOVES: usize = u8::MAX as usize;

pub struct History<S: State> {
    pub(crate) current: usize,             // index of the current state
    pub(crate) states: [S; MAX_FULLMOVES], // stack of previous states
}

impl<S: State> History<S> {
    // new creates a new history with all states initialized to the default
    // state
    //
    // @return: new history
    pub fn new() -> Self {
        Self {
            current: 0,
            states: [S::default(); MAX_FULLMOVES],
        }
    }

    // init initializes the history with the given state
    //
    // @param: state - state to initialize the history with
    // @return: void
    // @side-effects: modifies the history
    pub fn init(&mut self, state: S) {
        self.clear();
        self.push(state);
    }

    // push adds a new state entry to the history
    //
    // @param: state - state to add to the history
    // @return: void
    // @side-effects: modifies the history, increments the current index
    // @requires: the current index is less than the fullmove limit
    #[inline(always)]
    pub fn push(&mut self, state: S) {
        debug_assert!(self.current < MAX_FULLMOVES, "history is full");
        self.states[self.current] = state;
        self.current += 1;
    }

    // push_next adds a new state entry to the history by deriving it from the
    // copy of the current state entry's header
    //
    // @return: mutable reference to the newly pushed state
    // @side-effects: modifies the history, increments the current index
    // @requires: the history is non-empty and not full
    #[inline(always)]
    pub fn push_next(&mut self) -> &mut S {
        debug_assert!(self.current > 0, "cannot clone from an empty history");
        debug_assert!(self.current < MAX_FULLMOVES, "history is full");

        let src = self.current - 1;
        let dst = self.current;
        let src_state = self.states[src];
        self.states[dst].copy_header_from(&src_state);
        self.current += 1;
        &mut self.states[dst]
    }

    // pop removes the last state entry from the history and returns it
    //
    // @return: the new top of stack if a pop occurred, otherwise None
    // @side-effects: modifies the history, decrements the current index
    // @requires: the current index is greater than 1
    #[inline(always)]
    pub fn pop(&mut self) -> Option<&mut S> {
        if self.current <= 1 {
            return None;
        }

        self.current -= 1;
        let idx = self.current - 1;
        Some(&mut self.states[idx])
    }

    // current returns an immutable reference to the top state entry
    //
    // @return: reference to the current state entry
    // @requires: the history is non-empty
    #[inline(always)]
    pub fn current(&self) -> &S {
        debug_assert!(self.current > 0, "history is empty");
        &self.states[self.current - 1]
    }

    // current_mut returns a mutable reference to the top state entry
    //
    // @return: mutable reference to the current state entry
    // @requires: the history is non-empty
    #[inline(always)]
    pub fn current_mut(&mut self) -> &mut S {
        debug_assert!(self.current > 0, "history is empty");
        let idx = self.current - 1;
        &mut self.states[idx]
    }

    // size returns the number of state entries in the history
    //
    // @return: the number of state entries in the history
    #[inline(always)]
    pub fn size(&self) -> usize {
        self.current
    }

    // is_empty returns true if the history is empty
    //
    // @return: true if the history is empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.current == 0
    }

    // is_full returns true if the history is full
    //
    // @return: true if the history is full
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        self.current == MAX_FULLMOVES
    }

    // clear resets the history to an empty state
    //
    // @return: void
    // @side-effects: sets the current index to 0
    #[inline(always)]
    pub fn clear(&mut self) {
        self.current = 0;
    }
}

impl<S: State> Default for History<S> {
    fn default() -> Self {
        Self::new()
    }
}