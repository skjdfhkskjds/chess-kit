use chess_kit_eval::{Accumulator, EvalState, Score};
use chess_kit_movegen::MoveGenerator;
use chess_kit_position::{PositionAttacks, PositionMoves, PositionView};
use chess_kit_primitives::{Depth, Move, MoveList, Sides, ZobristKey};
use chess_kit_transposition::TranspositionTable;

use crate::{Bound, SearchNode, SearchResult, move_ordering, quiescence};

/// Negamax is a fixed-depth negamax search with alpha-beta pruning
///
/// @type
#[derive(Default)]
pub struct Negamax {
    nodes: u64,
}

/// SearchContext groups the mutable state shared by recursive search calls
///
/// @type
struct SearchContext<'a, MoveGeneratorT, TranspositionTableT, AccumulatorT> {
    move_generator: &'a MoveGeneratorT,
    transposition_table: &'a mut TranspositionTableT,
    accumulator: &'a mut AccumulatorT,
}

impl Negamax {
    /// Score used to represent a checkmate at the root of the search.
    pub const CHECKMATE_SCORE: Score = 100_000;

    pub(crate) const INFINITY: Score = 1_000_000;
    const MATE_SCORE_THRESHOLD: Score = Self::CHECKMATE_SCORE - i8::MAX as Score;

    /// new creates a new negamax search
    ///
    /// @return: new negamax search
    pub const fn new() -> Self {
        Self { nodes: 0 }
    }

    /// search searches a position to a fixed depth and returns the best move
    /// found
    ///
    /// note: the accumulator must already contain the evaluation state for the
    ///       root position. evaluation scores are interpreted from White's
    ///       perspective
    ///
    /// note: the transposition table is retained between calls and must be
    ///       cleared if the evaluation function changes
    ///
    /// @param: position - mutable reference to the root position
    /// @param: move_generator - immutable reference to the move generator
    /// @param: transposition_table - mutable reference to the transposition table
    /// @param: accumulator - mutable reference to the evaluation accumulator
    /// @param: depth - fixed depth to search
    /// @return: best move, score, and visited node count for the search
    /// @side-effects: updates the transposition table and internal node count
    pub fn search<MoveGeneratorT, PositionT, AccumulatorT, EvalStateT, TranspositionTableT>(
        &mut self,
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

        self.nodes = 0;
        let mut context = SearchContext {
            move_generator,
            transposition_table,
            accumulator,
        };
        let (score, best_move) = self.negamax(
            position,
            &mut context,
            depth,
            0,
            -Self::INFINITY,
            Self::INFINITY,
        );

        SearchResult::new(best_move, score, self.nodes)
    }

    /// negamax recursively searches a position with an alpha-beta window
    ///
    /// @param: position - mutable reference to the current position
    /// @param: context - mutable reference to the shared search context
    /// @param: depth - remaining depth to search
    /// @param: ply - distance of the current node from the root
    /// @param: alpha - lower bound of the search window
    /// @param: beta - upper bound of the search window
    /// @return: best score and move found for the current node
    /// @side-effects: updates the transposition table and internal node count
    fn negamax<MoveGeneratorT, PositionT, AccumulatorT, EvalStateT, TranspositionTableT>(
        &mut self,
        position: &mut PositionT,
        context: &mut SearchContext<'_, MoveGeneratorT, TranspositionTableT, AccumulatorT>,
        depth: Depth,
        ply: Depth,
        mut alpha: Score,
        beta: Score,
    ) -> (Score, Option<Move>)
    where
        MoveGeneratorT: MoveGenerator,
        PositionT: PositionView + PositionAttacks + PositionMoves,
        AccumulatorT: Accumulator<EvalStateT>,
        EvalStateT: EvalState,
        TranspositionTableT: TranspositionTable<SearchNode>,
    {
        if depth == 0 {
            let score = quiescence::search(
                position,
                context.move_generator,
                context.accumulator,
                &mut self.nodes,
                ply,
                alpha,
                beta,
            );
            return (score, None);
        }

        self.nodes += 1;
        let key = position.key();
        let cached = context.transposition_table.probe(key).copied();
        let hash_move = cached.and_then(|node| node.best_move());

        if let Some(node) = cached
            && node.depth() >= depth
        {
            let score = Self::score_from_tt(node.score(), ply);
            let cutoff = match node.bound() {
                Bound::Exact => true,
                Bound::Lower => score >= beta,
                Bound::Upper => score <= alpha,
            };

            if cutoff {
                return (score, node.best_move());
            }
        }

        let mut moves = MoveList::new();
        context
            .move_generator
            .generate_legal_moves(position, &mut moves);

        if moves.is_empty() {
            let score = if position.checkers().not_empty() {
                -Self::CHECKMATE_SCORE + Score::from(ply)
            } else {
                0
            };
            Self::store(
                context.transposition_table,
                key,
                SearchNode::new(depth, Self::score_to_tt(score, ply), Bound::Exact, None),
            );
            return (score, None);
        }

        move_ordering::order_moves(position, &mut moves, hash_move);

        let original_alpha = alpha;
        let mut best_score = -Self::INFINITY;
        let mut best_move = None;

        for &mv in &moves {
            let eval = context.accumulator.push_next();
            let delta = position.play_unchecked(mv);
            eval.apply(delta);

            let (child_score, _) =
                self.negamax(position, context, depth - 1, ply + 1, -beta, -alpha);
            let score = -child_score;

            position.undo(mv);
            context.accumulator.pop();

            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }

            alpha = alpha.max(score);
            if alpha >= beta {
                break;
            }
        }

        let bound = if best_score <= original_alpha {
            Bound::Upper
        } else if best_score >= beta {
            Bound::Lower
        } else {
            Bound::Exact
        };
        Self::store(
            context.transposition_table,
            key,
            SearchNode::new(depth, Self::score_to_tt(best_score, ply), bound, best_move),
        );

        (best_score, best_move)
    }

    /// store caches a search node unless a deeper entry already exists for the
    /// same position
    ///
    /// @param: transposition_table - mutable reference to the transposition table
    /// @param: key - zobrist key of the searched position
    /// @param: node - search node to cache
    /// @return: void
    /// @side-effects: may modify the transposition table
    fn store<TranspositionTableT>(
        transposition_table: &mut TranspositionTableT,
        key: ZobristKey,
        node: SearchNode,
    ) where
        TranspositionTableT: TranspositionTable<SearchNode>,
    {
        let should_replace = match transposition_table.probe(key) {
            Some(cached) => cached.depth() <= node.depth(),
            None => true,
        };

        if should_replace {
            transposition_table.insert(key, node);
        }
    }

    /// score_to_tt converts a root-relative mate score into table-relative form
    ///
    /// @param: score - score to store in the transposition table
    /// @param: ply - distance of the current node from the root
    /// @return: score normalized for transposition-table storage
    #[inline]
    const fn score_to_tt(score: Score, ply: Depth) -> Score {
        if score >= Self::MATE_SCORE_THRESHOLD {
            score + ply as Score
        } else if score <= -Self::MATE_SCORE_THRESHOLD {
            score - ply as Score
        } else {
            score
        }
    }

    /// score_from_tt converts a table-relative mate score into root-relative form
    ///
    /// @param: score - score read from the transposition table
    /// @param: ply - distance of the current node from the root
    /// @return: score normalized for the current search root
    #[inline]
    const fn score_from_tt(score: Score, ply: Depth) -> Score {
        if score >= Self::MATE_SCORE_THRESHOLD {
            score - ply as Score
        } else if score <= -Self::MATE_SCORE_THRESHOLD {
            score + ply as Score
        } else {
            score
        }
    }

    /// evaluate returns the static evaluation from the side-to-move's perspective
    ///
    /// @param: position - immutable reference to the position to evaluate
    /// @param: accumulator - mutable reference to the evaluation accumulator
    /// @return: static evaluation from the side-to-move's perspective
    pub(crate) fn evaluate<PositionT, AccumulatorT, EvalStateT>(
        position: &PositionT,
        accumulator: &mut AccumulatorT,
    ) -> Score
    where
        PositionT: PositionView,
        AccumulatorT: Accumulator<EvalStateT>,
        EvalStateT: EvalState,
    {
        let score = accumulator.latest_mut().score();
        match position.turn() {
            Sides::White => score,
            Sides::Black => -score,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Negamax;

    #[test]
    fn transposition_scores_preserve_mate_distance_across_plies() {
        let winning_score = Negamax::CHECKMATE_SCORE - 5;
        let stored_winning_score = Negamax::score_to_tt(winning_score, 2);
        assert_eq!(
            Negamax::score_from_tt(stored_winning_score, 7),
            winning_score - 5
        );

        let losing_score = -Negamax::CHECKMATE_SCORE + 5;
        let stored_losing_score = Negamax::score_to_tt(losing_score, 2);
        assert_eq!(
            Negamax::score_from_tt(stored_losing_score, 7),
            losing_score + 5
        );
    }

    #[test]
    fn transposition_scores_leave_normal_evaluations_unchanged() {
        assert_eq!(Negamax::score_to_tt(900, 12), 900);
        assert_eq!(Negamax::score_from_tt(900, 12), 900);
    }
}
