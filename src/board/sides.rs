use crate::board::board::Board;

impl Board {
    // swap_sides swaps the sides of the board
    //
    // @return: void
    // @side-effects: modifies the game state
    #[inline(always)]
    pub(crate) fn swap_sides(&mut self) {
        self.state.zobrist_key ^= self.zobrist.side(self.state.turn);
        self.state.turn = self.state.turn.other();
        self.state.zobrist_key ^= self.zobrist.side(self.state.turn);
    }
}
