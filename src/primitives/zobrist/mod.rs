mod table;

use chess_kit_derive::BitOps;

use crate::primitives::{CastleRights, Pieces, Sides, Square};

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, BitOps)]
pub struct ZobristKey(u64);

type PieceRandoms = [[[ZobristKey; Square::TOTAL]; Pieces::TOTAL]; Sides::TOTAL];
type CastlingRandoms = [ZobristKey; CastleRights::TOTAL];
type SideRandoms = [ZobristKey; Sides::TOTAL];
type EnPassantRandoms = [ZobristKey; Square::TOTAL + 1];

// ZobristTable is a collection of random values used to generate/apply a zobrist
// key transformations for a given board position.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZobristTable {
    pieces: PieceRandoms,      // values for each piece on each square for each side
    castling: CastlingRandoms, // values for each castling right
    sides: SideRandoms,        // values for each side
    en_passant: EnPassantRandoms, // values for each en passant square
}
