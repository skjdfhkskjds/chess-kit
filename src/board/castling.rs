use crate::board::board::Board;
use crate::primitives::Castling;

impl Board {
    // set_castling sets the castling rights for the given side
    //
    // @param: self - mutable reference to the board
    // @param: castling - castling rights to set
    // @return: void
    // @side-effects: modifies the `board`
    pub fn set_castling(&mut self, castling: Castling) {
        self.state.castling |= castling;
        self.state.zobrist_key ^= self.zobrist.castling(self.state.castling);
    }

    // clear_castling clears the castling rights for the given side
    //
    // @param: self - mutable reference to the board
    // @param: castling - castling rights to clear
    // @return: void
    // @side-effects: modifies the `board`
    pub fn clear_castling(&mut self, castling: Castling) {
        self.state.castling &= !castling;
        self.state.zobrist_key ^= self.zobrist.castling(self.state.castling);
    }
}
