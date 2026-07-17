use std::time::Duration;

use chess_kit_primitives::Move;

/// Default search depth when a caller does not request one
pub const DEFAULT_SEARCH_DEPTH: u8 = 4;

/// Maximum search depth supported by the composed engine
pub const MAX_SEARCH_DEPTH: u8 = 8;

/// Default transposition table size in megabytes
pub const DEFAULT_TRANSPOSITION_TABLE_SIZE_MB: usize = 1024;

/// `PositionBase` is the root position from which a move history is applied
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PositionBase {
    StartPos,
    Fen(String),
}

/// `SearchRequest` is the protocol-agnostic search constraint set
///
/// @type
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SearchRequest {
    pub depth: Option<u8>,
}

impl SearchRequest {
    /// with_depth creates a search request for a fixed depth
    ///
    /// @param: depth - requested search depth in plies
    /// @return: new search request
    pub const fn with_depth(depth: u8) -> Self {
        Self { depth: Some(depth) }
    }
}

/// `SearchOutcome` is the result of a completed engine search
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchOutcome {
    pub best_move: Option<Move>, // best move found, or None when no legal move exists
    pub depth: u8,               // completed search depth in plies
    pub score: i32,              // position score in centipawns
    pub nodes: u64,              // number of nodes searched
    pub elapsed: Duration,       // elapsed search time
}
