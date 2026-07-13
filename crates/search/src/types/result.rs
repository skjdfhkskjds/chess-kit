use chess_kit_eval::Score;
use chess_kit_primitives::Move;

/// The result of a completed fixed-depth search.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SearchResult {
    pub best_move: Option<Move>,
    pub score: Score,
    pub nodes: u64,
}

impl SearchResult {
    pub const fn new(best_move: Option<Move>, score: Score, nodes: u64) -> Self {
        Self {
            best_move,
            score,
            nodes,
        }
    }
}
