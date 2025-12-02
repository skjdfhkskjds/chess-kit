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

    // push_moves pushes a set of moves to the move list as defined by the
    // given piece at the from square to the each of the to squares.
    //
    // @param: position - immutable reference to the position
    // @param: piece - piece to push the moves for
    // @param: from - square to push the moves from
    // @param: to_squares - bitboard of squares to push the moves to
    // @param: list - mutable reference to the move list
    // @return: void
    // @side-effects: modifies the `move list`
    pub(crate) fn push_moves<SideT: SideRanks, StateT: State + GameStateExt>(
        &self,
        position: &Position<AT, StateT>,
        piece: Pieces,
        from: Square,
        to_squares: Bitboard,
        list: &mut MoveList,
    ) {
        let en_passant = position.state.en_passant();

        // push a move for each of the `to` squares
        for to in to_squares.iter() {
            let mut mv = Move::new(piece, from, to);

            // set the captured piece for the move if there is one
            //
            // Note: a captured piece is the piece that currently occupies the
            //       target square. Notice that this definition excludes en-passant
            //       captures.
            let captured = position.pieces[to.idx()];
            if !matches!(captured, Pieces::None) {
                mv = mv.with_capture(captured);
            }

            // handle the special cases for the piece
            match piece {
                Pieces::Pawn => {
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
                    } else if to.distance(from) == 16 {
                        // the move is a double step pawn push
                        mv = mv.with_double_step();
                    } else if to.on_rank(SideT::PROMOTION_RANK) {
                        // generate all possible promotion moves
                        PROMOTION_PIECES.iter().for_each(|promotion_piece| {
                            list.push(mv.with_promotion(*promotion_piece));
                        });

                        // all move variants have been generated, move on to the
                        // next move instead of exiting out of the conditional
                        // block
                        continue;
                    }
                }
                Pieces::King => {
                    // check if the move is a castle
                    if to.distance(from) == 2 {
                        mv = mv.with_castle();
                    }
                }
                _ => {
                    // no special handling required for other pieces
                }
            }

            // push the move to the list
            list.push(mv);
        }
    }
}
