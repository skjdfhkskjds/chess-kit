use chess_kit_primitives::{Pieces, Sides, Square};

/// `Board` is a read-only value snapshot of the current engine position
///
/// Adapters can render this value without requiring the engine to know about a
/// presentation format
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    squares: [Option<(Sides, Pieces)>; Square::TOTAL],
    side_to_move: Sides,
}

impl Board {
    /// empty creates an empty board snapshot for the given side to move
    ///
    /// @param: side_to_move - side whose turn it is
    /// @return: empty board snapshot
    pub const fn empty(side_to_move: Sides) -> Self {
        Self {
            squares: [None; Square::TOTAL],
            side_to_move,
        }
    }

    /// piece_at returns the side and piece occupying a square
    ///
    /// @param: square - square to inspect
    /// @return: occupying side and piece, or None when the square is empty
    pub const fn piece_at(&self, square: Square) -> Option<(Sides, Pieces)> {
        self.squares[square.idx()]
    }

    /// side_to_move returns the side whose turn it is
    ///
    /// @return: side to move
    pub const fn side_to_move(&self) -> Sides {
        self.side_to_move
    }

    /// set_piece updates one square in a board snapshot
    ///
    /// This supports custom [`crate::Engine`] implementations while keeping
    /// the snapshot's storage representation private
    ///
    /// @param: square - square to update
    /// @param: piece - new occupying side and piece, or None to empty the square
    /// @return: void
    /// @side-effects: modifies the board snapshot
    pub fn set_piece(&mut self, square: Square, piece: Option<(Sides, Pieces)>) {
        self.squares[square.idx()] = piece;
    }

    /// with_piece adds a piece and returns the updated snapshot
    ///
    /// @param: square - square occupied by the piece
    /// @param: side - side that owns the piece
    /// @param: piece - piece occupying the square
    /// @return: updated board snapshot
    pub fn with_piece(mut self, square: Square, side: Sides, piece: Pieces) -> Self {
        self.set_piece(square, Some((side, piece)));
        self
    }
}
