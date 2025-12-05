use crate::primitives::{Pieces, Square};
use core::mem::transmute;
use std::fmt::{self, Display};

// bit-shift offsets to parse the move data according to the schema below
const TO_SHIFT: u16 = 6;
const PROMOTED_SHIFT: u16 = 12;
const MOVE_TYPE_SHIFT: u16 = 14;

// data-type masks to extract the data value from the move data
const SQUARE_MASK: u16 = 0x3F;
const PROMOTED_MASK: u16 = 0x3;
const MOVE_TYPE_MASK: u16 = 0x3;

// MoveType is an enum that represents the type of move
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u16)]
pub enum MoveType {
    Normal,
    Promotion = 1 << MOVE_TYPE_SHIFT,
    EnPassant = 2 << MOVE_TYPE_SHIFT,
    Castle = 3 << MOVE_TYPE_SHIFT,
}

// Move is a compact representation of a move
//
// the schema is as follows:
//
//         | from |   to |  promoted_to |    move_type |
// | ----- | ---- | ---- | ------------ | ------------ |
// |  bits |  0-5 | 6-11 |       12-14  |        14-16 |
// |  mask | 0x3f | 0x3f | 0x3 + Knight |          0x3 |
// | shift |    0 |    6 |           12 |           14 |
//
// note: for `promoted_to`, we can compact the Pieces enum into a 2-bit value
//       by making the smallest promotable piece (Knight) the base offset
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct Move {
    data: u16,
}

impl Move {
    // new creates a new move
    //
    // @param: from - square to move from
    // @param: to - square to move to
    // @return: new instance of a move
    pub fn new(from: Square, to: Square) -> Self {
        let data = (from.idx() as u16) + ((to.idx() as u16) << TO_SHIFT);

        Self { data }
    }

    // with_promotion sets the promotion data for the move and sets the move
    // type to promotion
    //
    // @param: promoted - piece that the pawn promoted to
    // @return: move with the promotion data set
    #[inline(always)]
    pub fn with_promotion(mut self, promoted: Pieces) -> Self {
        // convert the promoted piece to the 2-bit representation and set the
        // bits in the move data
        self.data += ((promoted.idx() - Pieces::Knight.idx()) as u16) << PROMOTED_SHIFT;

        // set the move type to promotion
        self.data += MoveType::Promotion as u16;

        self
    }

    // with_en_passant sets the move type to en passant
    //
    // @return: move with the move type set to en passant
    #[inline(always)]
    pub fn with_en_passant(mut self) -> Self {
        self.data += MoveType::EnPassant as u16;
        self
    }

    // with_castle sets the move type to castle
    //
    // @return: move with the move type set to castle
    #[inline(always)]
    pub fn with_castle(mut self) -> Self {
        self.data += MoveType::Castle as u16;
        self
    }

    // from returns the square that the piece is moving from
    //
    // @return: square that the piece is moving from
    #[inline(always)]
    pub fn from(&self) -> Square {
        Square::from_idx((self.data & SQUARE_MASK) as usize)
    }

    // to returns the square that the piece is moving to
    //
    // @param: self - immutable reference to the move
    // @return: square that the piece is moving to
    #[inline(always)]
    pub fn to(&self) -> Square {
        Square::from_idx(((self.data >> TO_SHIFT) & SQUARE_MASK) as usize)
    }

    // promoted_to returns the piece that the pawn promoted to
    //
    // @return: piece that the pawn promoted to
    #[inline(always)]
    pub fn promoted_to(&self) -> Pieces {
        Pieces::from_idx(
            ((self.data >> PROMOTED_SHIFT) & PROMOTED_MASK) as usize + Pieces::Knight.idx(),
        )
    }

    // type_of returns the type of move
    //
    // @return: type of move
    #[inline(always)]
    pub fn type_of(&self) -> MoveType {
        // SAFETY: the move type is always a valid move type
        unsafe { transmute::<u16, MoveType>(self.data & (MOVE_TYPE_MASK << MOVE_TYPE_SHIFT)) }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.from(), self.to())?;
        if matches!(self.type_of(), MoveType::Promotion) {
            write!(f, "{}", self.promoted_to())?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::Square::*;

    #[test]
    fn new_sets_from_to_and_default_type() {
        let mv = Move::new(A2, A4);
        assert_eq!(mv.from(), A2);
        assert_eq!(mv.to(), A4);
        assert_eq!(mv.type_of(), MoveType::Normal);
    }

    #[test]
    fn promotion_sets_type_and_promoted_piece() {
        let mv = Move::new(A7, A8).with_promotion(Pieces::Queen);
        assert_eq!(mv.type_of(), MoveType::Promotion);
        assert_eq!(mv.promoted_to(), Pieces::Queen);
        assert_eq!(format!("{}", mv), "a7a8Q");
    }

    #[test]
    fn en_passant_sets_type_bits_only() {
        let mv = Move::new(E5, D6).with_en_passant();
        assert_eq!(mv.type_of(), MoveType::EnPassant);
        assert_eq!(mv.from(), E5);
        assert_eq!(mv.to(), D6);
    }

    #[test]
    fn castle_sets_type_bits_only() {
        let mv = Move::new(E1, G1).with_castle();
        assert_eq!(mv.type_of(), MoveType::Castle);
        assert_eq!(mv.from(), E1);
        assert_eq!(mv.to(), G1);
    }

    #[test]
    fn promoted_piece_round_trips_with_knight_base_offset() {
        let mv = Move::new(B7, B8).with_promotion(Pieces::Knight);
        assert_eq!(mv.promoted_to(), Pieces::Knight);
        assert_eq!(mv.type_of(), MoveType::Promotion);
    }
}
