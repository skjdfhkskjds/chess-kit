use crate::attack_table::AttackTable;
use crate::position::position::Position;
use crate::primitives::{Castling, GameStateExt, State};

impl<AT, StateT> Position<AT, StateT>
where
    AT: AttackTable,
    StateT: State + GameStateExt,
{
    // set_castling sets the castling rights for SideT
    //
    // @param: castling - castling rights to set
    // @return: void
    // @side-effects: modifies the `position`
    #[inline(always)]
    pub(crate) fn set_castling(&mut self, castling: Castling) {
        let current_castling = self.state().castling();
        let old_key = self.zobrist.castling(current_castling);
        let new_key = self.zobrist.castling(castling);
        let state = self.state_mut();
        state.update_key(old_key);
        state.set_castling(castling);
        state.update_key(new_key);
    }
}
