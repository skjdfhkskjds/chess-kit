use chess_kit_eval::{Accumulator, EvalState, Score};
use chess_kit_movegen::MoveGenerator;
use chess_kit_position::{PositionAttacks, PositionMoves, PositionState};
use chess_kit_primitives::{Depth, Move, MoveList, Sides};

use crate::SearchResult;

/// A fixed-depth negamax search with alpha-beta pruning.
#[derive(Default)]
pub struct Negamax {
    nodes: u64,
}

impl Negamax {
    /// Score used to represent a checkmate at the root of the search.
    pub const CHECKMATE_SCORE: Score = 100_000;

    const INFINITY: Score = 1_000_000;

    pub const fn new() -> Self {
        Self { nodes: 0 }
    }

    /// Searches `position` to a fixed depth and returns the best move found.
    ///
    /// The accumulator must already contain the evaluation state for the root
    /// position. Evaluation scores are interpreted from White's perspective.
    pub fn search<MoveGeneratorT, PositionT, AccumulatorT, EvalStateT>(
        &mut self,
        position: &mut PositionT,
        move_generator: &MoveGeneratorT,
        accumulator: &mut AccumulatorT,
        depth: Depth,
    ) -> SearchResult
    where
        MoveGeneratorT: MoveGenerator,
        PositionT: PositionState + PositionAttacks + PositionMoves,
        AccumulatorT: Accumulator<EvalStateT>,
        EvalStateT: EvalState,
    {
        assert!(depth >= 0, "search depth must be non-negative");

        self.nodes = 0;
        let (score, best_move) = self.negamax(
            position,
            move_generator,
            accumulator,
            depth,
            0,
            -Self::INFINITY,
            Self::INFINITY,
        );

        SearchResult::new(best_move, score, self.nodes)
    }

    fn negamax<MoveGeneratorT, PositionT, AccumulatorT, EvalStateT>(
        &mut self,
        position: &mut PositionT,
        move_generator: &MoveGeneratorT,
        accumulator: &mut AccumulatorT,
        depth: Depth,
        ply: Depth,
        mut alpha: Score,
        beta: Score,
    ) -> (Score, Option<Move>)
    where
        MoveGeneratorT: MoveGenerator,
        PositionT: PositionState + PositionAttacks + PositionMoves,
        AccumulatorT: Accumulator<EvalStateT>,
        EvalStateT: EvalState,
    {
        self.nodes += 1;

        if depth == 0 {
            return (Self::evaluate(position, accumulator), None);
        }

        let mut moves = MoveList::new();
        move_generator.generate_legal_moves(position, &mut moves);

        if moves.is_empty() {
            let score = if position.checkers().not_empty() {
                -Self::CHECKMATE_SCORE + Score::from(ply)
            } else {
                0
            };
            return (score, None);
        }

        let mut best_score = -Self::INFINITY;
        let mut best_move = None;

        for &mv in &moves {
            let eval = accumulator.push_next();
            position.make_move(mv, eval);

            let (child_score, _) = self.negamax(
                position,
                move_generator,
                accumulator,
                depth - 1,
                ply + 1,
                -beta,
                -alpha,
            );
            let score = -child_score;

            position.unmake_move(mv);
            accumulator.pop();

            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }

            alpha = alpha.max(score);
            if alpha >= beta {
                break;
            }
        }

        (best_score, best_move)
    }

    fn evaluate<PositionT, AccumulatorT, EvalStateT>(
        position: &PositionT,
        accumulator: &mut AccumulatorT,
    ) -> Score
    where
        PositionT: PositionState,
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
