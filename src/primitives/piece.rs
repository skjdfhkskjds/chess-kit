use std::fmt;

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Default, Hash)]
pub struct Piece(usize);

impl Piece {
    pub const fn new(piece: usize) -> Self {
        Self(piece)
    }

    pub const fn unwrap(&self) -> usize {
        self.0
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece = match self.0 {
            0 => "P",
            1 => "N",
            2 => "B",
            3 => "R",
            4 => "Q",
            5 => "K",
            6 => ".",
            _ => "?",
        };

        match piece {
            "?" => Err(fmt::Error),
            _ => write!(f, "{}", piece),
        }
    }
}

pub struct Pieces;

impl Pieces {
    pub const TOTAL: usize = 6;

    pub const PAWN: Piece = Piece::new(0);
    pub const KNIGHT: Piece = Piece::new(1);
    pub const BISHOP: Piece = Piece::new(2);
    pub const ROOK: Piece = Piece::new(3);
    pub const QUEEN: Piece = Piece::new(4);
    pub const KING: Piece = Piece::new(5);
    pub const NONE: Piece = Piece::new(6);
}
