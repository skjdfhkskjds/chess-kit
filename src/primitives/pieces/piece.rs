use chess_kit_derive::IndexableEnum;

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, IndexableEnum)]
pub enum Pieces {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Pieces {
    pub const TOTAL: usize = 7;
}
