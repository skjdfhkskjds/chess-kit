mod display;
mod state;

pub use state::DefaultState;

use crate::primitives::{
    Bitboard, Castling, Copyable, Pieces, Side, Sides, Square, Stack, ZobristKey,
};
use std::fmt::Display;

pub type Clock = u16;

// History is a stack of states representing the state history of a position
//
// TODO: fix the generic bounds linting warning
#[allow(type_alias_bounds)]
pub type History<StateT: State> = Stack<StateT>;

// State is a composed trait that combines all read/write operations on the
// state
//
// @trait
pub trait State:
    ReadOnlyState + WriteOnlyState + Default + Copy + Clone + Display + Copyable
{
    // new creates a new, empty state
    //
    // @return: new, empty state
    fn new() -> Self;

    // reset resets the state to a new initial state
    //
    // @side-effects: modifies the `state`
    fn reset(&mut self);
}

// ReadOnlyState is a trait that defines all read operations on the state
//
// @trait
pub trait ReadOnlyState {
    // turn returns the side to move
    //
    // @return: the side to move
    fn turn(&self) -> Sides;

    // castling returns the current castling rights
    //
    // @return: the castling rights
    fn castling(&self) -> Castling;

    // en_passant returns the current en passant square, if any
    //
    // @return: the current en passant square, if any
    fn en_passant(&self) -> Option<Square>;

    // captured_piece returns the piece that was captured to arrive at this state
    //
    // @return: the piece that was captured to arrive at this state
    fn captured_piece(&self) -> Pieces;

    // halfmoves returns the value of the current halfmove clock
    //
    // @return: the current halfmove clock
    fn halfmoves(&self) -> Clock;

    // fullmoves returns the value of the current fullmove clock
    //
    // @return: the current fullmove clock
    fn fullmoves(&self) -> Clock;

    // key returns a key representing a unique identifier of the state
    //
    // @return: the key of the state
    fn key(&self) -> ZobristKey;

    // checkers returns the bitboard of pieces that are checking the opponent's king
    //
    // @return: bitboard of pieces that are checking the opponent's king
    fn checkers(&self) -> Bitboard;

    // king_blocker_pieces returns the bitboard of the side's king's blocker
    // pieces
    //
    // note: a blocker piece is not necessarily on the same side as the king it
    //       is blocking
    //
    // @return: bitboard of the side's king's blocker pieces
    fn king_blocker_pieces<SideT: Side>(&self) -> Bitboard;

    // pinning_pieces returns the bitboard of SideT's pieces that are pinning
    // SideT::Other's pieces to their king
    //
    // @return: bitboard of pieces that are pinning SideT::Other's pieces
    fn pinning_pieces<SideT: Side>(&self) -> Bitboard;

    // check_squares returns the bitboard of squares that a given piece would
    // have to be on to deliver check to SideT::Other's king
    //
    // @param: piece - piece to check the squares for
    // @return: bitboard of squares that deliver check to SideT::Other
    fn check_squares<SideT: Side>(&self, piece: Pieces) -> Bitboard;
}

// WriteOnlyState is a trait that defines all write operations on the state
//
// @trait
pub trait WriteOnlyState {
    // set_turn sets the turn-to-move of the state
    //
    // @param: turn - the turn to set
    // @return: void
    // @side-effects: modifies the `state`
    fn set_turn(&mut self, turn: Sides);

    // set_castling sets the castling rights
    //
    // @param: castling - the castling rights
    // @return: void
    // @side-effects: modifies the `state`
    fn set_castling(&mut self, castling: Castling);

    // set_en_passant sets the en passant square, if any
    //
    // @param: en_passant - the en passant square, if any
    // @return: void
    // @side-effects: modifies the `state`
    fn set_en_passant(&mut self, en_passant: Option<Square>);

    // set_captured_piece sets the piece that was captured to arrive at this state
    //
    // @param: piece - the piece that was captured to arrive at this state
    // @return: void
    // @side-effects: modifies the `state`
    fn set_captured_piece(&mut self, piece: Pieces);

    // set_halfmoves sets the value of the current halfmove clock
    //
    // note: this operation overwrites the current value of the halfmove clock,
    //       for incremental updates see `inc_halfmoves` and `dec_halfmoves`
    //
    // @param: halfmoves - the value of the current halfmove clock
    // @return: void
    // @side-effects: modifies the `state`
    fn set_halfmoves(&mut self, halfmoves: Clock);

    // inc_halfmoves increments the value of the current halfmove clock by one
    //
    // @return: void
    // @side-effects: modifies the `state`
    fn inc_halfmoves(&mut self);

    // dec_halfmoves decrements the value of the current halfmove clock by one
    //
    // @return: void
    // @side-effects: modifies the `state`
    fn dec_halfmoves(&mut self);

    // set_fullmoves sets the value of the current fullmove clock
    //
    // note: this operation overwrites the current value of the fullmove clock,
    //       for incremental updates see `inc_fullmoves` and `dec_fullmoves`
    //
    // @param: fullmoves - the value of the current fullmove clock
    // @return: void
    // @side-effects: modifies the `state`
    fn set_fullmoves(&mut self, fullmoves: Clock);

    // inc_fullmoves increments the value of the current fullmove clock by one
    //
    // @return: void
    // @side-effects: modifies the `state`
    fn inc_fullmoves(&mut self);

    // dec_fullmoves decrements the value of the current fullmove clock by one
    //
    // @return: void
    // @side-effects: modifies the `state`
    fn dec_fullmoves(&mut self);

    // set_key sets the key for the current state
    //
    // @param: key - the key to set
    // @return: void
    // @side-effects: modifies the `state`
    fn set_key(&mut self, key: ZobristKey);

    // update_key updates the key for the current state
    //
    // note: this is NOT a `set` operation, but rather an incremental updator
    //       for the state's key
    //
    // @param: key - the key to update with
    // @return: void
    // @side-effects: modifies the `state`
    fn update_key(&mut self, key: ZobristKey);

    // set_checkers sets the bitboard of pieces that are checking the opponent's king
    //
    // @param: checkers - bitboard of pieces that are checking the opponent's king
    // @return: void
    // @side-effects: modifies the `state`
    fn set_checkers(&mut self, checkers: Bitboard);

    // set_king_blocker_pieces sets the bitboard of the side's king's blocker
    // pieces
    //
    // note: a blocker piece is not necessarily on the same side as the king it
    //       is blocking
    //
    // @param: pieces - side's king's blocker pieces
    // @return: void
    // @side-effects: modifies the `state`
    fn set_king_blocker_pieces<SideT: Side>(&mut self, pieces: Bitboard);

    // set_pinning_pieces sets the bitboard of SideT's pieces that are pinning
    // SideT::Other's pieces to their king
    //
    // @param: pieces - SideT's pieces that are pinning SideT::Other's pieces
    // @return: void
    // @side-effects: modifies the `state`
    fn set_pinning_pieces<SideT: Side>(&mut self, pieces: Bitboard);

    // set_check_squares sets the bitboard of squares that a given piece would
    // have to be on to deliver check to SideT::Other's king
    //
    // @param: piece - piece to set the squares for
    // @param: squares - bitboard of squares that deliver check to SideT::Other
    // @return: void
    // @side-effects: modifies the `state`
    fn set_check_squares<SideT: Side>(&mut self, piece: Pieces, squares: Bitboard);
}
