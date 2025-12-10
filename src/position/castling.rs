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
        let key = self.zobrist.castling(self.state().castling()) ^ self.zobrist.castling(castling);
        self.state_mut().update_key(key);
        self.state_mut().set_castling(castling);
    }
}
