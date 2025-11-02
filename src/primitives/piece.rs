pub type Piece = usize;

pub struct Pieces;

impl Pieces {
    pub const PAWN: Piece = 0;
    pub const KNIGHT: Piece = 1;
    pub const BISHOP: Piece = 2;
    pub const ROOK: Piece = 3;
    pub const QUEEN: Piece = 4;
    pub const KING: Piece = 5;
    pub const TOTAL: Piece = 6;
    pub const NONE: Piece = 7;
}
