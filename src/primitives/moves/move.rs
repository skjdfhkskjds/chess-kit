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

use crate::primitives::{Pieces, Square};
use std::fmt::{self, Display};

// bit-shift offsets to parse the move data according to the schema above
const PIECE_SHIFT: u64 = 0;
const FROM_SHIFT: u64 = 3;
const TO_SHIFT: u64 = 9;
const CAPTURED_SHIFT: u64 = 15;
const PROMOTED_SHIFT: u64 = 18;
const IS_EN_PASSANT_SHIFT: u64 = 21;
const IS_DOUBLE_STEP_SHIFT: u64 = 22;
const IS_CASTLING_SHIFT: u64 = 23;
const SORT_SCORE_SHIFT: u64 = 24;

// data-type masks to extract the data value from the move data
const PIECE_MASK: u64 = 0x7;
const SQUARE_MASK: u64 = 0x3F;
const BOOL_MASK: u64 = 0x1;
const SCORE_MASK: u64 = 0xFFFFFFFF;
const MOVE_ONLY_MASK: u64 = 0x00_00_00_00_00_FF_FF_FF;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct Move {
    data: u64,
}

impl Move {
    // new creates a new move
    //
    // @param: piece - piece to move
    // @param: from - square to move from
    // @param: to - square to move to
    // @return: new move value
    pub fn new(piece: Pieces, from: Square, to: Square) -> Self {
        let data =
            piece.idx() as u64 | (from.idx() as u64) << FROM_SHIFT | (to.idx() as u64) << TO_SHIFT;

        Self { data }
    }

    // with_capture sets the captured piece for the move
    //
    // @param: self - mutable reference to the move
    // @param: captured - piece that was captured
    // @return: move with the captured piece set
    #[inline(always)]
    pub fn with_capture(mut self, captured: Pieces) -> Self {
        self.data |= (captured.idx() as u64) << CAPTURED_SHIFT;
        self
    }

    // with_promotion sets the promotion flag to true for the move
    //
    // @param: self - mutable reference to the move
    // @return: move with the promotion flag set
    #[inline(always)]
    pub fn with_promotion(mut self, promoted: Pieces) -> Self {
        self.data |= (promoted.idx() as u64) << PROMOTED_SHIFT;
        self
    }

    // with_en_passant sets the en passant flag to true for the move
    //
    // @param: self - mutable reference to the move
    // @return: move with the en passant flag set
    #[inline(always)]
    pub fn with_en_passant(mut self) -> Self {
        self.data |= 1 << IS_EN_PASSANT_SHIFT;
        self
    }

    // with_double_step sets the double step flag to true for the move
    //
    // @param: self - mutable reference to the move
    // @return: move with the double step flag set
    #[inline(always)]
    pub fn with_double_step(mut self) -> Self {
        self.data |= 1 << IS_DOUBLE_STEP_SHIFT;
        self
    }

    // with_castle sets the castle flag to true for the move
    //
    // @param: self - mutable reference to the move
    // @return: move with the castle flag set
    #[inline(always)]
    pub fn with_castle(mut self) -> Self {
        self.data |= 1 << IS_CASTLING_SHIFT;
        self
    }

    // piece returns the piece that is moving
    //
    // @param: self - immutable reference to the move
    // @return: piece that is moving
    #[inline(always)]
    pub fn piece(&self) -> Pieces {
        Pieces::from_idx(((self.data >> PIECE_SHIFT) & PIECE_MASK) as usize)
    }

    // from returns the square that the piece is moving from
    //
    // @param: self - immutable reference to the move
    // @return: square that the piece is moving from
    #[inline(always)]
    pub fn from(&self) -> Square {
        Self::to_square(self.data >> FROM_SHIFT)
    }

    // to returns the square that the piece is moving to
    //
    // @param: self - immutable reference to the move
    // @return: square that the piece is moving to
    #[inline(always)]
    pub fn to(&self) -> Square {
        Self::to_square(self.data >> TO_SHIFT)
    }

    // captured returns the piece that was captured
    //
    // @param: self - immutable reference to the move
    // @return: piece that was captured
    #[inline(always)]
    pub fn captured(&self) -> Pieces {
        Self::to_piece(self.data >> CAPTURED_SHIFT)
    }

    // promoted returns the piece that the pawn promoted to
    //
    // @param: self - immutable reference to the move
    // @return: piece that the pawn promoted to
    #[inline(always)]
    pub fn promoted(&self) -> Pieces {
        Self::to_piece(self.data >> PROMOTED_SHIFT)
    }

    // is_promotion returns whether the move is a promotion
    //
    // @param: self - immutable reference to the move
    // @return: true if the move is a promotion, false otherwise
    #[inline(always)]
    pub fn is_promotion(&self) -> bool {
        self.promoted() != Pieces::None
    }

    // is_en_passant returns whether the move is an en passant capture
    //
    // @param: self - immutable reference to the move
    // @return: true if the move is an en passant capture, false otherwise
    #[inline(always)]
    pub fn is_en_passant(&self) -> bool {
        Self::to_bool(self.data >> IS_EN_PASSANT_SHIFT)
    }

    // is_double_step returns whether the move is a double step
    //
    // @param: self - immutable reference to the move
    // @return: true if the move is a double step, false otherwise
    #[inline(always)]
    pub fn is_double_step(&self) -> bool {
        Self::to_bool(self.data >> IS_DOUBLE_STEP_SHIFT)
    }

    // is_castle returns whether the move is a castle
    //
    // @param: self - immutable reference to the move
    // @return: true if the move is a castle, false otherwise
    #[inline(always)]
    pub fn is_castle(&self) -> bool {
        Self::to_bool(self.data >> IS_CASTLING_SHIFT)
    }

    // get_sort_score returns the sort score of the move
    //
    // @param: self - immutable reference to the move
    // @return: sort score of the move
    // TODO: unused, find out if this is needed in search/eval
    #[inline(always)]
    pub fn get_sort_score(self) -> u32 {
        ((self.data >> SORT_SCORE_SHIFT) & SCORE_MASK) as u32
    }

    // set_sort_score sets the sort score of the move
    //
    // @param: self - mutable reference to the move
    // @param: value - sort score to set
    // @return: void
    // @side-effects: modifies the `move`
    // TODO: unused, find out if this is needed in search/eval
    #[inline(always)]
    pub fn set_sort_score(&mut self, value: u32) {
        let mask: u64 = SCORE_MASK << SORT_SCORE_SHIFT;
        let v: u64 = (value as u64) << SORT_SCORE_SHIFT;
        self.data = (self.data & !mask) | v;
    }

    // to_short_move converts the move to a short move
    //
    // @param: self - immutable reference to the move
    // @return: short move
    // TODO: unused, find out if this is needed in search/eval
    #[inline(always)]
    pub fn to_short_move(self) -> ShortMove {
        ShortMove::new((self.data & MOVE_ONLY_MASK) as u32)
    }

    // get_move returns the move as a u32
    //
    // @param: self - immutable reference to the move
    // @return: move as a u32
    // TODO: unused, find out if this is needed in search/eval
    #[inline(always)]
    pub fn get_move(&self) -> u32 {
        (self.data & MOVE_ONLY_MASK) as u32
    }

    // to_square is a helper routine that converts the shifted value to a square
    //
    // @param: value - bit-shifted data value to convert to a square
    // @return: square
    #[inline(always)]
    fn to_square(value: u64) -> Square {
        Square::from_idx((value & SQUARE_MASK) as usize)
    }

    // to_piece is a helper routine that converts the shifted value to a piece
    //
    // @param: value - bit-shifted data value to convert to a piece
    // @return: piece
    #[inline(always)]
    fn to_piece(value: u64) -> Pieces {
        Pieces::from_idx((value & PIECE_MASK) as usize)
    }

    // to_bool is a helper routine that converts the shifted value to a boolean
    //
    // @param: value - bit-shifted data value to convert to a boolean
    // @return: boolean
    #[inline(always)]
    fn to_bool(value: u64) -> bool {
        value & BOOL_MASK == 1
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.from(), self.to())?;
        if self.is_promotion() {
            write!(f, "{}", self.promoted())?;
        }
        Ok(())
    }
}

// TODO: unused, find out if this is needed in search/eval
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
