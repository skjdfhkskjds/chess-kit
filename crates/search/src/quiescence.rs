use chess_kit_eval::{Accumulator, EvalState, Score};
use chess_kit_movegen::{MoveGenerator, MoveType};
use chess_kit_position::{PositionAttacks, PositionMoves, PositionView};
use chess_kit_primitives::{Black, Depth, MoveList, Sides, White};

use crate::Negamax;

/// search continues through tactical moves until the position is quiet
///
/// @param: position - mutable reference to the current position
/// @param: move_generator - immutable reference to the move generator
/// @param: accumulator - mutable reference to the evaluation accumulator
/// @param: nodes - mutable reference to the search node count
/// @param: ply - distance of the current node from the root
/// @param: alpha - lower bound of the search window
/// @param: beta - upper bound of the search window
/// @return: best score found for the current node
/// @side-effects: updates the internal node count
pub(crate) fn search<MoveGeneratorT, PositionT, AccumulatorT, EvalStateT>(
    position: &mut PositionT,
    move_generator: &MoveGeneratorT,
    accumulator: &mut AccumulatorT,
    nodes: &mut u64,
    ply: Depth,
    mut alpha: Score,
    beta: Score,
) -> Score
where
    MoveGeneratorT: MoveGenerator,
    PositionT: PositionView + PositionAttacks + PositionMoves,
    AccumulatorT: Accumulator<EvalStateT>,
    EvalStateT: EvalState,
{
    *nodes += 1;

    let in_check = position.checkers().not_empty();
    let mut moves = MoveList::new();

    if in_check {
        // Standing pat is not legal while in check, so every legal evasion is
        // part of the quiescence search.
        move_generator.generate_legal_moves(position, &mut moves);
        if moves.is_empty() {
            return -Negamax::CHECKMATE_SCORE + Score::from(ply);
        }
    } else {
        move_generator.generate_moves(position, &mut moves, MoveType::Capture);
        retain_legal_moves(position, &mut moves);

        // Capture generation cannot distinguish a quiet position from
        // stalemate, so confirm terminal positions when no tactical move exists.
        if moves.is_empty() {
            let mut legal_moves = MoveList::new();
            move_generator.generate_legal_moves(position, &mut legal_moves);
            if legal_moves.is_empty() {
                return 0;
            }
        }
    }

    // Depth uses the full i8 range. Stop before incrementing past its maximum
    // in pathological checking sequences.
    if ply == i8::MAX {
        return Negamax::evaluate(position, accumulator);
    }

    let mut best_score = -Negamax::INFINITY;
    if !in_check {
        let stand_pat = Negamax::evaluate(position, accumulator);
        best_score = stand_pat;

        if stand_pat >= beta {
            return stand_pat;
        }
        alpha = alpha.max(stand_pat);
    }

    for &mv in &moves {
        let eval = accumulator.push_next();
        let delta = position.play_unchecked(mv);
        eval.apply(delta);

        let score = -search(
            position,
            move_generator,
            accumulator,
            nodes,
            ply + 1,
            -beta,
            -alpha,
        );

        position.undo(mv);
        accumulator.pop();

        best_score = best_score.max(score);
        alpha = alpha.max(score);
        if alpha >= beta {
            break;
        }
    }

    best_score
}

/// retain_legal_moves removes pseudo-legal tactical moves that expose the king
///
/// @param: position - immutable reference to the current position
/// @param: moves - mutable reference to the tactical move list
/// @return: void
/// @side-effects: may remove moves from the move list
fn retain_legal_moves<PositionT>(position: &PositionT, moves: &mut MoveList)
where
    PositionT: PositionView + PositionMoves,
{
    match position.turn() {
        Sides::White => moves.retain(|mv| position.is_legal_move::<White>(*mv)),
        Sides::Black => moves.retain(|mv| position.is_legal_move::<Black>(*mv)),
    }
}
