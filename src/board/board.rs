use crate::board::history::History;
use crate::board::state::State;
use crate::board::zobrist::Zobrist;
use crate::primitives::{Bitboard, Castling, Piece, Pieces, Side, Sides, Squares};
use rand::prelude::*;
use rand::rngs::StdRng;

pub struct Board {
    pub state: State,     // current state of the board
    pub history: History, // history of the board state

    sides: [Bitboard; Sides::TOTAL], // occupancy bitboard per side
    bitboards: [[Bitboard; Pieces::TOTAL]; Sides::TOTAL], // bitboard per piece per side
    pieces: [Piece; Squares::TOTAL], // piece type on each square

    zobrist: Zobrist, // zobrist random values for the board
}

impl Board {
    // new creates a new board with all bitboards and pieces initialized to 0
    // and the zobrist random values set to 0
    //
    // @return: new board
    pub fn new() -> Self {
        Self {
            state: State::new(),
            history: History::new(),
            sides: [Bitboard::new(0); Sides::TOTAL],
            bitboards: [[Bitboard::new(0); Pieces::TOTAL]; Sides::TOTAL],
            pieces: [Pieces::NONE; Squares::TOTAL],
            zobrist: Zobrist::new(),
        }
    }

    // init initializes the board with the given rng
    //
    // @param: rng - an optional, mutable reference to the rng, useful for seeding
    pub fn init(&mut self, rng: Option<&mut StdRng>) {
        match rng {
            Some(rng) => self.zobrist.init(rng),
            None => self.zobrist.init(&mut StdRng::from_rng(&mut rand::rng())),
        }

        self.init_sides();
        self.init_pieces();
        self.state.init(
            Sides::WHITE,
            Castling::all(),
            None,
            self.bitboards,
            self.zobrist,
        );
        self.history.init(self.state);
    }

    // init_sides initializes the `sides` bitboards by ORing the bitboards of
    // each side
    //
    // @param: self - mutable reference to the board
    // @return: void
    // @requires: `bitboards` is initialized
    // @side-effects: modifies the `sides` bitboards
    fn init_sides(&mut self) {
        let white = self.bitboards[Sides::WHITE];
        let black = self.bitboards[Sides::BLACK];

        for (w, b) in white.iter().zip(black.iter()) {
            self.sides[Sides::WHITE] |= *w;
            self.sides[Sides::BLACK] |= *b;
        }
    }

    // init_pieces initializes the `pieces` array by iterating through the
    // bitboards of each side and setting the piece type on each square
    //
    // @param: self - mutable reference to the board
    // @return: void
    // @requires: `bitboards` is initialized
    // @side-effects: modifies the `pieces` array
    fn init_pieces(&mut self) {
        let white = self.bitboards[Sides::WHITE];
        let black = self.bitboards[Sides::BLACK];

        // set the piece type on each square
        for square in 0..Squares::TOTAL {
            let mut on_square: Piece = Pieces::NONE;

            let mask = 1u64 << square; // bitmask for the square
            for (piece, (w, b)) in white.iter().zip(black.iter()).enumerate() {
                if w.bits() & mask != 0 {
                    on_square = piece;
                    break; // enforce exclusivity
                }
                if b.bits() & mask != 0 {
                    on_square = piece;
                    break; // enforce exclusivity
                }
            }

            self.pieces[square] = on_square;
        }
    }

    // get_piece returns the bitboard of the given side and piece
    //
    // @param: self - immutable reference to the board
    // @param: side - side to get the piece for
    // @param: piece - piece to get the bitboard for
    // @return: bitboard of the piece for the given side
    pub fn get_piece(&self, side: Side, piece: Piece) -> Bitboard {
        self.bitboards[side][piece]
    }

    // empty_squares gets the bitboard of all empty squares on the board
    //
    // @param: self - immutable reference to the board
    // @return: bitboard of all empty squares on the board
    pub fn empty_squares(&self) -> Bitboard {
        !(self.sides[Sides::WHITE] | self.sides[Sides::BLACK])
    }
}
