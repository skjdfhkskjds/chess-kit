pub mod position;
pub mod castling;
pub mod display;
pub mod fen;
pub mod moves;
pub mod pieces;
pub mod rules;
pub mod sides;
mod attacks;

pub use position::Position;
pub use fen::{FENError, FENParser, Parser};

use crate::primitives::{Black, SideCastling, Square, White};

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
