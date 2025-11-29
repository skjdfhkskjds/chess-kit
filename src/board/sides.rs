use crate::board::board::Board;

impl Board {
    // swap_sides swaps the sides of the board
    //
    // @param: self - mutable reference to the board
    // @return: void
    // @side-effects: modifies the `board`
    pub fn swap_sides(&mut self) {
        self.state.zobrist_key ^= self.zobrist.side(self.state.turn);
        self.state.turn = self.state.turn.other();
        self.state.zobrist_key ^= self.zobrist.side(self.state.turn);
    }
}
