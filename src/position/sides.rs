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
        let side_delta = self.zobrist.side::<SideT>();
        let other_delta = self.zobrist.side::<SideT::Other>();
        let state = self.state_mut();
        state.update_key(side_delta);
        state.set_turn(SideT::Other::SIDE);
        state.update_key(other_delta);
    }
}
