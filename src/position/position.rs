use crate::attack_table::AttackTable;
use crate::position::fen::{FENError, FENParser, Parser};
use crate::position::{Position, PositionFromFEN, PositionState};
use crate::primitives::{Bitboard, Black, Pieces, Side, Sides, Square, White, ZobristTable};
use crate::state::{GameStateExt, History, State};
use std::marker::PhantomData;

pub struct DefaultPosition<AT: AttackTable, StateT: State + GameStateExt> {
    pub history: History<StateT>,            // history of the position state
    pub sides: [Bitboard; Sides::TOTAL + 1], // occupancy bitboard per side
    pub bitboards: [[Bitboard; Pieces::TOTAL]; Sides::TOTAL], // bitboard per piece per side
    pub pieces: [Pieces; Square::TOTAL],     // piece type on each square

    _attack_table: PhantomData<AT>,
}

impl<AT, StateT> Position for DefaultPosition<AT, StateT>
where
    AT: AttackTable,
    StateT: State + GameStateExt,
{
    // new creates a new position with all bitboards and pieces initialized to 0
    // and the zobrist random values set to 0
    //
    // @impl: Position::new
    fn new() -> Self {
        Self {
            history: History::default(),
            sides: [Bitboard::empty(); Sides::TOTAL + 1],
            bitboards: [[Bitboard::empty(); Pieces::TOTAL]; Sides::TOTAL],
            pieces: [Pieces::None; Square::TOTAL],
            _attack_table: PhantomData,
        }
    }

    // reset resets the position to a new initial state
    //
    // @impl: Position::reset
    fn reset(&mut self) {
        self.history.clear();
        self.history.push(StateT::default());
        self.state_mut().reset();
        self.sides = [Bitboard::empty(); Sides::TOTAL + 1];
        self.bitboards = [[Bitboard::empty(); Pieces::TOTAL]; Sides::TOTAL];
        self.pieces = [Pieces::None; Square::TOTAL];
    }
}

impl<AT, StateT> DefaultPosition<AT, StateT>
where
    AT: AttackTable,
    StateT: State + GameStateExt,
{
    // init initializes the position
    fn init(&mut self) {
        self.init_sides();
        self.init_pieces();

        match self.turn() {
            Sides::White => self.init_state::<White>(),
            Sides::Black => self.init_state::<Black>(),
        }
    }

    // init_sides initializes the `sides` bitboards by ORing the bitboards of
    // each side
    //
    // @return: void
    // @side-effects: modifies the `sides` bitboards
    // @requires: `bitboards` is initialized
    fn init_sides(&mut self) {
        let white = self.bitboards[Sides::White.idx()];
        let black = self.bitboards[Sides::Black.idx()];

        for (w, b) in white.iter().zip(black.iter()) {
            self.sides[Sides::White.idx()] |= *w;
            self.sides[Sides::Black.idx()] |= *b;
        }

        self.sides[Sides::TOTAL] = self.occupancy::<White>() | self.occupancy::<Black>();
    }

    // init_pieces initializes the `pieces` array by iterating through the
    // bitboards of each side and setting the piece type on each square
    //
    // @return: void
    // @requires: `bitboards` is initialized
    // @side-effects: modifies the `pieces` array
    fn init_pieces(&mut self) {
        let white = self.bitboards[Sides::White.idx()];
        let black = self.bitboards[Sides::Black.idx()];

        // set the piece type on each square
        for square in 0..Square::TOTAL {
            let mut on_square: Pieces = Pieces::None;

            let mask = 1u64 << square; // bitmask for the square
            for (piece, (w, b)) in white.iter().zip(black.iter()).enumerate() {
                if (w & mask).not_empty() {
                    on_square = Pieces::from_idx(piece);
                    break; // enforce exclusivity
                }
                if (b & mask).not_empty() {
                    on_square = Pieces::from_idx(piece);
                    break; // enforce exclusivity
                }
            }

            self.pieces[square] = on_square;
        }
    }

    // init_state initializes the state for the given position
    //
    // @return: void
    // @side-effects: modifies the `state`
    fn init_state<SideT: Side>(&mut self) {
        // in our position definition, we define the state's en passant square
        // to be present if the previous move was a double pawn push AND the
        // opponent has a pawn next to the destination square of the double pawn
        // push
        //
        // TODO: move this hack elsewhere
        if let Some(en_passant_square) = self.state().en_passant() {
            let attacking_pawns = self.get_piece::<SideT>(Pieces::Pawn)
                & AT::pawn_targets::<SideT::Other>(en_passant_square);

            // if there are no pawns that can attack the en passant square, then
            // no en passant capture is possible
            if attacking_pawns.is_empty() {
                self.state_mut().set_en_passant(None);
            }
        }

        // set the state key
        let key = ZobristTable::new_key(
            self.state().turn(),
            self.state().castling(),
            self.state().en_passant(),
            self.bitboards,
        );
        self.state_mut().set_key(key);

        // update the check info and checkers in the state
        let checkers = self.is_checked_by::<SideT>();
        self.state_mut().set_checkers(checkers);
        self.update_check_info::<SideT>();
    }
}

impl<AT, StateT> PositionFromFEN for DefaultPosition<AT, StateT>
where
    AT: AttackTable,
    StateT: State + GameStateExt,
{
    // load_fen loads a new position from the given FEN string
    //
    // @impl: PositionFromFEN::load_fen
    fn load_fen(&mut self, fen: &str) -> Result<(), FENError> {
        let fen_parser = FENParser::parse(fen);
        if fen_parser.is_err() {
            return Err(fen_parser.err().unwrap());
        }

        let parsed = fen_parser.unwrap();
        self.history.clear();
        self.history.push(StateT::default());
        self.bitboards = parsed.pieces.bitboards;
        {
            let state = self.state_mut();
            state.set_turn(parsed.turn.turn);
            state.set_castling(parsed.castling.castling);
            state.set_en_passant(parsed.en_passant.square);
            state.set_halfmoves(parsed.halfmove_parser.clock);
            state.set_fullmoves(parsed.fullmove_parser.clock);
        }

        // TODO: move the board initialization elsewhere
        self.init();
        Ok(())
    }
}
