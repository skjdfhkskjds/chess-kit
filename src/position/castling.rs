use crate::attack_table::AttackTable;
use crate::position::position::Position;
use crate::primitives::{Castling, GameStateExt, State};

impl<AT, S> Position<AT, S>
where
    AT: AttackTable,
    S: State + GameStateExt,
{
    // set_castling sets the castling rights for the given side
    //
    // @param: castling - castling rights to set
    // @return: void
    // @side-effects: modifies the `position`
    #[inline(always)]
    pub(crate) fn set_castling(&mut self, castling: Castling) {
        self.state
            .update_key(self.zobrist.castling(self.state.castling()));
        self.state.set_castling(castling);
        self.state.update_key(self.zobrist.castling(castling));
    }
}
