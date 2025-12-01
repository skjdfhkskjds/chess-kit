mod table;

use crate::primitives::{CastleRights, Piece, Sides, Square};

type PieceRandoms = [[[u64; Square::TOTAL]; Piece::TOTAL]; Sides::TOTAL];
type CastlingRandoms = [u64; CastleRights::TOTAL];
type SideRandoms = [u64; Sides::TOTAL];
type EnPassantRandoms = [u64; Square::TOTAL + 1];

pub type ZobristKey = u64;

// Zobrist is a collection of random values used to generate/apply a zobrist key
// for a given board position.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ZobristTable {
    pieces: PieceRandoms,      // values for each piece on each square for each side
    castling: CastlingRandoms, // values for each castling right
    sides: SideRandoms,        // values for each side
    en_passant: EnPassantRandoms, // values for each en passant square
}
