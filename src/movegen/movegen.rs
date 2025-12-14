use crate::attack_table::AttackTable;
use crate::movegen::{MoveGenerator, MoveType};
use crate::position::{PositionAttacks, PositionMoves, PositionState};
use crate::primitives::{Black, MoveList, Sides, White, moves::MoveType::EnPassant};
use std::marker::PhantomData;

pub struct DefaultMoveGenerator<AT: AttackTable> {
    _attack_table: PhantomData<AT>,
}

impl<AT: AttackTable> MoveGenerator for DefaultMoveGenerator<AT> {
    // new creates a new move generator
    //
    // @impl: MoveGenerator::new
    #[inline(always)]
    fn new() -> Self {
        Self {
            _attack_table: PhantomData,
        }
    }

    // generate_moves generates all the pseudo-legal moves of the given move type
    // from the current position and pushes them to the move list
    //
    // @impl: MoveGenerator::generate_moves
    fn generate_moves<PositionT: PositionState + PositionAttacks>(
        &self,
        position: &PositionT,
        list: &mut MoveList,
        move_type: MoveType,
    ) {
        match position.turn() {
            Sides::White => {
                self.generate_moves_for_side::<White, PositionT>(position, list, move_type)
            }
            Sides::Black => {
                self.generate_moves_for_side::<Black, PositionT>(position, list, move_type)
            }
        }
    }

    // generate_legal_moves generates all the legal moves from the current position
    // and pushes them to the move list
    //
    // @impl: MoveGenerator::generate_legal_moves
    fn generate_legal_moves<PositionT: PositionState + PositionAttacks + PositionMoves>(
        &self,
        position: &PositionT,
        list: &mut MoveList,
    ) {
        // if the side to move is in check, just generate evasions during legal
        // move generation
        let move_type = if position.checkers().not_empty() {
            MoveType::Evasions
        } else {
            MoveType::NonEvasions
        };

        match position.turn() {
            Sides::White => {
                let king_square = position.king_square::<White>();
                let pinned =
                    position.king_blocker_pieces::<White>() & position.occupancy::<White>();

                // generate all the pseudo-legal moves
                self.generate_moves_for_side::<White, PositionT>(position, list, move_type);

                // filter the moves to only include legal moves
                list.filter(|mv| {
                    !(((pinned.has_square(mv.from()))
                        || mv.from() == king_square
                        || matches!(mv.type_of(), EnPassant))
                        && !position.is_legal_move::<White>(mv))
                })
            }
            Sides::Black => {
                let king_square = position.king_square::<Black>();
                let pinned =
                    position.king_blocker_pieces::<Black>() & position.occupancy::<Black>();

                // generate all the pseudo-legal moves
                self.generate_moves_for_side::<Black, PositionT>(position, list, move_type);

                // filter the moves to only include legal moves
                list.filter(|mv| {
                    !(((pinned.has_square(mv.from()))
                        || mv.from() == king_square
                        || matches!(mv.type_of(), EnPassant))
                        && !position.is_legal_move::<Black>(mv))
                })
            }
        }
    }
}
