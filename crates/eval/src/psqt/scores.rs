use super::PSQTEvalState;
use super::constants::{MIDDLEGAME_PHASE_MAX, MIDDLEGAME_PHASE_MIN};
use crate::Score;
use chess_kit_primitives::Sides;

impl PSQTEvalState {
    const MIDDLEGAME_PHASE_DENOM: f32 = (MIDDLEGAME_PHASE_MAX - MIDDLEGAME_PHASE_MIN) as f32;

    /// weighted_phase calculates the weighted phase value based on the current
    /// phase value
    ///
    /// @return: weighted phase value
    /// @side-effects: modifies the eval state
    #[inline(always)]
    pub(super) fn weighted_phase(&self) -> f32 {
        // interpolate the phase value between the min and max
        let result = (self.phase - MIDDLEGAME_PHASE_MIN) as f32 / Self::MIDDLEGAME_PHASE_DENOM;

        // clamp the result to the range [0.0, 1.0]
        //
        // note: this is required to prevent overflow/underflow when calculating
        //       the score using the weighted phase value
        result.clamp(0.0, 1.0)
    }

    /// update_score updates the score based on the current weighted phase value
    /// and the accumulated scores
    ///
    /// @return: void
    /// @side-effects: modifies the eval state
    #[inline(always)]
    pub(super) fn update_score(&mut self) {
        // calculate the middlegame score for both sides
        let mid_score = (self.scores[Sides::White].middlegame()
            + self.scores[Sides::Black].middlegame()) as f32;

        // calculate the endgame score for both sides
        let end_score =
            (self.scores[Sides::White].endgame() + self.scores[Sides::Black].endgame()) as f32;

        // interpolate the score between the middlegame and endgame scores based
        // on the weighted phase value
        let weighted_phase = self.weighted_phase();
        self.score =
            (mid_score * weighted_phase + end_score * (1.0 - weighted_phase)).round() as Score;
    }
}
