use crate::attack_table::AttackTable;
use crate::position::{DefaultPosition, PositionState};
use crate::primitives::{
    Bitboard, Castling, GameStateExt, Pieces, Side, Sides, Square, State, ZobristKey,
};

impl<AT, StateT> DefaultPosition<AT, StateT>
where
    AT: AttackTable,
    StateT: State + GameStateExt,
{
    // state returns a reference to the current state
    //
    // @return: reference to the current state
    #[inline(always)]
    pub fn state(&self) -> &StateT {
        self.history.current()
    }

    // state_mut returns a mutable reference to the current state
    //
    // @return: mutable reference to the current state
    #[inline(always)]
    pub fn state_mut(&mut self) -> &mut StateT {
        self.history.current_mut()
    }
}

impl<AT, StateT> PositionState for DefaultPosition<AT, StateT>
where
    AT: AttackTable,
    StateT: State + GameStateExt,
{
    // total_occupancy gets the full occupancy bitboard of both sides
    //
    // @impl: PositionState::total_occupancy
    #[inline(always)]
    fn total_occupancy(&self) -> Bitboard {
        self.sides[Sides::TOTAL]
    }

    // empty_squares gets the bitboard of all empty squares
    //
    // @impl: PositionState::empty_squares
    #[inline(always)]
    fn empty_squares(&self) -> Bitboard {
        !self.total_occupancy()
    }

    // occupancy gets the occupancy bitboard of SideT
    //
    // @impl: PositionState::occupancy
    #[inline(always)]
    fn occupancy<SideT: Side>(&self) -> Bitboard {
        self.sides[SideT::INDEX]
    }

    // piece_at gets the piece type at the given square
    //
    // @impl: PositionState::piece_at
    #[inline(always)]
    fn piece_at(&self, square: Square) -> Pieces {
        self.pieces[square.idx()]
    }

    // turn gets the side to move
    //
    // @impl: PositionState::turn
    #[inline(always)]
    fn turn(&self) -> Sides {
        self.state().turn()
    }

    // king_square gets the square that the king of SideT is currently occupying
    //
    // @impl: PositionState::king_square
    #[inline(always)]
    fn king_square<SideT: Side>(&self) -> Square {
        debug_assert!(
            self.get_piece::<SideT>(Pieces::King).exactly_one(),
            "invalid number of kings found for side {}, position: {}",
            SideT::SIDE,
            self
        );
        self.get_piece::<SideT>(Pieces::King).must_first()
    }

    // get_piece gets a bitboard representing all the squares that are occupied
    // by all of SideT's `piece` pieces
    //
    // @impl: PositionState::get_piece
    #[inline(always)]
    fn get_piece<SideT: Side>(&self, piece: Pieces) -> Bitboard {
        self.bitboards[SideT::INDEX][piece.idx()]
    }

    // en_passant gets the current en passant square, if it exists
    //
    // @impl: PositionState::en_passant
    #[inline(always)]
    fn en_passant(&self) -> Option<Square> {
        self.state().en_passant()
    }

    // castling gets the representation of the current castling rights in the
    // position
    //
    // @impl: PositionState::castling
    #[inline(always)]
    fn castling(&self) -> Castling {
        self.state().castling()
    }

    // key gets the unique key identifier for the current position
    //
    // @impl: PositionState::key
    #[inline(always)]
    fn key(&self) -> ZobristKey {
        self.state().key()
    }
}
