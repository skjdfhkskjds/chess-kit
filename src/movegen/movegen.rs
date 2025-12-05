use crate::attack_table::AttackTable;
use crate::position::Position;
use crate::primitives::{Bitboard, GameStateExt, Move, MoveList, Pieces, SideRanks, Square, State};

// list of pieces that a pawn can promote to
const PROMOTION_PIECES: [Pieces; 4] = [Pieces::Queen, Pieces::Rook, Pieces::Bishop, Pieces::Knight];

pub struct MoveGenerator<AT: AttackTable> {
    pub(crate) attack_table: &'static AT,
}

impl<AT: AttackTable> MoveGenerator<AT> {
    // new creates a new move generator
    //
    // @param: attack_table - attack table to use for the move generator
    // @return: new move generator
    #[inline(always)]
    pub fn new(attack_table: &'static AT) -> Self {
        Self { attack_table }
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
    pub(crate) fn push_moves(&self, from: Square, to_squares: Bitboard, list: &mut MoveList) {
        // push a move for each of the `to` squares
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
    // @param: from - square to push the moves from
    // @param: to_squares - bitboard of squares to push the moves to
    // @param: list - mutable reference to the move list
    // @param: en_passant - en passant square, if any
    // @return: void
    // @side-effects: modifies the `move list`
    pub(crate) fn push_pawn_moves<SideT: SideRanks>(
        &self,
        from: Square,
        to_squares: Bitboard,
        list: &mut MoveList,
        en_passant: Option<Square>,
    ) {
        // handle promotion moves first
        if from.on_rank(SideT::PROMOTABLE_RANK) {
            for to in to_squares.iter() {
                PROMOTION_PIECES.iter().for_each(|promotion_piece| {
                    list.push(Move::new(from, to).with_promotion(*promotion_piece));
                });
            }
            return;
        }

        // push a move for each of the `to` squares
        for to in to_squares.iter() {
            let mut mv = Move::new(from, to);

            // a pawn is moving, so we need to handle the cases
            //
            // 1. en passant capture
            // 2. double step pawn push
            // 3. promotion

            // check if the move is an en passant capture
            let is_en_passant = match en_passant {
                Some(square) => square == to,
                None => false,
            };

            if is_en_passant {
                // the move is an en passant capture
                mv = mv.with_en_passant();
            }

            // push the move to the list
            list.push(mv);
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
    pub(crate) fn push_castling_moves(
        &self,
        from: Square,
        to_squares: Bitboard,
        list: &mut MoveList,
    ) {
        // push a move for each of the `to` squares
        for to in to_squares.iter() {
            list.push(Move::new(from, to).with_castle());
        }
    }
}
