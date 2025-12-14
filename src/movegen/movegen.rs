use crate::attack_table::AttackTable;
use crate::movegen::MoveType;
use crate::primitives::{Bitboard, Move, MoveList, Pieces, Square};
use std::marker::PhantomData;

pub struct MoveGenerator<AT: AttackTable> {
    _attack_table: PhantomData<AT>,
}

impl<AT: AttackTable> MoveGenerator<AT> {
    // new creates a new move generator
    //
    // @return: new move generator
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            _attack_table: PhantomData,
        }
    }
}

impl<AT: AttackTable> MoveGenerator<AT> {
    // push_moves pushes a set of moves of any non-pawn piece from the given
    // from square to the given to squares
    //
    // @param: from - square to push the moves from
    // @param: to_squares - bitboard of squares to push the moves to
    // @param: list - mutable reference to the move list
    // @return: void
    // @side-effects: modifies the `move list`
    #[inline(always)]
    pub(crate) fn push_moves(&self, from: Square, to_squares: Bitboard, list: &mut MoveList) {
        for to in to_squares.iter() {
            list.push(Move::new(from, to));
        }
    }

    // push_pawn_moves pushes a set of moves of a pawn from the given from square
    // to the given to squares
    //
    // note: we explicitly handle pawn moves here to handle branchless code for
    //       other piece types where en-passant captures are not possible
    //
    // @param: to_squares - bitboard of squares to push the moves to
    // @param: offset - offset to add to a given to square of a pawn to get the
    //                  from square of a pawn push
    // @param: list - mutable reference to the move list
    // @return: void
    // @side-effects: modifies the `move list`
    #[inline(always)]
    pub(crate) fn push_pawn_moves(&self, to_squares: Bitboard, offset: i8, list: &mut MoveList) {
        for to in to_squares.iter() {
            let from = Square::from_idx((to.idx() as i8 - offset) as usize);
            list.push(Move::new(from, to));
        }
    }

    // push_pawn_en_passant_captures pushes a set of moves of a pawn from the given
    // from squares to the given en passant square
    //
    // @param: from_squares - bitboard of squares to push the moves from
    // @param: en_passant_square - square to push the en passant captures to
    // @param: list - mutable reference to the move list
    // @return: void
    // @side-effects: modifies the `move list`
    #[inline(always)]
    pub(crate) fn push_pawn_en_passant_captures(
        &self,
        from_squares: Bitboard,
        en_passant_square: Square,
        list: &mut MoveList,
    ) {
        for from in from_squares.iter() {
            list.push(Move::new(from, en_passant_square).with_en_passant());
        }
    }

    // push_pawn_promotions pushes a set of moves of a pawn that is promoting
    // to the given to squares
    //
    // @param: to_squares - bitboard of squares to push the promoting pawns to
    // @param: offset - offset to add to a given to square of a pawn to get the
    //                  from square of a pawn promotion
    // @param: is_capture - whether the pawn is capturing a piece when promoting
    // @param: list - mutable reference to the move list
    // @param: move_type - move type to generate promotions of
    #[inline(always)]
    pub(crate) fn push_pawn_promotions(
        &self,
        to_squares: Bitboard,
        offset: i8,
        is_capture: bool,
        list: &mut MoveList,
        move_type: MoveType,
    ) {
        for to in to_squares.iter() {
            let from = Square::from_idx((to.idx() as i8 - offset) as usize);

            if !matches!(move_type, MoveType::Quiet) {
                list.push(Move::new(from, to).with_promotion(Pieces::Queen));
            }

            if matches!(move_type, MoveType::Evasions | MoveType::NonEvasions)
                || (matches!(move_type, MoveType::Capture) && is_capture)
                || (matches!(move_type, MoveType::Quiet) && !is_capture)
            {
                list.push(Move::new(from, to).with_promotion(Pieces::Knight));
                list.push(Move::new(from, to).with_promotion(Pieces::Bishop));
                list.push(Move::new(from, to).with_promotion(Pieces::Rook));
            }
        }
    }

    // push_castling_moves pushes a set of moves for castling from the given from
    // square to the given to squares
    //
    // @param: from - square to push the moves from
    // @param: to_squares - bitboard of squares to push the castling moves to
    // @param: list - mutable reference to the move list
    // @return: void
    // @side-effects: modifies the `move list`
    #[inline(always)]
    pub(crate) fn push_castling_moves(
        &self,
        from: Square,
        to_squares: Bitboard,
        list: &mut MoveList,
    ) {
        for to in to_squares.iter() {
            list.push(Move::new(from, to).with_castle());
        }
    }
}
