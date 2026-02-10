use chess_kit_attack_table::AttackTable;
use crate::position::DefaultPosition;
use crate::State;
use chess_kit_primitives::{Side, ZobristTable};

impl<AT, StateT> DefaultPosition<AT, StateT>
where
    AT: AttackTable,
    StateT: State,
{
    // swap_sides swaps the turn to move from SideT to SideT::Other
    //
    // @return: void
    // @side-effects: modifies the game state
    #[inline(always)]
    pub(crate) fn swap_sides<SideT: Side>(&mut self) {
        // compute the new key for the position
        let key = ZobristTable::side::<SideT>() ^ ZobristTable::side::<SideT::Other>();
        self.state_mut().update_key(key);

        // set the new turn in the state
        self.state_mut().set_turn(SideT::Other::SIDE);
    }
}
