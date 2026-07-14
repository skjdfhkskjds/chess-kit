mod errors;
mod fen;

pub use errors::*;
pub use fen::*;

use chess_kit_primitives::{Castling, Clock, Pieces, Sides, Square};

type PieceOnSquare = Option<(Sides, Pieces)>;

/// Setup is the format-independent data needed to initialize a position
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Setup {
    pieces: [PieceOnSquare; Square::TOTAL],
    side_to_move: Sides,
    castling: Castling,
    en_passant: Option<Square>,
    halfmoves: Clock,
    fullmoves: Clock,
}

impl Setup {
    /// pieces returns the piece and side occupying each square
    ///
    /// @return: piece and side occupying each square, or `None` for an empty square
    pub fn pieces(&self) -> &[PieceOnSquare; Square::TOTAL] {
        &self.pieces
    }

    /// side_to_move returns the side whose turn it is
    ///
    /// @return: side to move
    pub const fn side_to_move(&self) -> Sides {
        self.side_to_move
    }

    /// castling returns the available castling rights
    ///
    /// @return: available castling rights
    pub const fn castling(&self) -> Castling {
        self.castling
    }

    /// en_passant returns the en passant target square, if any
    ///
    /// @return: en passant target square, if any
    pub const fn en_passant(&self) -> Option<Square> {
        self.en_passant
    }

    /// halfmoves returns the number of halfmoves since the last zeroing move
    ///
    /// @return: halfmove clock
    pub const fn halfmoves(&self) -> Clock {
        self.halfmoves
    }

    /// fullmoves returns the current fullmove number
    ///
    /// @return: fullmove number
    pub const fn fullmoves(&self) -> Clock {
        self.fullmoves
    }
}

impl Default for Setup {
    /// default returns the setup for the standard starting position
    ///
    /// @return: format-independent setup for the standard starting position
    fn default() -> Self {
        Fen::default().into()
    }
}
