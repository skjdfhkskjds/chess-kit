use crate::primitives::zobrist::{
    CASTLING_RANDOMS, EN_PASSANT_RANDOMS, PIECE_RANDOMS, SIDE_RANDOMS,
};
use crate::primitives::{
    Bitboard, Black, Castling, Pieces, Side, Sides, Square, White, ZobristKey, ZobristTable,
};

impl ZobristTable {
    // new creates a new, uninitialized zobrist table
    //
    // @return: new instance of a zobrist table
    pub fn new() -> Self {
        Self {}
    }

    // // init initializes the zobrist table with random values using the rng
    // //
    // // @param: rng - mutable reference to the random number generator
    // pub fn init(&mut self, rng: &mut StdRng) {
    //     // generate random values for each piece on each square for each side
    //     self.pieces.iter_mut().for_each(|piece| {
    //         piece.iter_mut().for_each(|square| {
    //             square.iter_mut().for_each(|side| {
    //                 *side = ZobristKey::from(rng.random::<u64>());
    //             });
    //         });
    //     });

    //     // generate random values for each castling right
    //     self.castling.iter_mut().for_each(|castling| {
    //         *castling = ZobristKey::from(rng.random::<u64>());
    //     });

    //     // generate random values for each side
    //     self.sides.iter_mut().for_each(|side| {
    //         *side = ZobristKey::from(rng.random::<u64>());
    //     });

    //     // generate random values for each en passant square
    //     self.en_passant.iter_mut().for_each(|en_passant| {
    //         *en_passant = ZobristKey::from(rng.random::<u64>());
    //     });
    // }

    // new_key generates a new zobrist key for the given position
    //
    // @param: side - side to move
    // @param: castling - castling rights
    // @param: en_passant - en passant square
    // @param: bitboards - bitboards to generate the zobrist key for
    // @return: zobrist key for the given position
    pub fn new_key(
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
                        Sides::White => {
                            key ^= ZobristTable::piece::<White>(Pieces::from_idx(piece), square)
                        }
                        Sides::Black => {
                            key ^= ZobristTable::piece::<Black>(Pieces::from_idx(piece), square)
                        }
                    }
                }
            }
        }
        key ^= ZobristTable::castling(castling);
        match side {
            Sides::White => key ^= ZobristTable::side::<White>(),
            Sides::Black => key ^= ZobristTable::side::<Black>(),
        }
        key ^= ZobristTable::en_passant(en_passant);
        key
    }

    // piece returns the random value for the given side, piece, and square
    //
    // @param: piece - piece to get the random value for
    // @param: square - square to get the random value for
    // @return: random value for the given side, piece, and square
    #[inline(always)]
    pub fn piece<S: Side>(piece: Pieces, square: Square) -> ZobristKey {
        PIECE_RANDOMS[S::INDEX][piece.idx()][square.idx()]
    }

    // castling returns the random value for the given castling rights
    //
    // @param: castling - castling rights to get the random value for
    // @return: random value for the given castling rights
    #[inline(always)]
    pub fn castling(castling: Castling) -> ZobristKey {
        CASTLING_RANDOMS[u8::from(castling) as usize]
    }

    // side returns the random value for the given side
    //
    // @param: side - side to get the random value for
    // @return: random value for the given side
    #[inline(always)]
    pub fn side<S: Side>() -> ZobristKey {
        SIDE_RANDOMS[S::INDEX]
    }

    // en_passant returns the random value for the given en passant square
    // or the random value associated with an absence of en passant
    //
    // @param: en_passant - en passant square to get the random value for
    // @return: random value for the given en passant square
    #[inline(always)]
    pub fn en_passant(en_passant: Option<Square>) -> ZobristKey {
        match en_passant {
            Some(square) => EN_PASSANT_RANDOMS[square.file().idx()],
            None => ZobristKey::new(0),
        }
    }
}

impl Default for ZobristTable {
    fn default() -> Self {
        Self::new()
    }
}
