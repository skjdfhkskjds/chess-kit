mod attacks;
mod castling;
mod display;
mod fen;
mod moves;
mod pieces;
mod position;
mod rules;
mod sides;
mod state;

pub use fen::{FENError, FENParser, Parser};
pub use position::DefaultPosition;

use crate::primitives::{
    Bitboard, Black, Castling, Move, Pieces, Side, SideCastling, Sides, Square, White, ZobristKey,
};
use std::fmt::Display;

// Position is the full composed trait of all operations that must be supported
// by a position
//
// @trait
pub trait Position:
    PositionState + PositionAttacks + PositionMoves + PositionFromFEN + Display
{
    // new creates a new, uninitialized position with zero occupancy, state, and
    // piece information
    //
    // @return: new, uninitialized position
    fn new() -> Self;

    // reset resets the position to a new initial state
    //
    // @return: void
    // @side-effect: resets the position and internal state
    fn reset(&mut self);
}

// PositionFromFEN is a trait that defines the ability to load a position from a
// FEN string
//
// @trait
pub trait PositionFromFEN {
    // load_fen loads and initializes a new position from the given FEN string
    //
    // @param: fen - FEN string to create the position from
    // @return: void, or an error if the FEN string is invalid
    // @side-effect: initializes the position and internal state
    fn load_fen(&mut self, fen: &str) -> Result<(), FENError>;
}

// PositionState is a trait that defines all state-related readonly queries on a
// given position
//
// @trait
pub trait PositionState {
    // total_occupancy gets the occupancy bitboard of all pieces on the board
    //
    // @return: full occupancy bitboard of both sides
    fn total_occupancy(&self) -> Bitboard;

    // empty_squares gets the bitboard of all empty squares on the board
    //
    // note: this is logically equivalent to `!(self.total_occupancy())`
    //
    // @return: bitboard of all empty squares on the board
    fn empty_squares(&self) -> Bitboard;

    // occupancy gets an occupancy bitboard representing all of SideT's pieces
    // on the board
    //
    // @marker: SideT - side to get the occupancy for
    // @return: occupancy bitboard of SideT
    fn occupancy<SideT: Side>(&self) -> Bitboard;

    // piece_at gets the piece type that is currently occupying the given square
    //
    // @param: square - square to get the piece type at
    // @return: piece type at the given square
    fn piece_at(&self, square: Square) -> Pieces;

    // king_square gets the square that the king of SideT is currently occupying
    //
    // @marker: SideT - side to get the king square for
    // @return: square of SideT's king
    fn king_square<SideT: Side>(&self) -> Square;

    // get_piece gets a bitboard representing all the squares that are occupied
    // by all of SideT's `piece` pieces
    //
    // @marker: SideT - side to get the piece for
    // @param: piece - piece to get the bitboard for
    // @return: occupancy bitboard of SideT's `piece`
    fn get_piece<SideT: Side>(&self, piece: Pieces) -> Bitboard;

    // turn gets the side to move from the current position
    //
    // @return: side to move
    fn turn(&self) -> Sides;

    // en_passant gets the current en passant square, if it exists
    //
    // @return: current en passant square, or None if there is no valid en
    //          passant square
    fn en_passant(&self) -> Option<Square>;

    // castling gets the representation of the current castling rights in the
    // position
    //
    // @return: current castling rights
    fn castling(&self) -> Castling;

    // key gets the unique key identifier for the current position
    //
    // @return: unique identifier of the position
    fn key(&self) -> ZobristKey;
}

// PositionAttacks is a trait that defines all the attack-related queries that
// can be made to a position
//
// @trait
pub trait PositionAttacks {
    // is_attacked_by returns a bitboard of all the squares that are attacked by
    // SideT::Other's pieces
    //
    // @marker: SideT - side to check if is attacked by
    // @param: square - square to check if is attacked by
    // @param: occupancy - occupancy to check for attacks based on
    // @return: bitboard of all the squares that are attacked by SideT::Other's pieces
    fn is_attacked_by<SideT: Side>(&self, square: Square, occupancy: Bitboard) -> Bitboard;

    // is_attacked returns true if the given square on SideT is attacked by
    // SideT::Other
    //
    // @marker: SideT - side to check if is attacked by
    // @param: square - square to check if is attacked by
    // @param: occupancy - occupancy to check for attacks based on
    // @return: true if the given square is attacked by SideT::Other, false otherwise
    fn is_attacked<SideT: Side>(&self, square: Square, occupancy: Bitboard) -> bool;

    // checkers returns the bitboard of pieces that are checking the side-to-
    // move's king on the board
    //
    // note: this function does not take the SideT parameter to semantically
    //       convey the implicitly assumed side-to-move based on the current
    //       position
    //
    // @return: bitboard of pieces that are checking the side-to-move's king
    fn checkers(&self) -> Bitboard;

    // king_blocker_pieces gets the occupancy bitboard of all pieces that are
    // blocking SideT's king from being in check
    //
    // note: a blocker piece is not necessarily on the same side as the king it
    //       is blocking, but it must be the only piece in between the sniper
    //       and the king
    //
    // @marker: SideT - side to get the king blocker pieces for
    // @return: bitboard of SideT's king's blocker pieces
    fn king_blocker_pieces<SideT: Side>(&self) -> Bitboard;

    // pinning_pieces gets the occupancy bitboard of SideT's pieces that are
    // attacking SideT::Other's king-blocker pieces
    //
    // @marker: SideT - side to get the pinning pieces for
    // @return: bitboard of pieces that are pinning SideT::Other's pieces
    fn pinning_pieces<SideT: Side>(&self) -> Bitboard;

    // check_squares returns the bitboard of squares that a given piece on SideT
    // would have to be on to deliver check to SideT::Other's king
    //
    // @marker: SideT - side to check the squares for
    // @param: piece - piece to check the squares for
    // @return: bitboard of squares that deliver check to SideT::Other
    fn check_squares<SideT: Side>(&self, piece: Pieces) -> Bitboard;
}

// PositionMoves is a trait that defines all the move-making operations that can
// be made in a position
//
// @trait
pub trait PositionMoves {
    // make_move makes the given move from the current position
    //
    // @param: mv - move to make
    // @return: void
    // @side-effect: modifies the position and internal state
    // @requires: the given move must be legal for the current position
    fn make_move(&mut self, mv: Move);

    // unmake_move undoes the given move from the current position
    //
    // @param: mv - move to undo
    // @return: void
    // @side-effect: modifies the position and internal state
    // @requires: the move must have been made last turn
    fn unmake_move(&mut self, mv: Move);

    // is_legal_move checks if the given move is legal from the current position
    // when played by SideT
    //
    // note: this method does not check that the king is still in check after
    //       the move is made, instead, we delegate this logic to the move
    //       generator. as a result, this method only checks that a king not
    //       currently in check would be left in check after the move is made
    //
    // @marker: SideT - side to check if the move is legal for
    // @param: mv - move to check if is legal
    // @return: true if the move is legal, false otherwise
    fn is_legal_move<SideT: Side>(&self, mv: Move) -> bool;

    // delivers_check checks if the given move delivers a check to SideT::Other
    // when played by SideT
    //
    // @marker: SideT - side to check if the move delivers a check for
    // @param: mv - move to check if delivers a check
    // @return: true if the move delivers a check, false otherwise
    fn delivers_check<SideT: SideCastlingSquares>(&self, mv: Move) -> bool;
}

// `SideCastlingSquares` is a trait that defines the squares for the king and
// rooks for a given side
//
// @trait
// TODO: move this somewhere else to decouple castling squares from the board
// Note: handling castling squares as constants innately prevents support for
//       chess960/alternative setups.
pub trait SideCastlingSquares: SideCastling {
    // KING is the square of that the king starts on for the given side
    const KING: Square;

    // KINGSIDE_ROOK is the square of the kingside rook for the given side
    const KINGSIDE_ROOK: Square;

    // QUEENSIDE_ROOK is the square of the queenside rook for the given side
    const QUEENSIDE_ROOK: Square;

    // KINGSIDE_DESTINATION is the square that the king moves to after kingside
    // castling
    const KINGSIDE_DESTINATION: Square;

    // KINGSIDE_ROOK_DESTINATION is the square that the kingside rook moves to
    // after kingside castling
    const KINGSIDE_ROOK_DESTINATION: Square;

    // QUEENSIDE_DESTINATION is the square that the king moves to after queenside
    // castling
    const QUEENSIDE_DESTINATION: Square;

    // QUEENSIDE_ROOK_DESTINATION is the square that the queenside rook moves to
    // after queenside castling
    const QUEENSIDE_ROOK_DESTINATION: Square;

    // QUEENSIDE_ROOK_INTERMEDIATE is the square that the queenside rook moves
    // through during queenside castling
    //
    // Note: this is required for the queenside since the space between the king
    //       and the rook is one more than during kingside castling
    const QUEENSIDE_ROOK_INTERMEDIATE: Square;
}

impl SideCastlingSquares for White {
    const KING: Square = Square::E1;
    const KINGSIDE_ROOK: Square = Square::H1;
    const QUEENSIDE_ROOK: Square = Square::A1;
    const KINGSIDE_DESTINATION: Square = Square::G1;
    const KINGSIDE_ROOK_DESTINATION: Square = Square::F1;
    const QUEENSIDE_DESTINATION: Square = Square::C1;
    const QUEENSIDE_ROOK_DESTINATION: Square = Square::D1;
    const QUEENSIDE_ROOK_INTERMEDIATE: Square = Square::B1;
}

impl SideCastlingSquares for Black {
    const KING: Square = Square::E8;
    const KINGSIDE_ROOK: Square = Square::H8;
    const QUEENSIDE_ROOK: Square = Square::A8;
    const KINGSIDE_DESTINATION: Square = Square::G8;
    const KINGSIDE_ROOK_DESTINATION: Square = Square::F8;
    const QUEENSIDE_DESTINATION: Square = Square::C8;
    const QUEENSIDE_ROOK_DESTINATION: Square = Square::D8;
    const QUEENSIDE_ROOK_INTERMEDIATE: Square = Square::B8;
}
