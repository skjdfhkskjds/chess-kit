use crate::position::position::Position;
use crate::primitives::{Side, State};

impl<S: State> Position<S> {
    // swap_sides swaps the sides of the position
    //
    // @return: void
    // @side-effects: modifies the game state
    #[inline(always)]
    pub(crate) fn swap_sides<SideT: Side>(&mut self) {
        self.state.update_key(self.zobrist.side::<SideT>());
        self.state.set_turn(SideT::Other::SIDE);
        self.state.update_key(self.zobrist.side::<SideT::Other>());
    }
}
