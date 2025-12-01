use crate::primitives::{
    Castling, Clock, Move, ReadOnlyState, Sides, Square, State, WriteOnlyState, ZobristKey,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DefaultState {
    pub turn: Sides,                // side to move
    pub castling: Castling,         // castling rights
    pub en_passant: Option<Square>, // active en passant square, if any

    pub halfmoves: Clock, // halfmove clock
    pub fullmoves: Clock, // fullmove clock

    pub zobrist_key: ZobristKey, // zobrist key for the current position
    pub next_move: Move,         // next move to be made
}

impl State for DefaultState {
    // new creates a new, empty state
    //
    // @impl: State::new
    #[inline(always)]
    fn new() -> Self {
        Self {
            turn: Sides::White,
            castling: Castling::all(),
            en_passant: None,
            halfmoves: 0,
            fullmoves: 0,
            zobrist_key: 0,
            next_move: Move::default(),
        }
    }

    // reset resets the state to a new initial state
    //
    // @impl: State::reset
    #[inline(always)]
    fn reset(&mut self) {
        self.turn = Sides::White;
        self.castling = Castling::all();
        self.en_passant = None;
        self.halfmoves = 0;
        self.fullmoves = 0;
        self.zobrist_key = 0;
        self.next_move = Move::default();
    }
}

impl ReadOnlyState for DefaultState {
    // turn returns the side to move
    //
    // @impl: ReadOnlyState::turn
    #[inline(always)]
    fn turn(&self) -> Sides {
        self.turn
    }

    // castling returns the current castling rights
    //
    // @impl: ReadOnlyState::castling
    #[inline(always)]
    fn castling(&self) -> Castling {
        self.castling
    }

    // en_passant returns the current en passant square, if any
    //
    // @impl: ReadOnlyState::en_passant
    #[inline(always)]
    fn en_passant(&self) -> Option<Square> {
        self.en_passant
    }

    // halfmoves returns the value of the current halfmove clock
    //
    // @impl: ReadOnlyState::halfmoves
    #[inline(always)]
    fn halfmoves(&self) -> Clock {
        self.halfmoves
    }

    // fullmoves returns the value of the current fullmove clock
    //
    // @impl: ReadOnlyState::fullmoves
    #[inline(always)]
    fn fullmoves(&self) -> Clock {
        self.fullmoves
    }

    // key returns a key representing a unique identifier of the state
    //
    // @impl: ReadOnlyState::key
    #[inline(always)]
    fn key(&self) -> ZobristKey {
        self.zobrist_key
    }

    // next_move returns the next move to be made from the current state
    //
    // @impl: ReadOnlyState::next_move
    #[inline(always)]
    fn next_move(&self) -> Move {
        self.next_move
    }
}

impl WriteOnlyState for DefaultState {
    // set_turn sets the side to move
    //
    // @impl: WriteOnlyState::set_turn
    #[inline(always)]
    fn set_turn(&mut self, turn: Sides) {
        self.turn = turn;
    }

    // set_castling sets the castling rights
    //
    // @impl: WriteOnlyState::set_castling
    #[inline(always)]
    fn set_castling(&mut self, castling: Castling) {
        self.castling = castling;
    }

    // set_en_passant sets the en passant square, if any
    //
    // @impl: WriteOnlyState::set_en_passant
    #[inline(always)]
    fn set_en_passant(&mut self, en_passant: Option<Square>) {
        self.en_passant = en_passant;
    }

    // set_halfmoves sets the value of the current halfmove clock
    //
    // @impl: WriteOnlyState::set_halfmoves
    #[inline(always)]
    fn set_halfmoves(&mut self, halfmoves: Clock) {
        self.halfmoves = halfmoves;
    }

    // inc_halfmoves increments the value of the current halfmove clock by one
    //
    // @impl: WriteOnlyState::inc_halfmoves
    #[inline(always)]
    fn inc_halfmoves(&mut self) {
        self.halfmoves += 1;
    }

    // dec_halfmoves decrements the value of the current halfmove clock by one
    //
    // @impl: WriteOnlyState::dec_halfmoves
    #[inline(always)]
    fn dec_halfmoves(&mut self) {
        self.halfmoves -= 1;
    }

    // set_fullmoves sets the value of the current fullmove clock
    //
    // @impl: WriteOnlyState::set_fullmoves
    #[inline(always)]
    fn set_fullmoves(&mut self, fullmoves: Clock) {
        self.fullmoves = fullmoves;
    }

    // inc_fullmoves increments the value of the current fullmove clock by one
    //
    // @impl: WriteOnlyState::inc_fullmoves
    #[inline(always)]
    fn inc_fullmoves(&mut self) {
        self.fullmoves += 1;
    }

    // dec_fullmoves decrements the value of the current fullmove clock by one
    //
    // @impl: WriteOnlyState::dec_fullmoves
    #[inline(always)]
    fn dec_fullmoves(&mut self) {
        self.fullmoves -= 1;
    }

    // set_key sets the key for the current state
    //
    // @impl: WriteOnlyState::set_key
    #[inline(always)]
    fn set_key(&mut self, key: ZobristKey) {
        self.zobrist_key = key;
    }

    // update_key updates the key for the current state
    //
    // note: XOR's the current key with the given key, not a `set`
    //
    // @impl: WriteOnlyState::update_key
    #[inline(always)]
    fn update_key(&mut self, key: ZobristKey) {
        self.zobrist_key ^= key;
    }

    // set_next_move sets the next move to be made from the current state
    //
    // @impl: WriteOnlyState::set_next_move
    #[inline(always)]
    fn set_next_move(&mut self, next_move: Move) {
        self.next_move = next_move;
    }
}

impl Default for DefaultState {
    fn default() -> Self {
        Self::new()
    }
}
