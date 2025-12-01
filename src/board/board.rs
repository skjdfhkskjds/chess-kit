use crate::board::fen::{FENError, FENParser, Parser};
use crate::primitives::{Bitboard, History, Pieces, Side, Sides, Square, State, ZobristTable};
use rand::prelude::*;
use rand::rngs::StdRng;

pub struct Board {
    pub state: State,     // current state of the board
    pub history: History, // history of the board state

    pub sides: [Bitboard; Sides::TOTAL], // occupancy bitboard per side
    pub bitboards: [[Bitboard; Pieces::TOTAL]; Sides::TOTAL], // bitboard per piece per side
    pub pieces: [Pieces; Square::TOTAL],  // piece type on each square

    pub zobrist: ZobristTable, // zobrist random values for the board
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
            sides: [Bitboard::empty(); Sides::TOTAL],
            bitboards: [[Bitboard::empty(); Pieces::TOTAL]; Sides::TOTAL],
            pieces: [Pieces::None; Square::TOTAL],
            zobrist: ZobristTable::new(),
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
        self.state.zobrist_key = self.zobrist.new_key(
            self.state.turn,
            self.state.castling,
            self.state.en_passant,
            self.bitboards,
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
    // @param: self - mutable reference to the board
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

    // reset resets the board to a new initial state
    //
    // @side-effects: modifies the `board`
    pub fn reset(&mut self) {
        self.state.reset();
        self.history.clear();
        self.sides = [Bitboard::empty(); Sides::TOTAL];
        self.bitboards = [[Bitboard::empty(); Pieces::TOTAL]; Sides::TOTAL];
        self.pieces = [Pieces::None; Square::TOTAL];
    }

    // occupancy gets the bitboard of all pieces on the board
    // 
    // Note: this value is not actually dependent on the side parameter, but
    //       it is included to provide compile-time resolution of the indices
    //
    // @param: self - immutable reference to the board
    // @return: bitboard of all pieces on the board
    #[inline(always)]
    pub fn occupancy<S: Side>(&self) -> Bitboard {
        self.sides[S::INDEX] | self.sides[S::Other::INDEX]
    }

    // empty_squares gets the bitboard of all empty squares on the board
    //
    // Note: logically equivalent to `!(self.occupancy())`
    //
    // @param: self - immutable reference to the board
    // @return: bitboard of all empty squares on the board
    #[inline(always)]
    pub fn empty_squares<S: Side>(&self) -> Bitboard {
        !self.occupancy::<S>()
    }

    // turn gets the side to move
    //
    // @param: self - immutable reference to the board
    // @return: side to move
    #[inline(always)]
    pub fn turn(&self) -> Sides {
        self.state.turn
    }
}

impl TryFrom<&str> for Board {
    type Error = FENError;

    // try_from creates a new board from the given FEN string
    //
    // @param: fen - FEN string to create the board from
    // @return: new board, or an error if the FEN string is invalid
    fn try_from(fen: &str) -> Result<Self, Self::Error> {
        let fen_parser = FENParser::parse(fen);
        if fen_parser.is_err() {
            return Err(fen_parser.err().unwrap());
        }

        let mut board = Self::new();
        let parsed = fen_parser.unwrap();
        board.bitboards = parsed.pieces.bitboards;
        board.state.turn = parsed.turn.turn;
        board.state.castling = parsed.castling.castling;
        board.state.en_passant = parsed.en_passant.square;
        board.state.halfmoves = parsed.halfmove_count.halfmove_count;
        board.state.fullmoves = parsed.fullmove_count.fullmove_count;

        // TODO: move the board initialization elsewhere
        board.init(None);
        Ok(board)
    }
}
