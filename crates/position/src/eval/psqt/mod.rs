mod constants;
mod piece_values;
mod scores;

use constants::{PHASE_VALUES, PIECE_TABLES};
use piece_values::PieceValue;

use crate::eval::{EvalState, Score};
use crate::Position;
use chess_kit_primitives::{Black, Pieces, Side, Sides, Square, White};
use chess_kit_collections::Copyable;

pub type GamePhase = i16;
pub type PSQTable = [PieceValue; Square::TOTAL];

// PSQTEval is the evaluation state for PSQT-based evaluation
#[derive(Copy, Clone)]
pub struct PSQTEvalState {
    phase: GamePhase,                   // incrementally updated game phase value
    scores: [PieceValue; Sides::TOTAL], // accumulated scores per side

    score: Score, // computed score of the position
}

impl EvalState for PSQTEvalState {
    // new creates a new, empty eval state
    //
    // @impl: EvalState::new
    #[inline(always)]
    fn new() -> Self {
        Self {
            phase: 0,
            scores: [PieceValue::default(); Sides::TOTAL],
            score: 0,
        }
    }

    // init initializes the eval state for based on the given position
    //
    // @impl: EvalState::init
    #[inline(always)]
    fn init<PositionT: Position>(&mut self, position: &PositionT) {
        for piece in Pieces::ALL {
            // initialize the phase and scores for the white side's pieces
            let white = position.get_piece::<White>(piece);
            for sq in white.iter() {
                self.on_set_piece::<White>(piece, sq);
            }

            // initialize the phase and scores for the black side's pieces
            let black = position.get_piece::<Black>(piece);
            for sq in black.iter() {
                self.on_set_piece::<Black>(piece, sq);
            }
        }
    }

    // score returns the evaluation score of this state
    //
    // @impl: EvalState::score
    #[inline(always)]
    fn score(&mut self) -> Score {
        // TODO: figure out a way to implement at-most-once semantics for score
        //       computation while preserving lazy pop/clear
        self.update_score();

        self.score
    }

    // on_set_piece is the incremental update callback that fires when a piece
    // is set on the board for the given side
    //
    // @impl: EvalState::on_set_piece
    #[inline(always)]
    fn on_set_piece<SideT: Side>(&mut self, piece: Pieces, square: Square) {
        self.phase += PHASE_VALUES[piece];
        self.scores[SideT::INDEX] += PIECE_TABLES[SideT::INDEX][piece][square];
    }

    // on_remove_piece is the incremental update callback that fires when a piece
    // is removed from the board for the given side
    //
    // @impl: EvalState::on_remove_piece
    #[inline(always)]
    fn on_remove_piece<SideT: Side>(&mut self, piece: Pieces, square: Square) {
        self.phase -= PHASE_VALUES[piece];
        self.scores[SideT::INDEX] -= PIECE_TABLES[SideT::INDEX][piece][square];
    }
}

impl Copyable for PSQTEvalState {
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
