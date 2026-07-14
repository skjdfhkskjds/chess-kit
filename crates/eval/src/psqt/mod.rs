mod constants;
mod piece_values;
mod scores;

use constants::{PHASE_VALUES, PIECE_TABLES};
use piece_values::PieceValue;

use crate::{EvalState, Score};
use chess_kit_collections::Copyable;
use chess_kit_position::PositionView;
use chess_kit_primitives::{MoveDelta, PieceDeltaKind, Pieces, Sides, Square};

pub type GamePhase = i16;
pub type PSQTable = [PieceValue; Square::TOTAL];

/// `PSQTEvalState` is the evaluation state implementation for PSQT-based
/// evaluation
///
/// @type
#[derive(Copy, Clone)]
pub struct PSQTEvalState {
    phase: GamePhase,                   // incrementally updated game phase value
    scores: [PieceValue; Sides::TOTAL], // accumulated scores per side

    score: Score, // computed score of the position
}

impl EvalState for PSQTEvalState {
    /// Initializes the PSQT accumulator from a position.
    #[inline]
    fn from_position<P: PositionView>(position: &P) -> Self {
        let mut state = Self {
            phase: 0,
            scores: [PieceValue::default(); Sides::TOTAL],
            score: 0,
        };

        for piece in Pieces::ALL {
            for square in position.get_piece::<chess_kit_primitives::White>(piece) {
                state.add(Sides::White, piece, square);
            }
            for square in position.get_piece::<chess_kit_primitives::Black>(piece) {
                state.add(Sides::Black, piece, square);
            }
        }

        state
    }

    #[inline]
    fn apply(&mut self, delta: MoveDelta) {
        for change in delta.iter() {
            match change.kind() {
                PieceDeltaKind::Added => self.add(change.side(), change.piece(), change.square()),
                PieceDeltaKind::Removed => {
                    self.remove(change.side(), change.piece(), change.square())
                }
            }
        }
    }

    /// score returns the evaluation score of this state
    ///
    /// @impl: EvalState::score
    #[inline]
    fn score(&mut self) -> Score {
        // TODO: figure out a way to implement at-most-once semantics for score
        //       computation while preserving lazy pop/clear
        self.update_score();

        self.score
    }
}

impl PSQTEvalState {
    #[inline]
    fn add(&mut self, side: Sides, piece: Pieces, square: Square) {
        self.phase += PHASE_VALUES[piece];
        self.scores[side] += PIECE_TABLES[side][piece][square];
    }

    #[inline]
    fn remove(&mut self, side: Sides, piece: Pieces, square: Square) {
        self.phase -= PHASE_VALUES[piece];
        self.scores[side] -= PIECE_TABLES[side][piece][square];
    }
}

impl Copyable for PSQTEvalState {
    /// copy_from copies the contents of another PSQTEvalState into this one
    ///
    /// @impl: Copyable::copy_from
    #[inline]
    fn copy_from(&mut self, other: &Self) {
        *self = *other;
    }
}

impl Default for PSQTEvalState {
    #[inline]
    fn default() -> Self {
        Self {
            phase: 0,
            scores: [PieceValue::default(); Sides::TOTAL],
            score: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chess_kit_attack_table::DefaultAttackTable;
    use chess_kit_position::{DefaultPosition, PositionMoves};
    use chess_kit_primitives::{Move, Square};

    #[test]
    fn incremental_deltas_match_fresh_position_initialization() {
        let mut position = DefaultPosition::<DefaultAttackTable>::default();
        let mut incremental = PSQTEvalState::from_position(&position);

        for mv in [
            Move::new(Square::E2, Square::E4),
            Move::new(Square::D7, Square::D5),
            Move::new(Square::E4, Square::D5),
        ] {
            incremental.apply(position.play_unchecked(mv));
            let fresh = PSQTEvalState::from_position(&position);

            assert_eq!(incremental.phase, fresh.phase);
            for side in [Sides::White, Sides::Black] {
                assert_eq!(
                    incremental.scores[side].middlegame(),
                    fresh.scores[side].middlegame()
                );
                assert_eq!(
                    incremental.scores[side].endgame(),
                    fresh.scores[side].endgame()
                );
            }
        }
    }
}
