use crate::primitives::{
    Bitboard, Black, CastleRights, Castling, Piece, Side, Sides, Square, White,
};
use rand::prelude::*;
use rand::rngs::StdRng;

type PieceRandoms = [[[u64; Square::TOTAL]; Piece::TOTAL]; Sides::TOTAL];
type CastlingRandoms = [u64; CastleRights::TOTAL];
type SideRandoms = [u64; Sides::TOTAL];
type EnPassantRandoms = [u64; Square::TOTAL + 1];

pub type ZobristKey = u64;

// Zobrist is a collection of random values used to generate/apply a zobrist key
// for a given board position.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Zobrist {
    piece_randoms: PieceRandoms, // values for each piece on each square for each side
    castling_randoms: CastlingRandoms, // values for each castling right
    side_randoms: SideRandoms,   // values for each side
    en_passant_randoms: EnPassantRandoms, // values for each en passant square
}

impl Zobrist {
    // new creates a new Zobrist instance with all random values set to 0
    //
    // @return: new Zobrist instance
    pub fn new() -> Self {
        Self {
            piece_randoms: [[[0; Square::TOTAL]; Piece::TOTAL]; Sides::TOTAL],
            castling_randoms: [0; CastleRights::TOTAL],
            side_randoms: [0; Sides::TOTAL],
            en_passant_randoms: [0; Square::TOTAL + 1],
        }
    }

    // init initializes the Zobrist instance with random values using the rng
    //
    // @param: rng - mutable reference to the random number generator
    pub fn init(&mut self, rng: &mut StdRng) {
        // generate random values for each piece on each square for each side
        self.piece_randoms.iter_mut().for_each(|piece| {
            piece.iter_mut().for_each(|square| {
                square.iter_mut().for_each(|side| {
                    *side = rng.random::<u64>();
                });
            });
        });

        // generate random values for each castling right
        self.castling_randoms.iter_mut().for_each(|castling| {
            *castling = rng.random::<u64>();
        });

        // generate random values for each side
        self.side_randoms.iter_mut().for_each(|side| {
            *side = rng.random::<u64>();
        });

        // generate random values for each en passant square
        self.en_passant_randoms.iter_mut().for_each(|en_passant| {
            *en_passant = rng.random::<u64>();
        });
    }

    // key generates a zobrist key for the given position
    //
    // @param: side - side to move
    // @param: castling - castling rights
    // @param: en_passant - en passant square
    // @param: bitboards - bitboards to generate the zobrist key for
    // @return: zobrist key for the given position
    pub fn key(
        &self,
        side: Sides,
        castling: Castling,
        en_passant: Option<Square>,
        bitboards: [[Bitboard; Piece::TOTAL]; Sides::TOTAL],
    ) -> ZobristKey {
        let mut key = 0;
        for (side, bitboards) in bitboards.iter().enumerate() {
            for (piece, bitboard) in bitboards.iter().enumerate() {
                for square in bitboard.iter() {
                    match Sides::from_idx(side) {
                        Sides::White => key ^= self.piece::<White>(Piece::from_idx(piece), square),
                        Sides::Black => key ^= self.piece::<Black>(Piece::from_idx(piece), square),
                    }
                }
            }
        }
        key ^= self.side(side);
        key ^= self.castling(castling);
        key ^= self.en_passant(en_passant);
        key
    }

    // piece returns the random value for the given side, piece, and square
    //
    // @param: piece - piece to get the random value for
    // @param: square - square to get the random value for
    // @return: random value for the given side, piece, and square
    pub fn piece<S: Side>(&self, piece: Piece, square: Square) -> ZobristKey {
        self.piece_randoms[S::INDEX][piece.idx()][square.idx()]
    }

    // castling returns the random value for the given castling rights
    //
    // @param: castling - castling rights to get the random value for
    // @return: random value for the given castling rights
    pub fn castling(&self, castling: Castling) -> ZobristKey {
        self.castling_randoms[castling.bits() as usize]
    }

    // side returns the random value for the given side
    //
    // @param: side - side to get the random value for
    // @return: random value for the given side
    pub fn side(&self, side: Sides) -> ZobristKey {
        self.side_randoms[side.idx()]
    }

    // en_passant returns the random value for the given en passant square
    // or the random value associated with an absence of en passant
    //
    // @param: en_passant - en passant square to get the random value for
    // @return: random value for the given en passant square
    pub fn en_passant(&self, en_passant: Option<Square>) -> ZobristKey {
        match en_passant {
            Some(square) => self.en_passant_randoms[square.idx()],
            None => self.en_passant_randoms[Square::TOTAL],
        }
    }
}

impl Default for Zobrist {
    fn default() -> Self {
        Self::new()
    }
}
