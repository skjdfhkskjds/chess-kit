use crate::board::zobrist::ZobristKey;
use crate::primitives::{Castling, Move, Side, Square};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct State {
    pub turn: Side,                 // side to move
    pub castling: Castling,         // castling rights
    pub en_passant: Option<Square>, // active en passant square, if any
    pub zobrist_key: ZobristKey,    // zobrist key for the current position

    pub halfmoves: u16, // halfmove clock
    pub fullmoves: u8,  // fullmove clock

    pub next_move: Move, // next move to be made
}

impl State {
    pub fn new() -> Self {
        Self {
            turn: Side::White,
            castling: Castling::all(),
            en_passant: None,
            zobrist_key: 0,
            halfmoves: 0,
            fullmoves: 0,
            next_move: Move::default(),
        }
    }

    // reset resets the state to a new initial state
    //
    // @side-effects: modifies the `state`
    pub fn reset(&mut self) {
        self.turn = Side::White;
        self.castling = Castling::all();
        self.en_passant = None;
        self.zobrist_key = 0;
        self.halfmoves = 0;
        self.fullmoves = 0;
        self.next_move = Move::default();
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}