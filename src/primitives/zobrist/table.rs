use crate::primitives::{
    Bitboard, Black, CastleRights, Castling, Pieces, Side, Sides, Square, White, ZobristKey,
    ZobristTable,
};
use rand::prelude::*;
use rand::rngs::StdRng;

impl ZobristTable {
    // new creates a new, uninitialized zobrist table
    //
    // @return: new instance of a zobrist table
    pub fn new() -> Self {
        Self {
            pieces: [[[ZobristKey::default(); Square::TOTAL]; Pieces::TOTAL]; Sides::TOTAL],
            castling: [ZobristKey::default(); CastleRights::TOTAL],
            sides: [ZobristKey::default(); Sides::TOTAL],
            en_passant: [ZobristKey::default(); Square::TOTAL + 1],
        }
    }

    // init initializes the zobrist table with random values using the rng
    //
    // @param: rng - mutable reference to the random number generator
    pub fn init(&mut self, rng: &mut StdRng) {
        // generate random values for each piece on each square for each side
        self.pieces.iter_mut().for_each(|piece| {
            piece.iter_mut().for_each(|square| {
                square.iter_mut().for_each(|side| {
                    *side = ZobristKey::from(rng.random::<u64>());
                });
            });
        });

        // generate random values for each castling right
        self.castling.iter_mut().for_each(|castling| {
            *castling = ZobristKey::from(rng.random::<u64>());
        });

        // generate random values for each side
        self.sides.iter_mut().for_each(|side| {
            *side = ZobristKey::from(rng.random::<u64>());
        });

        // generate random values for each en passant square
        self.en_passant.iter_mut().for_each(|en_passant| {
            *en_passant = ZobristKey::from(rng.random::<u64>());
        });
    }

    // new_key generates a new zobrist key for the given position
    //
    // @param: side - side to move
    // @param: castling - castling rights
    // @param: en_passant - en passant square
    // @param: bitboards - bitboards to generate the zobrist key for
    // @return: zobrist key for the given position
    pub fn new_key(
        &self,
        side: Sides,
        castling: Castling,
        en_passant: Option<Square>,
        bitboards: [[Bitboard; Pieces::TOTAL]; Sides::TOTAL],
    ) -> ZobristKey {
        let mut key = ZobristKey::default();
        for (side, bitboards) in bitboards.iter().enumerate() {
            for (piece, bitboard) in bitboards.iter().enumerate() {
                for square in bitboard.iter() {
                    match Sides::from_idx(side) {
                        Sides::White => key ^= self.piece::<White>(Pieces::from_idx(piece), square),
                        Sides::Black => key ^= self.piece::<Black>(Pieces::from_idx(piece), square),
                    }
                }
            }
        }
        key ^= self.castling(castling);
        match side {
            Sides::White => key ^= self.side::<White>(),
            Sides::Black => key ^= self.side::<Black>(),
        }
        key ^= self.en_passant(en_passant);
        key
    }

    // piece returns the random value for the given side, piece, and square
    //
    // @param: piece - piece to get the random value for
    // @param: square - square to get the random value for
    // @return: random value for the given side, piece, and square
    #[inline(always)]
    pub fn piece<S: Side>(&self, piece: Pieces, square: Square) -> ZobristKey {
        self.pieces[S::INDEX][piece.idx()][square.idx()]
    }

    // castling returns the random value for the given castling rights
    //
    // @param: castling - castling rights to get the random value for
    // @return: random value for the given castling rights
    #[inline(always)]
    pub fn castling(&self, castling: Castling) -> ZobristKey {
        self.castling[castling.bits() as usize]
    }

    // side returns the random value for the given side
    //
    // @param: side - side to get the random value for
    // @return: random value for the given side
    #[inline(always)]
    pub fn side<S: Side>(&self) -> ZobristKey {
        self.sides[S::INDEX]
    }

    // en_passant returns the random value for the given en passant square
    // or the random value associated with an absence of en passant
    //
    // @param: en_passant - en passant square to get the random value for
    // @return: random value for the given en passant square
    #[inline(always)]
    pub fn en_passant(&self, en_passant: Option<Square>) -> ZobristKey {
        match en_passant {
            Some(square) => self.en_passant[square.idx()],
            None => self.en_passant[Square::TOTAL],
        }
    }
}

impl Default for ZobristTable {
    fn default() -> Self {
        Self::new()
    }
}
