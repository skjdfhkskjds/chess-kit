/*
Move format explanation

"data" contains all the move information, starting from LSB:

Field       :   bits     Decimal values
============================================
PIECE       :   3        0-7 (use only 0-6)
FROM        :   6        0-63
TO          :   6        0-63
CAPTURE     :   3        0-7 (captured piece)
PROMOTION   :   3        0-7 (piece promoted to)
ENPASSANT   :   1        0-1
DOUBLESTEP  :   1        0-1
CASTLING    :   1        0-1
SORTSCORE   :   16       0-65536


---------------------------------- move data -------------------------------------------
0000000000000000    0        0          0         000       000     000000 000000 000
SORTSCORE           CASTLING DOUBLESTEP ENPASSANT PROMOTION CAPTURE TO     FROM   PIECE
----------------------------------------------------------------------------------------

Field:      PROMOTION   CAPTURE     TO          FROM        PIECE
Bits:       3           3           6           6           3
Shift:      18 bits     15 bits     9 bits      3 bits      0 bits
& Value:    0x7 (7)     0x7 (7)     0x3F (63)   0x3F (63)   0x7 (7)

Field:      SORTSCORE   CASTLING    DOUBLESTEP  ENPASSANT
Bits:       32          1           1           1
Shift:      24 bits     23 bits     22 bits     21 bits
& Value:    0xFFFFFFFF  0x1         0x1 (1)     0x1 (1)

Get the TO field from "data" by:
    -- Shift 9 bits Right
    -- AND (&) with 0x3F

Obviously, storing information in "data" is the other way around.PIECE_NAME
Storing the "To" square: Shift LEFT 9 bits, then XOR with "data".


Note: credits to https://codeberg.org/mvanthoor/rustic
*/

use crate::primitives::{Piece, Pieces, Square};
use std::fmt::{self, Display};

#[repr(usize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Shift {
    Piece = 0,
    FromSquare = 3,
    ToSquare = 9,
    Capture = 15,
    Promotion = 18,
    EnPassant = 21,
    DoubleStep = 22,
    Castling = 23,
    SortScore = 24,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct Move {
    data: usize,
}

const MOVE_ONLY: usize = 0x00_00_00_00_00_FF_FF_FF;

// These functions decode the move data.
impl Move {
    pub fn new(
        piece: Piece,
        from: Square,
        to: Square,
        capture: Piece,
        promotion: Piece,
        en_passant: bool,
        double_step: bool,
        castling: bool,
    ) -> Self {
        let data = piece.unwrap()
            | (from.unwrap() << Shift::FromSquare as usize)
            | (to.unwrap() << Shift::ToSquare as usize)
            | (capture.unwrap() << Shift::Capture as usize)
            | (promotion.unwrap() << Shift::Promotion as usize)
            | ((en_passant as usize) << Shift::EnPassant as usize)
            | ((double_step as usize) << Shift::DoubleStep as usize)
            | ((castling as usize) << Shift::Castling as usize);

        Self { data }
    }

    pub fn piece(&self) -> Piece {
        Piece::new((self.data >> Shift::Piece as u64) & 0x7)
    }

    pub fn from(&self) -> Square {
        Square::new((self.data >> Shift::FromSquare as u64) & 0x3F)
    }

    pub fn to(&self) -> Square {
        Square::new((self.data >> Shift::ToSquare as u64) & 0x3F)
    }

    pub fn captured(&self) -> Piece {
        Piece::new((self.data >> Shift::Capture as u64) & 0x7)
    }

    pub fn promoted(&self) -> Piece {
        Piece::new((self.data >> Shift::Promotion as u64) & 0x7)
    }

    pub fn en_passant(&self) -> bool {
        ((self.data >> Shift::EnPassant as u64) & 0x1) as u8 == 1
    }

    pub fn double_step(&self) -> bool {
        ((self.data >> Shift::DoubleStep as u64) & 0x1) as u8 == 1
    }

    pub fn castling(&self) -> bool {
        ((self.data >> Shift::Castling as u64) & 0x1) as u8 == 1
    }

    pub fn get_sort_score(self) -> u32 {
        ((self.data >> Shift::SortScore as u64) & 0xFFFFFFFF) as u32
    }

    pub fn set_sort_score(&mut self, value: u32) {
        let mask: usize = 0xFFFFFFFF << Shift::SortScore as usize;
        let v: usize = (value as usize) << Shift::SortScore as usize;
        self.data = (self.data & !mask) | v;
    }

    pub fn to_short_move(self) -> ShortMove {
        ShortMove::new((self.data & MOVE_ONLY) as u32)
    }

    pub fn get_move(&self) -> u32 {
        (self.data & MOVE_ONLY) as u32
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let promotion = if self.promoted() != Pieces::NONE {
            format!("{}", self.promoted())
        } else {
            "".to_string()
        };
        write!(f, "{}{}{}", self.from(), self.to(), promotion)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct ShortMove {
    data: u32,
}

impl ShortMove {
    pub fn new(m: u32) -> Self {
        Self { data: m }
    }

    pub fn get_move(&self) -> u32 {
        self.data
    }
}
