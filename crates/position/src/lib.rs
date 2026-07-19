mod attacks;
mod castling;
mod display;
mod errors;
mod gamestate;
mod moves;
mod pieces;
mod position;
mod rules;
mod setup;
mod sides;
mod snapshot;
mod state;

pub use errors::PlayError;
pub use gamestate::DrawState;
pub(crate) use gamestate::{History, PositionState};
pub use position::DefaultPosition;
pub use setup::{FENError, Fen, Setup};
pub use snapshot::PositionSnapshot;

use chess_kit_primitives::{
    Bitboard, Castling, Move, MoveDelta, Pieces, Side, Sides, Square, ZobristKey,
};
use std::fmt::Display;

/// `Position` is the full composed trait of all operations that must be supported
/// by a position
///
/// @trait
pub trait Position: Default + PositionView + PositionAttacks + PositionMoves + Display {}

impl<T> Position for T where T: Default + PositionView + PositionAttacks + PositionMoves + Display {}

/// `PositionView` is a trait that defines all state-related readonly queries on a
/// given position
///
/// @trait
pub trait PositionView {
    /// total_occupancy gets the occupancy bitboard of all pieces on the board
    ///
    /// @return: full occupancy bitboard of both sides
    fn total_occupancy(&self) -> Bitboard;

    /// empty_squares gets the bitboard of all empty squares on the board
    ///
    /// note: this is logically equivalent to `!(self.total_occupancy())`
    ///
    /// @return: bitboard of all empty squares on the board
    fn empty_squares(&self) -> Bitboard;

    /// occupancy gets an occupancy bitboard representing all of SideT's pieces
    /// on the board
    ///
    /// @marker: SideT - side to get the occupancy for
    /// @return: occupancy bitboard of SideT
    fn occupancy<SideT: Side>(&self) -> Bitboard;

    /// piece_at gets the piece type that is currently occupying the given square
    ///
    /// @param: square - square to get the piece type at
    /// @return: piece type at the given square
    fn piece_at(&self, square: Square) -> Pieces;

    /// king_square gets the square that the king of SideT is currently occupying
    ///
    /// @marker: SideT - side to get the king square for
    /// @return: square of SideT's king
    fn king_square<SideT: Side>(&self) -> Square;

    /// get_piece gets a bitboard representing all the squares that are occupied
    /// by all of SideT's `piece` pieces
    ///
    /// @marker: SideT - side to get the piece for
    /// @param: piece - piece to get the bitboard for
    /// @return: occupancy bitboard of SideT's `piece`
    fn get_piece<SideT: Side>(&self, piece: Pieces) -> Bitboard;

    /// turn gets the side to move from the current position
    ///
    /// @return: side to move
    fn turn(&self) -> Sides;

    /// en_passant gets the current en passant square, if it exists
    ///
    /// @return: current en passant square, or None if there is no valid en
    ///          passant square
    fn en_passant(&self) -> Option<Square>;

    /// castling gets the representation of the current castling rights in the
    /// position
    ///
    /// @return: current castling rights
    fn castling(&self) -> Castling;

    /// key gets the unique key identifier for the current position
    ///
    /// @return: unique identifier of the position
    fn key(&self) -> ZobristKey;
}

/// `PositionAttacks` is a trait that defines all the attack-related queries that
/// can be made to a position
///
/// @trait
pub trait PositionAttacks {
    /// is_attacked_by returns a bitboard of all the squares that are attacked by
    /// SideT::Other's pieces
    ///
    /// @marker: SideT - side to check if is attacked by
    /// @param: square - square to check if is attacked by
    /// @param: occupancy - occupancy to check for attacks based on
    /// @return: bitboard of all the squares that are attacked by SideT::Other's pieces
    fn is_attacked_by<SideT: Side>(&self, square: Square, occupancy: Bitboard) -> Bitboard;

    /// is_attacked returns true if the given square on SideT is attacked by
    /// SideT::Other
    ///
    /// @marker: SideT - side to check if is attacked by
    /// @param: square - square to check if is attacked by
    /// @param: occupancy - occupancy to check for attacks based on
    /// @return: true if the given square is attacked by SideT::Other, false otherwise
    fn is_attacked<SideT: Side>(&self, square: Square, occupancy: Bitboard) -> bool;

    /// checkers returns the bitboard of pieces that are checking the side-to-
    /// move's king on the board
    ///
    /// note: this function does not take the SideT parameter to semantically
    ///       convey the implicitly assumed side-to-move based on the current
    ///       position
    ///
    /// @return: bitboard of pieces that are checking the side-to-move's king
    fn checkers(&self) -> Bitboard;

    /// king_blocker_pieces gets the occupancy bitboard of all pieces that are
    /// blocking SideT's king from being in check
    ///
    /// note: a blocker piece is not necessarily on the same side as the king it
    ///       is blocking, but it must be the only piece in between the sniper
    ///       and the king
    ///
    /// @marker: SideT - side to get the king blocker pieces for
    /// @return: bitboard of SideT's king's blocker pieces
    fn king_blocker_pieces<SideT: Side>(&self) -> Bitboard;

    /// pinning_pieces gets the occupancy bitboard of SideT's pieces that are
    /// attacking SideT::Other's king-blocker pieces
    ///
    /// @marker: SideT - side to get the pinning pieces for
    /// @return: bitboard of pieces that are pinning SideT::Other's pieces
    fn pinning_pieces<SideT: Side>(&self) -> Bitboard;

    /// check_squares returns the bitboard of squares that a given piece on SideT
    /// would have to be on to deliver check to SideT::Other's king
    ///
    /// @marker: SideT - side to check the squares for
    /// @param: piece - piece to check the squares for
    /// @return: bitboard of squares that deliver check to SideT::Other
    fn check_squares<SideT: Side>(&self, piece: Pieces) -> Bitboard;
}

/// `PositionMoves` is a trait that defines all the move-making operations that can
/// be made in a position
///
/// @trait
pub trait PositionMoves {
    /// play plays a legal move and returns its deterministic piece delta
    ///
    /// @param: mv - move to make
    /// @return: deterministic piece delta, or an error if the move is illegal
    /// @side-effects: modifies the position and internal state on success
    fn play(&mut self, mv: Move) -> Result<MoveDelta, PlayError>;

    /// play_unchecked plays a move without checking its legality
    ///
    /// @param: mv - move to make
    /// @return: deterministic piece delta
    /// @side-effects: modifies the position and internal state
    /// @requires: the given move must be legal for the current position
    fn play_unchecked(&mut self, mv: Move) -> MoveDelta;

    /// undo undoes the given move from the current position
    ///
    /// @param: mv - move to undo
    /// @return: void
    /// @side-effects: modifies the position and internal state
    /// @requires: the move must have been made last turn
    fn undo(&mut self, mv: Move);

    /// is_legal_move checks if the given move is legal from the current position
    /// when played by SideT
    ///
    /// note: this method does not check that the king is still in check after
    ///       the move is made, instead, we delegate this logic to the move
    ///       generator. as a result, this method only checks that a king not
    ///       currently in check would be left in check after the move is made
    ///
    /// @marker: SideT - side to check if the move is legal for
    /// @param: mv - move to check if is legal
    /// @return: true if the move is legal, false otherwise
    fn is_legal_move<SideT: Side>(&self, mv: Move) -> bool;

    /// delivers_check checks if the given move delivers a check to SideT::Other
    /// when played by SideT
    ///
    /// @marker: SideT - side to check if the move delivers a check for
    /// @param: mv - move to check if delivers a check
    /// @return: true if the move delivers a check, false otherwise
    fn delivers_check<SideT: Side>(&self, mv: Move) -> bool;
}

// `CastlingSquares` is a per-side table of castling squares
chess_kit_primitives::define_sides! {
    CastlingSquares: Square {
        KING as king => (Square::E1, Square::E8),
        KINGSIDE_ROOK as kingside_rook => (Square::H1, Square::H8),
        QUEENSIDE_ROOK as queenside_rook => (Square::A1, Square::A8),
        KINGSIDE_DESTINATION as kingside_destination => (Square::G1, Square::G8),
        KINGSIDE_ROOK_DESTINATION as kingside_rook_destination => (Square::F1, Square::F8),
        QUEENSIDE_DESTINATION as queenside_destination => (Square::C1, Square::C8),
        QUEENSIDE_ROOK_DESTINATION as queenside_rook_destination => (Square::D1, Square::D8),
        QUEENSIDE_ROOK_INTERMEDIATE as queenside_rook_intermediate => (Square::B1, Square::B8),
    }
}
