use crate::setup::Setup;
use crate::{History, PositionState, PositionView};
use chess_kit_attack_table::AttackTable;
use chess_kit_primitives::{Bitboard, Pieces, Side, Sides, Square, ZobristTable, call_as};
use std::marker::PhantomData;

/// DefaultPosition is the default position implementation
///
/// DefaultPosition owns the board representation and private history while using `AT` to
/// derive attacks and tactical state
///
/// @type
pub struct DefaultPosition<AT: AttackTable> {
    pub(crate) history: History,
    pub(crate) sides: [Bitboard; Sides::TOTAL + 1],
    pub(crate) bitboards: [[Bitboard; Pieces::TOTAL]; Sides::TOTAL],
    pub(crate) pieces: [Pieces; Square::TOTAL],
    _attack_table: PhantomData<AT>,
}

impl<AT: AttackTable> DefaultPosition<AT> {
    /// empty creates a position with an empty board and initialized history
    ///
    /// @return: empty position
    fn empty() -> Self {
        let mut history = History::default();
        history.push(PositionState::default());
        Self {
            history,
            sides: [Bitboard::empty(); Sides::TOTAL + 1],
            bitboards: [[Bitboard::empty(); Pieces::TOTAL]; Sides::TOTAL],
            pieces: [Pieces::None; Square::TOTAL],
            _attack_table: PhantomData,
        }
    }

    /// from_setup creates a position from a validated setup
    ///
    /// @param: setup - validated setup to load
    /// @return: initialized position
    fn from_setup(setup: Setup) -> Self {
        let mut position = Self::empty();

        for (square, piece) in Square::ALL.into_iter().zip(setup.pieces()) {
            if let Some((side, piece)) = piece {
                position.bitboards[*side][*piece] |= Bitboard::square(square);
            }
        }

        {
            let state = position.state_mut();
            state.set_turn(setup.side_to_move());
            state.set_castling(setup.castling());
            state.set_en_passant(setup.en_passant());
            state.set_halfmoves(setup.halfmoves());
            state.set_fullmoves(setup.fullmoves());
        }

        position.initialize();
        position
    }

    /// initialize derives the position's board and incremental caches
    ///
    /// @return: void
    /// @side-effects: initializes the position caches
    fn initialize(&mut self) {
        self.initialize_sides();
        self.initialize_pieces();
        call_as!(self.turn(), |SideT| self.initialize_state::<SideT>());
    }

    /// initialize_sides derives the per-side and total occupancy bitboards
    ///
    /// @return: void
    /// @side-effects: initializes the position occupancy
    fn initialize_sides(&mut self) {
        self.sides = [Bitboard::empty(); Sides::TOTAL + 1];
        for piece in Pieces::ALL {
            self.sides[Sides::White] |= self.bitboards[Sides::White][piece];
            self.sides[Sides::Black] |= self.bitboards[Sides::Black][piece];
        }
        self.sides[Sides::TOTAL] = self.sides[Sides::White] | self.sides[Sides::Black];
    }

    /// initialize_pieces derives the piece type occupying each square
    ///
    /// @return: void
    /// @side-effects: initializes the position piece map
    fn initialize_pieces(&mut self) {
        self.pieces = [Pieces::None; Square::TOTAL];
        for square in Square::ALL {
            let mask = Bitboard::square(square);
            for side in [Sides::White, Sides::Black] {
                for piece in Pieces::ALL {
                    if self.bitboards[side][piece].intersects(mask) {
                        self.pieces[square] = piece;
                    }
                }
            }
        }
    }

    /// initialize_state derives incremental state for SideT to move
    ///
    /// @marker: SideT - side to move in the initialized position
    /// @return: void
    /// @side-effects: initializes the position state
    fn initialize_state<SideT: Side>(&mut self) {
        if let Some(en_passant_square) = self.state().en_passant() {
            let attacking_pawns = self.get_piece::<SideT>(Pieces::Pawn)
                & AT::pawn_targets::<SideT::Other>(en_passant_square);
            if attacking_pawns.is_empty() {
                self.state_mut().set_en_passant(None);
            }
        }

        let key = ZobristTable::new_key::<SideT>(
            self.state().castling(),
            self.state().en_passant(),
            self.bitboards,
        );
        self.state_mut().set_key(key);
        self.update_material_draw_state();

        let checkers = self.is_checked_by::<SideT>();
        self.state_mut().set_checkers(checkers);
        self.update_check_info::<SideT>();
    }
}

impl<AT: AttackTable> Default for DefaultPosition<AT> {
    /// default creates the standard starting position
    ///
    /// @return: standard starting position
    fn default() -> Self {
        Setup::default().into()
    }
}

impl<AT: AttackTable> From<Setup> for DefaultPosition<AT> {
    /// from creates a position from a validated setup
    ///
    /// @param: setup - validated setup to load
    /// @return: initialized position
    fn from(setup: Setup) -> Self {
        Self::from_setup(setup)
    }
}
