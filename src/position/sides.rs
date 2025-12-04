use crate::attack_table::AttackTable;
use crate::position::position::Position;
use crate::primitives::{GameStateExt, Side, State};

impl<AT, StateT> Position<AT, StateT>
where
    AT: AttackTable,
    StateT: State + GameStateExt,
{
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
