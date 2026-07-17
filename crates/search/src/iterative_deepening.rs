use chess_kit_eval::{Accumulator, EvalState};
use chess_kit_movegen::MoveGenerator;
use chess_kit_position::{PositionAttacks, PositionMoves, PositionView};
use chess_kit_primitives::Depth;
use chess_kit_transposition::TranspositionTable;

use crate::{Negamax, SearchNode, SearchResult};

/// iterative_deepening searches successively deeper depths with negamax
///
/// The transposition table is retained between iterations so shallower results
/// provide hash moves to deeper searches. The returned score and best move come
/// from the requested depth, while the node count is cumulative across every
/// completed iteration.
///
/// @param: negamax - fixed-depth negamax search invoked for each iteration
/// @param: position - mutable reference to the root position
/// @param: move_generator - immutable reference to the move generator
/// @param: transposition_table - mutable reference to the transposition table
/// @param: accumulator - mutable reference to the evaluation accumulator
/// @param: depth - maximum depth to search
/// @return: final best move and score with a cumulative visited node count
/// @side-effects: updates the transposition table and negamax node count
pub fn iterative_deepening<
    MoveGeneratorT,
    PositionT,
    AccumulatorT,
    EvalStateT,
    TranspositionTableT,
>(
    negamax: &mut Negamax,
    position: &mut PositionT,
    move_generator: &MoveGeneratorT,
    transposition_table: &mut TranspositionTableT,
    accumulator: &mut AccumulatorT,
    depth: Depth,
) -> SearchResult
where
    MoveGeneratorT: MoveGenerator,
    PositionT: PositionView + PositionAttacks + PositionMoves,
    AccumulatorT: Accumulator<EvalStateT>,
    EvalStateT: EvalState,
    TranspositionTableT: TranspositionTable<SearchNode>,
{
    assert!(depth >= 0, "search depth must be non-negative");

    if depth == 0 {
        return negamax.search(
            position,
            move_generator,
            transposition_table,
            accumulator,
            0,
        );
    }

    let mut completed = None;
    let mut nodes = 0;
    for current_depth in 1..=depth {
        let result = negamax.search(
            position,
            move_generator,
            transposition_table,
            accumulator,
            current_depth,
        );
        nodes += result.nodes;
        completed = Some(result);
    }

    let completed = completed.expect("a positive search depth must complete an iteration");
    SearchResult::new(completed.best_move, completed.score, nodes)
}
