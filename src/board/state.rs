use crate::board::zobrist::{Zobrist, ZobristKey};
use crate::primitives::{Bitboard, Castling, Pieces, Side, Sides, Square};

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State {
    pub turn: Side,                 // side to move
    pub castling: Castling,         // castling rights
    pub en_passant: Option<Square>, // active en passant square, if any
    pub zobrist_key: ZobristKey,    // zobrist key for the current position

    pub halfmoves: u8, // halfmove clock
    pub fullmoves: u8, // fullmove clock
}

impl State {
    pub fn new() -> Self {
        Self {
            turn: Sides::WHITE,
            castling: Castling::all(),
            en_passant: None,
            zobrist_key: 0,
            halfmoves: 0,
            fullmoves: 0,
        }
    }

    // init initializes the state with the given position
    //
    // @param: side - side to move
    // @param: castling - castling rights
    // @param: en_passant - en passant square
    // @param: bitboards - bitboards to generate the zobrist key for
    // @param: zobrist - zobrist instance to use for generating the zobrist key
    // @side-effects: modifies the `state`
    // @requires: `bitboards` is initialized
    pub fn init(
        &mut self,
        side: Side,
        castling: Castling,
        en_passant: Option<Square>,
        bitboards: [[Bitboard; Pieces::TOTAL]; Sides::TOTAL],
        zobrist: Zobrist,
    ) {
        self.turn = side;
        self.castling = castling;
        self.en_passant = en_passant;
        self.zobrist_key = zobrist.key(side, castling, en_passant, bitboards);
    }
}
