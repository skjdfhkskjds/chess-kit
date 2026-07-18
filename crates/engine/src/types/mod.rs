use std::time::Duration;

use chess_kit_primitives::{Move, SearchDepth};
use chess_kit_search::SearchResult;

/// `EngineConfig` contains construction-time settings for a composed engine session
///
/// @type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EngineConfig {
    pub transposition_table_size_mb: usize, // transposition table allocation in megabytes
}

impl EngineConfig {
    /// new creates an engine configuration
    ///
    /// @param: transposition_table_size_mb - transposition table allocation in megabytes
    /// @return: new engine configuration
    pub const fn new(transposition_table_size_mb: usize) -> Self {
        Self {
            transposition_table_size_mb,
        }
    }
}

/// `PositionBase` is the root position from which a move history is applied
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PositionBase {
    StartPos,
    Fen(String),
}

/// `SearchOutcome` is the result of a completed engine search
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchOutcome {
    pub best_move: Option<Move>, // best move found, or None when no legal move exists
    pub depth: SearchDepth,      // completed positive search depth in plies
    pub score: i32,              // position score in centipawns
    pub nodes: u64,              // number of nodes searched
    pub elapsed: Duration,       // elapsed search time
}

impl From<(SearchResult, SearchDepth, Duration)> for SearchOutcome {
    /// from enriches an internal search result with engine-boundary metadata.
    fn from((result, depth, elapsed): (SearchResult, SearchDepth, Duration)) -> Self {
        Self {
            best_move: result.best_move,
            depth,
            score: result.score,
            nodes: result.nodes,
            elapsed,
        }
    }
}
