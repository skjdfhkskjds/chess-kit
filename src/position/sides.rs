use crate::position::position::Position;
use crate::primitives::Side;

impl Position {
    // swap_sides swaps the sides of the position
    //
    // @return: void
    // @side-effects: modifies the game state
    #[inline(always)]
    pub(crate) fn swap_sides<S: Side>(&mut self) {
        self.state.zobrist_key ^= self.zobrist.side::<S>();
        self.state.turn = S::Other::SIDE;
        self.state.zobrist_key ^= self.zobrist.side::<S::Other>();
    }
}
