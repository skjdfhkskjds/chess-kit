use chess_kit_attack_table::AttackTable;
use crate::position::DefaultPosition;
use crate::State;
use chess_kit_primitives::{Castling, ZobristTable};

impl<AT, StateT> DefaultPosition<AT, StateT>
where
    AT: AttackTable,
    StateT: State,
{
    // set_castling sets the castling rights for SideT
    //
    // @param: castling - castling rights to set
    // @return: void
    // @side-effects: modifies the `position`
    #[inline(always)]
    pub(crate) fn set_castling(&mut self, castling: Castling) {
        let key =
            ZobristTable::castling(self.state().castling()) ^ ZobristTable::castling(castling);
        self.state_mut().update_key(key);
        self.state_mut().set_castling(castling);
    }
}
