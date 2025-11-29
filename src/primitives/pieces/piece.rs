use chess_kit_derive::IndexableEnum;

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, IndexableEnum)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    None,
}

impl Piece {
    pub const TOTAL: usize = 7;
}
