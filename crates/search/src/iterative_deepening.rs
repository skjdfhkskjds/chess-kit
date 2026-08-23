use std::time::Instant;

use chess_kit_eval::{Accumulator, EvalState};
use chess_kit_movegen::MoveGenerator;
use chess_kit_position::{PositionAttacks, PositionMoves, PositionView};
use chess_kit_primitives::Depth;
use chess_kit_transposition::TranspositionTable;

use crate::{IterativeSearchResult, Negamax, SearchNode, SearchResult, negamax::SearchStatus};

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
    if depth == 0 {
        return negamax.search(
            position,
            move_generator,
            transposition_table,
            accumulator,
            0,
        );
    }

    iterative_deepening_until(
        negamax,
        position,
        move_generator,
        transposition_table,
        accumulator,
        depth,
        None,
    )
    .result
}

/// `iterative_deepening_until` searches successively deeper depths until the
/// maximum depth or deadline is reached.
///
/// The first iteration always completes so callers receive a legal fallback
/// move even when given an already-expired deadline. An interrupted iteration
/// never replaces the move and score from the last completed one.
///
/// @param: negamax - fixed-depth negamax search invoked for each iteration
/// @param: position - mutable reference to the root position
/// @param: move_generator - immutable reference to the move generator
/// @param: transposition_table - mutable reference to the transposition table
/// @param: accumulator - mutable reference to the evaluation accumulator
/// @param: maximum_depth - greatest depth allowed in plies
/// @param: deadline - optional instant at which search should stop
/// @return: result and depth from the last fully completed iteration
/// @side-effects: updates the transposition table and negamax node count
pub fn iterative_deepening_until<
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
    maximum_depth: Depth,
    deadline: Option<Instant>,
) -> IterativeSearchResult
where
    MoveGeneratorT: MoveGenerator,
    PositionT: PositionView + PositionAttacks + PositionMoves,
    AccumulatorT: Accumulator<EvalStateT>,
    EvalStateT: EvalState,
    TranspositionTableT: TranspositionTable<SearchNode>,
{
    assert!(maximum_depth > 0, "maximum search depth must be positive");

    let mut completed = None;
    let mut nodes = 0;
    for current_depth in 1..=maximum_depth {
        // Completing depth one guarantees a usable move for extremely short
        // controls; every later iteration observes the caller's deadline.
        let iteration_deadline = if completed.is_some() { deadline } else { None };
        match negamax.search_until(
            position,
            move_generator,
            transposition_table,
            accumulator,
            current_depth,
            iteration_deadline,
        ) {
            SearchStatus::Complete(result) => {
                nodes += result.nodes;
                completed = Some((result, current_depth));
            }
            SearchStatus::Stopped(visited) => {
                nodes += visited;
                break;
            }
        }
    }

    let (completed, depth) = completed.expect("depth one must complete without a deadline");
    IterativeSearchResult::new(
        SearchResult::new(completed.best_move, completed.score, nodes),
        depth,
    )
}
