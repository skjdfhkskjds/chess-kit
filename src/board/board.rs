use crate::board::fen::{FENParser, Parser};
use crate::board::history::History;
use crate::board::state::State;
use crate::board::zobrist::Zobrist;
use crate::primitives::{Bitboard, Piece, Pieces, Side, Sides, Squares};
use rand::prelude::*;
use rand::rngs::StdRng;

pub struct Board {
    pub state: State,     // current state of the board
    pub history: History, // history of the board state

    pub sides: [Bitboard; Sides::TOTAL], // occupancy bitboard per side
    pub bitboards: [[Bitboard; Pieces::TOTAL]; Sides::TOTAL], // bitboard per piece per side
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
            sides: [Bitboard::empty(); Sides::TOTAL],
            bitboards: [[Bitboard::empty(); Pieces::TOTAL]; Sides::TOTAL],
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
        self.sides = [Bitboard::empty(); Sides::TOTAL];
        self.bitboards = [[Bitboard::empty(); Pieces::TOTAL]; Sides::TOTAL];
        self.pieces = [Pieces::NONE; Squares::TOTAL];
    }

    // occupancy gets the bitboard of all pieces on the board
    //
    // @param: self - immutable reference to the board
    // @return: bitboard of all pieces on the board
    #[inline(always)]
    pub fn occupancy(&self) -> Bitboard {
        self.sides[Sides::WHITE] | self.sides[Sides::BLACK]
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
        self.turn() ^ 1
    }
}

impl From<&str> for Board {
    // from creates a new board from the given FEN string
    //
    // @param: fen - FEN string to create the board from
    // @return: new board
    // @panic: if the FEN string is invalid
    fn from(fen: &str) -> Self {
        let fen_parser = FENParser::parse(fen);
        match fen_parser {
            Ok(fen_parser) => {
                let mut board = Self::new();
                board.bitboards = fen_parser.pieces.bitboards;
                board.state.turn = fen_parser.turn.turn;
                board.state.castling = fen_parser.castling.castling;
                board.state.en_passant = fen_parser.en_passant.square;
                board.state.halfmoves = fen_parser.halfmove_count.halfmove_count;
                board.state.fullmoves = fen_parser.fullmove_count.fullmove_count;
                board
            }
            Err(e) => {
                panic!("Failed to parse FEN: {}", e);
            }
        }
    }
}
