pub type Side = usize;

pub struct Sides;

impl Sides {
    pub const WHITE: Side = 0;
    pub const BLACK: Side = 1;
    pub const TOTAL: Side = 2;
}


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


pub type Square = usize;

pub struct Squares;

impl Squares {
    pub const TOTAL: Square = 64;
}
