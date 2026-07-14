use crate::position::DefaultPosition;
use crate::{DrawState, PositionState, PositionView};
use chess_kit_attack_table::AttackTable;
use chess_kit_primitives::{Bitboard, Castling, Pieces, Side, Sides, Square, ZobristKey};

impl<AT> DefaultPosition<AT>
where
    AT: AttackTable,
{
    /// state returns a reference to the current state
    ///
    /// @return: reference to the current state
    #[inline]
    pub(crate) fn state(&self) -> &PositionState {
        self.history.top()
    }

    /// state_mut returns a mutable reference to the current state
    ///
    /// @return: mutable reference to the current state
    #[inline]
    pub(crate) fn state_mut(&mut self) -> &mut PositionState {
        self.history.top_mut()
    }

    /// draw_state returns the incrementally maintained draw information
    ///
    /// @return: draw information for the current position
    #[inline]
    pub fn draw_state(&self) -> DrawState {
        self.state().draw_state()
    }
}

impl<AT> PositionView for DefaultPosition<AT>
where
    AT: AttackTable,
{
    /// total_occupancy gets the full occupancy bitboard of both sides
    ///
    /// @impl: PositionView::total_occupancy
    #[inline]
    fn total_occupancy(&self) -> Bitboard {
        self.sides[Sides::TOTAL]
    }

    /// empty_squares gets the bitboard of all empty squares
    ///
    /// @impl: PositionView::empty_squares
    #[inline]
    fn empty_squares(&self) -> Bitboard {
        !self.total_occupancy()
    }

    /// occupancy gets the occupancy bitboard of SideT
    ///
    /// @impl: PositionView::occupancy
    #[inline]
    fn occupancy<SideT: Side>(&self) -> Bitboard {
        self.sides[SideT::SIDE]
    }

    /// piece_at gets the piece type at the given square
    ///
    /// @impl: PositionView::piece_at
    #[inline]
    fn piece_at(&self, square: Square) -> Pieces {
        self.pieces[square]
    }

    /// turn gets the side to move
    ///
    /// @impl: PositionView::turn
    #[inline]
    fn turn(&self) -> Sides {
        self.state().turn()
    }

    /// king_square gets the square that the king of SideT is currently occupying
    ///
    /// @impl: PositionView::king_square
    #[inline]
    fn king_square<SideT: Side>(&self) -> Square {
        debug_assert!(
            self.get_piece::<SideT>(Pieces::King).exactly_one(),
            "invalid number of kings found for side {}, position: {}",
            SideT::SIDE,
            self
        );
        self.get_piece::<SideT>(Pieces::King).first_unchecked()
    }

    /// get_piece gets a bitboard representing all the squares that are occupied
    /// by all of SideT's `piece` pieces
    ///
    /// @impl: PositionView::get_piece
    #[inline]
    fn get_piece<SideT: Side>(&self, piece: Pieces) -> Bitboard {
        self.bitboards[SideT::SIDE][piece]
    }

    /// en_passant gets the current en passant square, if it exists
    ///
    /// @impl: PositionView::en_passant
    #[inline]
    fn en_passant(&self) -> Option<Square> {
        self.state().en_passant()
    }

    /// castling gets the representation of the current castling rights in the
    /// position
    ///
    /// @impl: PositionView::castling
    #[inline]
    fn castling(&self) -> Castling {
        self.state().castling()
    }

    /// key gets the unique key identifier for the current position
    ///
    /// @impl: PositionView::key
    #[inline]
    fn key(&self) -> ZobristKey {
        self.state().key()
    }
}
