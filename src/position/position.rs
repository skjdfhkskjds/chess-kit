use crate::attack_table::AttackTable;
use crate::position::fen::{FENError, FENParser, Parser};
use crate::primitives::{
    Bitboard, GameStateExt, History, Pieces, Side, Sides, Square, State, ZobristTable,
};
use rand::prelude::*;
use rand::rngs::StdRng;

pub struct Position<AT: AttackTable, StateT: State + GameStateExt> {
    pub attack_table: &'static AT, // reference to the static attack table
    pub history: History<StateT>,  // history of the position state

    pub sides: [Bitboard; Sides::TOTAL], // occupancy bitboard per side
    pub bitboards: [[Bitboard; Pieces::TOTAL]; Sides::TOTAL], // bitboard per piece per side
    pub pieces: [Pieces; Square::TOTAL], // piece type on each square

    pub zobrist: ZobristTable, // zobrist random values for the position
}

impl<AT, StateT> Position<AT, StateT>
where
    AT: AttackTable,
    StateT: State + GameStateExt,
{
    // new creates a new position with all bitboards and pieces initialized to 0
    // and the zobrist random values set to 0
    //
    // @return: new position
    pub fn new(attack_table: &'static AT) -> Self {
        Self {
            attack_table,
            history: History::default(),
            sides: [Bitboard::empty(); Sides::TOTAL],
            bitboards: [[Bitboard::empty(); Pieces::TOTAL]; Sides::TOTAL],
            pieces: [Pieces::None; Square::TOTAL],
            zobrist: ZobristTable::default(),
        }
    }

    // init initializes the position with the given rng
    //
    // @param: rng - an optional, mutable reference to the rng, useful for seeding
    pub fn init(&mut self, rng: Option<&mut StdRng>) {
        if self.history.is_empty() {
            self.history.init(StateT::default());
        }

        match rng {
            Some(rng) => self.zobrist.init(rng),
            None => self.zobrist.init(&mut StdRng::from_rng(&mut rand::rng())),
        }

        self.init_sides();
        self.init_pieces();
        let key = self.zobrist.new_key(
            self.state().turn(),
            self.state().castling(),
            self.state().en_passant(),
            self.bitboards,
        );
        self.state_mut().set_key(key);
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

    // reset resets the position to a new initial state
    //
    // @side-effects: modifies the `position`
    pub fn reset(&mut self) {
        self.history.clear();
        self.history.push(StateT::default());
        self.state_mut().reset();
        self.sides = [Bitboard::empty(); Sides::TOTAL];
        self.bitboards = [[Bitboard::empty(); Pieces::TOTAL]; Sides::TOTAL];
        self.pieces = [Pieces::None; Square::TOTAL];
    }

    // occupancy gets the occupancy bitboard of SideT
    //
    // @param: side - side to get the occupancy for
    // @return: occupancy bitboard of SideT
    #[inline(always)]
    pub fn occupancy<SideT: Side>(&self) -> Bitboard {
        self.sides[SideT::INDEX]
    }

    // total_occupancy gets the full occupancy bitboard of both sides
    //
    // @return: full occupancy bitboard of both sides
    #[inline(always)]
    pub fn total_occupancy(&self) -> Bitboard {
        self.sides[Sides::White.idx()] | self.sides[Sides::Black.idx()]
    }

    // empty_squares gets the bitboard of all empty squares
    //
    // note: logically equivalent to `!(self.occupancy())`
    //
    // @return: bitboard of all the empty squares
    #[inline(always)]
    pub fn empty_squares(&self) -> Bitboard {
        !self.total_occupancy()
    }

    // piece_at gets the piece type at the given square
    //
    // @param: square - square to get the piece type at
    // @return: piece type at the given square
    #[inline(always)]
    pub fn piece_at(&self, square: Square) -> Pieces {
        self.pieces[square.idx()]
    }

    // turn gets the side to move
    //
    // @return: side to move
    #[inline(always)]
    pub fn turn(&self) -> Sides {
        self.state().turn()
    }

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

impl<AT, StateT> Position<AT, StateT>
where
    AT: AttackTable,
    StateT: State + GameStateExt,
{
    // load_fen loads a new position from the given FEN string
    //
    // @param: fen - FEN string to create the position from
    // @return: initialized position, or an error if the FEN string is invalid
    pub fn load_fen(&mut self, fen: &str) -> Result<(), FENError> {
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
        self.init(None);
        Ok(())
    }
}
