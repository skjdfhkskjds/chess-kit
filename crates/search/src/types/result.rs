use chess_kit_eval::Score;
use chess_kit_primitives::{Depth, Move};

/// The result of a completed search.
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

/// `IterativeSearchResult` couples a search result with its last fully
/// completed depth.
///
/// @type
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct IterativeSearchResult {
    pub result: SearchResult,
    pub depth: Depth,
}

impl IterativeSearchResult {
    /// `new` creates an iterative result at a completed depth.
    ///
    /// @param: result - result from the last fully completed iteration
    /// @param: depth - fully completed depth in plies
    /// @return: iterative search result
    pub const fn new(result: SearchResult, depth: Depth) -> Self {
        Self { result, depth }
    }
}
