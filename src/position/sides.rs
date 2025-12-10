use crate::attack_table::AttackTable;
use crate::position::position::Position;
use crate::primitives::{GameStateExt, Side, State};

impl<AT, StateT> Position<AT, StateT>
where
    AT: AttackTable,
    StateT: State + GameStateExt,
{
    // swap_sides swaps the turn to move from SideT to SideT::Other
    //
    // @return: void
    // @side-effects: modifies the game state
    #[inline(always)]
    pub(crate) fn swap_sides<SideT: Side>(&mut self) {
        // compute the new key for the position
        let key = self.zobrist.side::<SideT>() ^ self.zobrist.side::<SideT::Other>();
        self.state_mut().update_key(key);

        // set the new turn in the state
        self.state_mut().set_turn(SideT::Other::SIDE);
    }
}
