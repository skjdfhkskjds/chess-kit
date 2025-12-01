use crate::board::board::Board;
use crate::primitives::Castling;

impl Board {
    // set_castling sets the castling rights for the given side
    //
    // @param: castling - castling rights to set
    // @return: void
    // @side-effects: modifies the `board`
    #[inline(always)]
    pub(crate) fn set_castling(&mut self, castling: Castling) {
        self.state.zobrist_key ^= self.zobrist.castling(self.state.castling);
        self.state.castling = castling;
        self.state.zobrist_key ^= self.zobrist.castling(self.state.castling);
    }
}
