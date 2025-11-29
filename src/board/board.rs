use crate::board::fen::{FENError, FENParser, Parser};
use crate::board::history::History;
use crate::board::state::State;
use crate::board::zobrist::Zobrist;
use crate::primitives::{Bitboard, Piece, Pieces, Side, Squares};
use rand::prelude::*;
use rand::rngs::StdRng;

pub struct Board {
    pub state: State,     // current state of the board
    pub history: History, // history of the board state

    pub sides: [Bitboard; Side::TOTAL], // occupancy bitboard per side
    pub bitboards: [[Bitboard; Pieces::TOTAL]; Side::TOTAL], // bitboard per piece per side
    pub pieces: [Piece; Squares::TOTAL], // piece type on each square

    pub zobrist: Zobrist, // zobrist random values for the board
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
            sides: [Bitboard::empty(); Side::TOTAL],
            bitboards: [[Bitboard::empty(); Pieces::TOTAL]; Side::TOTAL],
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
        self.state.zobrist_key = self.zobrist.key(
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
        let white = self.bitboards[Side::White.idx()];
        let black = self.bitboards[Side::Black.idx()];

        for (w, b) in white.iter().zip(black.iter()) {
            self.sides[Side::White.idx()] |= *w;
            self.sides[Side::Black.idx()] |= *b;
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
        let white = self.bitboards[Side::White.idx()];
        let black = self.bitboards[Side::Black.idx()];

        // set the piece type on each square
        for square in 0..Squares::TOTAL {
            let mut on_square: Piece = Pieces::NONE;

            let mask = 1u64 << square; // bitmask for the square
            for (piece, (w, b)) in white.iter().zip(black.iter()).enumerate() {
                if !(w & mask).is_empty() {
                    on_square = Piece::new(piece);
                    break; // enforce exclusivity
                }
                if !(b & mask).is_empty() {
                    on_square = Piece::new(piece);
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
        self.sides = [Bitboard::empty(); Side::TOTAL];
        self.bitboards = [[Bitboard::empty(); Pieces::TOTAL]; Side::TOTAL];
        self.pieces = [Pieces::NONE; Squares::TOTAL];
    }

    // occupancy gets the bitboard of all pieces on the board
    //
    // @param: self - immutable reference to the board
    // @return: bitboard of all pieces on the board
    #[inline(always)]
    pub fn occupancy(&self) -> Bitboard {
        self.sides[Side::White.idx()] | self.sides[Side::Black.idx()]
    }

    // empty_squares gets the bitboard of all empty squares on the board
    //
    // @note: logically equivalent to `!(self.occupancy())`
    //
    // @param: self - immutable reference to the board
    // @return: bitboard of all empty squares on the board
    #[inline(always)]
    pub fn empty_squares(&self) -> Bitboard {
        !self.occupancy()
    }

    // turn gets the side to move
    //
    // @param: self - immutable reference to the board
    // @return: side to move
    #[inline(always)]
    pub fn turn(&self) -> Side {
        self.state.turn
    }

    // opponent gets the opponent side
    //
    // @param: self - immutable reference to the board
    // @return: opponent side
    #[inline(always)]
    pub fn opponent(&self) -> Side {
        self.turn().other()
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
