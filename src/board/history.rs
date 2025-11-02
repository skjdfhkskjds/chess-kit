use crate::board::state::State;

const MAX_FULLMOVES: usize = u8::MAX as usize;

pub struct History {
    current: usize,                 // index of the current state
    states: [State; MAX_FULLMOVES], // stack of previous states
}

impl History {
    pub fn new() -> Self {
        Self {
            current: 0,
            states: [State::default(); MAX_FULLMOVES],
        }
    }

    // push adds a new state entry to the history
    // 
    // @param: state - state to add to the history
    // @side-effects: modifies the history, increments the current index
    // @requires: the current index is less than the fullmove limit
    pub fn push(&mut self, state: State) {
        assert!(self.current < MAX_FULLMOVES, "history is full");
        self.states[self.current] = state;
        self.current += 1;
    }

    // pop removes the last state entry from the history and returns it
    //
    // @return: the last state entry, if any
    // @side-effects: modifies the history, decrements the current index
    pub fn pop(&mut self) -> Option<State> {
        if self.current == 0 {
            return None;
        }
        self.current -= 1;
        Some(self.states[self.current])
    }

    // size returns the number of state entries in the history
    //
    // @return: the number of state entries in the history
    pub fn size(&self) -> usize {
        self.current
    }

    // is_empty returns true if the history is empty
    //
    // @return: true if the history is empty
    pub fn is_empty(&self) -> bool {
        self.current == 0
    }

    // is_full returns true if the history is full
    //
    // @return: true if the history is full
    pub fn is_full(&self) -> bool {
        self.current == MAX_FULLMOVES
    }

    // clear resets the history to an empty state
    // 
    // Note: this technically doesn't reset the states array, but for all
    // intents and purposes, clears the history since the states array is
    // indexed and accessed only by the current index
    //
    // @side-effects: sets the current index to 0
    pub fn clear(&mut self) {
        self.current = 0;
    }
}
