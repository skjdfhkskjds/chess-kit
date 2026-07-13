use std::fmt::Display;
use std::time::Duration;

use super::{PositionCommand, SearchLimits, UciMove};

/// Search information emitted immediately before `bestmove`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SearchInfo {
    pub depth: Option<u8>,
    pub score_cp: Option<i32>,
    pub nodes: Option<u64>,
    pub elapsed: Option<Duration>,
}

/// The result of a completed UCI search.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchResult {
    pub best_move: Option<UciMove>,
    pub ponder: Option<UciMove>,
    pub info: SearchInfo,
}

impl SearchResult {
    pub fn new(best_move: Option<UciMove>) -> Self {
        Self {
            best_move,
            ponder: None,
            info: SearchInfo::default(),
        }
    }
}

/// Adapter implemented by a chess engine that wants to speak UCI.
pub trait UciEngine {
    type Error: Display;

    fn name(&self) -> &str;
    fn author(&self) -> &str;
    fn new_game(&mut self) -> Result<(), Self::Error>;
    fn set_position(&mut self, position: &PositionCommand) -> Result<(), Self::Error>;
    fn search(&mut self, limits: &SearchLimits) -> Result<SearchResult, Self::Error>;

    /// Stops an active search. Synchronous engines may keep the default no-op.
    fn stop(&mut self) -> Result<Option<SearchResult>, Self::Error> {
        Ok(None)
    }

    /// Notifies a pondering engine that its expected move was played.
    fn ponder_hit(&mut self) {}

    /// Enables or disables optional diagnostic output.
    fn set_debug(&mut self, _enabled: bool) {}
}
