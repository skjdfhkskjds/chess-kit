use crate::primitives::{Side, Castling, Square};
use crate::board::zobrist::ZobristKey;

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State {
    pub turn: Side,                 // side to move
    pub castling: Castling,         // castling rights
    pub en_passant: Option<Square>, // active en passant square, if any
    pub zobrist_key: ZobristKey,    // zobrist key for the current position

    pub halfmoves: u8,              // halfmove clock
    pub fullmoves: u8,              // fullmove clock
}
