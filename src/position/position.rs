use crate::attack_table::AttackTable;
use crate::position::fen::{FENError, FENParser, Parser};
use crate::primitives::{
    Bitboard, GameStateExt, History, Pieces, Side, Sides, Square, State, ZobristTable,
};
use rand::prelude::*;
use rand::rngs::StdRng;

pub struct Position<AT: AttackTable, StateT: State + GameStateExt> {
    pub attack_table: &'static AT, // attack table for the position
    pub state: StateT,             // current state of the position
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
            state: StateT::default(),
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
        match rng {
            Some(rng) => self.zobrist.init(rng),
            None => self.zobrist.init(&mut StdRng::from_rng(&mut rand::rng())),
        }

        self.init_sides();
        self.init_pieces();
        self.state.set_key(self.zobrist.new_key(
            self.state.turn(),
            self.state.castling(),
            self.state.en_passant(),
            self.bitboards,
        ));
        self.history.init(self.state);
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
                if !(w & mask).is_empty() {
                    on_square = Pieces::from_idx(piece);
                    break; // enforce exclusivity
                }
                if !(b & mask).is_empty() {
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
        self.state.reset();
        self.history.clear();
        self.sides = [Bitboard::empty(); Sides::TOTAL];
        self.bitboards = [[Bitboard::empty(); Pieces::TOTAL]; Sides::TOTAL];
        self.pieces = [Pieces::None; Square::TOTAL];
    }

    // occupancy gets the bitboard of the given side's pieces in the position
    //
    // @param: side - side to get the occupancy for
    // @return: bitboard of the given side's pieces in the position
    #[inline(always)]
    pub fn occupancy<SideT: Side>(&self) -> Bitboard {
        self.sides[SideT::INDEX]
    }

    // total_occupancy gets the bitboard of all pieces in the position
    //
    //
    // @return: bitboard of all pieces in the position
    #[inline(always)]
    pub fn total_occupancy(&self) -> Bitboard {
        self.sides[Sides::White.idx()] | self.sides[Sides::Black.idx()]
    }

    // empty_squares gets the bitboard of all empty squares in the position
    //
    // note: logically equivalent to `!(self.occupancy())`
    //
    // @return: bitboard of all empty squares in the position
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
        self.state.turn()
    }

    // state returns a reference to the current state of the position
    //
    // @return: reference to the current state of the position
    #[inline(always)]
    pub fn state(&self) -> &StateT {
        &self.state
    }
}

impl<AT, StateT> Position<AT, StateT>
where
    AT: AttackTable,
    StateT: State + GameStateExt,
{
    // try_from creates a new position from the given FEN string
    //
    // @param: fen - FEN string to create the position from
    // @return: new position, or an error if the FEN string is invalid
    pub fn load_fen(&mut self, fen: &str) -> Result<(), FENError> {
        let fen_parser = FENParser::parse(fen);
        if fen_parser.is_err() {
            return Err(fen_parser.err().unwrap());
        }

        let parsed = fen_parser.unwrap();
        self.bitboards = parsed.pieces.bitboards;
        self.state.set_turn(parsed.turn.turn);
        self.state.set_castling(parsed.castling.castling);
        self.state.set_en_passant(parsed.en_passant.square);
        self.state.set_halfmoves(parsed.halfmove_parser.clock);
        self.state.set_fullmoves(parsed.fullmove_parser.clock);

        // TODO: move the board initialization elsewhere
        self.init(None);
        Ok(())
    }
}
