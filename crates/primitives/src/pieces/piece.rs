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

    // ALL is a constant array of all pieces except for Pieces::None
    pub const ALL: [Pieces; Self::TOTAL - 1] = [
        Self::Pawn,
        Self::Knight,
        Self::Bishop,
        Self::Rook,
        Self::Queen,
        Self::King,
    ];
}
