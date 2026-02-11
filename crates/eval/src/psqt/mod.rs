mod constants;
mod piece_values;
mod scores;

use constants::{PHASE_VALUES, PIECE_TABLES};
use piece_values::PieceValue;

use crate::{EvalState, Score};
use chess_kit_collections::Copyable;
use chess_kit_primitives::{Pieces, Side, Sides, Square};

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
    /// new creates a new, empty eval state
    ///
    /// @impl: EvalState::new
    #[inline(always)]
    fn new() -> Self {
        Self {
            phase: 0,
            scores: [PieceValue::default(); Sides::TOTAL],
            score: 0,
        }
    }

    /// score returns the evaluation score of this state
    ///
    /// @impl: EvalState::score
    #[inline(always)]
    fn score(&mut self) -> Score {
        // TODO: figure out a way to implement at-most-once semantics for score
        //       computation while preserving lazy pop/clear
        self.update_score();

        self.score
    }

    /// on_set_piece is the incremental update callback that fires when a piece
    /// is set on the board for the given side
    ///
    /// @impl: EvalState::on_set_piece
    #[inline(always)]
    fn on_set_piece<SideT: Side>(&mut self, piece: Pieces, square: Square) {
        self.phase += PHASE_VALUES[piece];
        self.scores[SideT::SIDE] += PIECE_TABLES[SideT::SIDE][piece][square];
    }

    /// on_remove_piece is the incremental update callback that fires when a piece
    /// is removed from the board for the given side
    ///
    /// @impl: EvalState::on_remove_piece
    #[inline(always)]
    fn on_remove_piece<SideT: Side>(&mut self, piece: Pieces, square: Square) {
        self.phase -= PHASE_VALUES[piece];
        self.scores[SideT::SIDE] -= PIECE_TABLES[SideT::SIDE][piece][square];
    }
}

impl Copyable for PSQTEvalState {
    /// copy_from copies the contents of another PSQTEvalState into this one
    ///
    /// @impl: Copyable::copy_from
    #[inline(always)]
    fn copy_from(&mut self, other: &Self) {
        *self = *other;
    }
}

impl Default for PSQTEvalState {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
