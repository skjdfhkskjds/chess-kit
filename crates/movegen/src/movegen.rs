use crate::{MoveGenerationStrategy, MoveGenerator};
use chess_kit_attack_table::AttackTable;
use chess_kit_position::{PositionAttacks, PositionMoves, PositionView};
use chess_kit_primitives::{MoveList, call_as, moves::MoveType::EnPassant};
use std::marker::PhantomData;

/// `DefaultMoveGenerator` is a default implementation of the `MoveGenerator` trait
///
/// @type
pub struct DefaultMoveGenerator<AT: AttackTable> {
    _attack_table: PhantomData<AT>,
}

impl<AT: AttackTable> MoveGenerator for DefaultMoveGenerator<AT> {
    /// new creates a new move generator
    ///
    /// @impl: MoveGenerator::new
    #[inline]
    fn new() -> Self {
        Self {
            _attack_table: PhantomData,
        }
    }

    /// generate_moves generates all the pseudo-legal moves of the given move type
    /// from the current position and pushes them to the move list
    ///
    /// @impl: MoveGenerator::generate_moves
    fn generate_moves<PositionT: PositionView + PositionAttacks>(
        &self,
        position: &PositionT,
        list: &mut MoveList,
        strategy: MoveGenerationStrategy,
    ) {
        call_as!(position.turn(), |SideT| self
            .generate_moves_for_side::<SideT, PositionT>(
                position, list, strategy
            ));
    }

    /// generate_legal_moves generates all the legal moves from the current position
    /// and pushes them to the move list
    ///
    /// @impl: MoveGenerator::generate_legal_moves
    fn generate_legal_moves<PositionT: PositionView + PositionAttacks + PositionMoves>(
        &self,
        position: &PositionT,
        list: &mut MoveList,
    ) {
        // if the side to move is in check, just generate evasions during legal
        // move generation
        let strategy = if position.checkers().not_empty() {
            MoveGenerationStrategy::Evasions
        } else {
            MoveGenerationStrategy::NonEvasions
        };

        call_as!(position.turn(), |SideT| {
            let king_square = position.king_square::<SideT>();
            let pinned = position.king_blocker_pieces::<SideT>() & position.occupancy::<SideT>();

            self.generate_moves_for_side::<SideT, PositionT>(position, list, strategy);
            list.retain(|mv| {
                !((pinned.has_square(mv.from())
                    || mv.from() == king_square
                    || matches!(mv.type_of(), EnPassant))
                    && !position.is_legal_move::<SideT>(*mv))
            });
        });
    }
}
