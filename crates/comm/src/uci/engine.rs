use std::fmt::Display;
use std::time::Duration;

use chess_kit_engine::SearchOutcome;
use chess_kit_primitives::SearchDepth;

use super::{PositionCommand, SearchLimits, UciMove};

/// `SearchInfo` is a type that represents optional UCI search information
/// emitted immediately before `bestmove`
///
/// @type
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SearchInfo {
    pub depth: Option<SearchDepth>, // completed positive search depth in plies
    pub score_cp: Option<i32>,      // position score in centipawns
    pub nodes: Option<u64>,         // number of nodes searched
    pub elapsed: Option<Duration>,  // elapsed search time
}

/// `SearchResult` is a type that represents the result of a completed UCI search
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchResult {
    pub best_move: Option<UciMove>, // best move found, or None when no legal move exists
    pub ponder: Option<UciMove>,    // expected reply to search while pondering
    pub info: SearchInfo,           // optional information describing the search
}

impl SearchResult {
    /// new creates a search result for the given best move
    ///
    /// @param: best_move - best legal move found by the engine, or None if no
    ///                     legal move exists
    /// @return: new search result with no ponder move or search information
    pub fn new(best_move: Option<UciMove>) -> Self {
        Self {
            best_move,
            ponder: None,
            info: SearchInfo::default(),
        }
    }
}

impl From<&SearchOutcome> for SearchInfo {
    /// from translates protocol-neutral engine metrics into UCI information.
    fn from(outcome: &SearchOutcome) -> Self {
        Self {
            depth: Some(outcome.depth),
            score_cp: Some(outcome.score),
            nodes: Some(outcome.nodes),
            elapsed: Some(outcome.elapsed),
        }
    }
}

impl From<SearchOutcome> for SearchResult {
    /// from translates a completed engine search into a UCI result.
    fn from(outcome: SearchOutcome) -> Self {
        Self {
            best_move: outcome.best_move.map(UciMove::from),
            ponder: None,
            info: SearchInfo::from(&outcome),
        }
    }
}

/// `UciEngine` is the backend contract used by the UCI command loop.
///
/// Most callers should wrap their protocol-neutral engine in [`super::UciAdapter`]
/// instead of implementing this protocol-specific trait themselves.
///
/// @trait
pub trait UciEngine {
    /// Error is the displayable error returned by fallible engine operations
    type Error: Display;

    /// name returns the name advertised by the engine during UCI initialization
    ///
    /// @return: engine name
    fn name(&self) -> &str;

    /// author returns the author advertised by the engine during UCI initialization
    ///
    /// @return: engine author
    fn author(&self) -> &str;

    /// new_game notifies the engine that the next position belongs to a new game
    ///
    /// @return: Ok on success, or the engine error
    /// @side-effects: resets engine state associated with the previous game
    fn new_game(&mut self) -> Result<(), Self::Error>;

    /// set_position replaces the current engine position
    ///
    /// @param: position - base position and move history to apply
    /// @return: Ok on success, or the engine error
    /// @side-effects: modifies the current engine position
    fn set_position(&mut self, position: &PositionCommand) -> Result<(), Self::Error>;

    /// search searches the current position using the given limits
    ///
    /// @param: limits - constraints to apply to the search
    /// @return: completed search result, or the engine error
    /// @side-effects: may modify engine search state
    fn search(&mut self, limits: &SearchLimits) -> Result<SearchResult, Self::Error>;

    /// stop stops an active search
    ///
    /// note: synchronous engines may keep the default no-op implementation
    ///
    /// @return: final search result if one is available, or the engine error
    /// @side-effects: may stop or finalize an active search
    fn stop(&mut self) -> Result<Option<SearchResult>, Self::Error> {
        Ok(None)
    }

    /// ponder_hit notifies a pondering engine that its expected move was played
    ///
    /// @return: void
    /// @side-effects: may convert a ponder search into a normal search
    fn ponder_hit(&mut self) {}

    /// set_debug enables or disables optional diagnostic output
    ///
    /// @param: enabled - whether diagnostic output should be enabled
    /// @return: void
    /// @side-effects: modifies the engine's diagnostic output mode
    fn set_debug(&mut self, _enabled: bool) {}
}
