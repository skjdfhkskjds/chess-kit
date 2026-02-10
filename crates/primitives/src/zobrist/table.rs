use crate::zobrist::{
    CASTLING_RANDOMS, EN_PASSANT_RANDOMS, PIECE_RANDOMS, SIDE_RANDOMS,
};
use crate::{
    Bitboard, Black, Castling, Pieces, Side, Sides, Square, White, ZobristKey, ZobristTable,
};

impl ZobristTable {
    // new_key generates a new zobrist key for the given position
    //
    // @marker: SideT - side to move
    // @param: castling - castling rights
    // @param: en_passant - en passant square
    // @param: bitboards - bitboards to generate the zobrist key for
    // @return: zobrist key for the given position
    pub fn new_key<SideT: Side>(
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
        key ^= ZobristTable::side::<SideT>();
        key ^= ZobristTable::en_passant(en_passant);
        key
    }

    // piece returns the random value for the given side, piece, and square
    //
    // @marker: SideT - side to get the random value for
    // @param: piece - piece to get the random value for
    // @param: square - square to get the random value for
    // @return: random value for the given side, piece, and square
    #[inline(always)]
    pub fn piece<SideT: Side>(piece: Pieces, square: Square) -> ZobristKey {
        PIECE_RANDOMS[SideT::INDEX][piece][square]
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
    // @marker: SideT - side to get the random value for
    // @return: random value for the given side
    #[inline(always)]
    pub fn side<SideT: Side>() -> ZobristKey {
        SIDE_RANDOMS[SideT::INDEX]
    }

    // en_passant returns the random value for the given en passant square
    // or the random value associated with an absence of en passant
    //
    // @param: en_passant - en passant square to get the random value for
    // @return: random value for the given en passant square
    #[inline(always)]
    pub fn en_passant(en_passant: Option<Square>) -> ZobristKey {
        match en_passant {
            Some(square) => EN_PASSANT_RANDOMS[square.file()],
            None => ZobristKey::new(0),
        }
    }
}
