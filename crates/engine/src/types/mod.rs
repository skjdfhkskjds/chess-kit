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

/// `SearchLimits` describes the depth and clock constraints for one engine
/// search.
///
/// A clock-limited search normally uses [`SearchDepth::MAX`] as its maximum
/// depth. Supplying a lower value combines both constraints and stops at
/// whichever limit is reached first.
///
/// @type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SearchLimits {
    pub maximum_depth: SearchDepth,
    pub move_time: Option<Duration>,
    pub white_time: Option<Duration>,
    pub black_time: Option<Duration>,
    pub white_increment: Option<Duration>,
    pub black_increment: Option<Duration>,
    pub moves_to_go: Option<u32>,
}

impl SearchLimits {
    /// `depth` creates an untimed fixed-maximum-depth search.
    ///
    /// @param: maximum_depth - greatest depth to search in plies
    /// @return: fixed-maximum-depth search limits
    pub const fn depth(maximum_depth: SearchDepth) -> Self {
        Self {
            maximum_depth,
            move_time: None,
            white_time: None,
            black_time: None,
            white_increment: None,
            black_increment: None,
            moves_to_go: None,
        }
    }

    /// `move_time` creates a search with a fixed time allocation and the
    /// greatest supported depth.
    ///
    /// @param: move_time - time allocated to this move
    /// @return: time-limited search limits
    pub const fn move_time(move_time: Duration) -> Self {
        Self {
            maximum_depth: SearchDepth::MAX,
            move_time: Some(move_time),
            white_time: None,
            black_time: None,
            white_increment: None,
            black_increment: None,
            moves_to_go: None,
        }
    }
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
