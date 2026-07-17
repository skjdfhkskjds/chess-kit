use std::str::FromStr;

use chess_kit::comm::uci::{
    BasePosition, PositionCommand, SearchInfo, SearchLimits, SearchResult, UciEngine, UciMove,
};
use chess_kit::engine::{
    DefaultEngine, Engine, PositionBase, SearchRequest, format_uci_move,
};

/// `UciAdapter` translates UCI protocol types into the internal engine API
///
/// This type owns only presentation concerns for the UCI wire format: mapping
/// `PositionCommand` / `SearchLimits` into engine calls and mapping search
/// outcomes back into UCI `SearchResult` values
///
/// @type
pub struct UciAdapter {
    engine: DefaultEngine,
}

impl UciAdapter {
    /// new wraps an engine session for UCI presentation
    ///
    /// @param: engine - protocol-agnostic engine session
    /// @return: UCI adapter
    pub fn new(engine: DefaultEngine) -> Self {
        Self { engine }
    }
}

impl UciEngine for UciAdapter {
    type Error = chess_kit::engine::EngineError;

    fn name(&self) -> &str {
        self.engine.name()
    }

    fn author(&self) -> &str {
        self.engine.author()
    }

    fn new_game(&mut self) -> Result<(), Self::Error> {
        self.engine.new_game()
    }

    fn set_position(&mut self, command: &PositionCommand) -> Result<(), Self::Error> {
        let base = match &command.base {
            BasePosition::StartPos => PositionBase::StartPos,
            BasePosition::Fen(fen) => PositionBase::Fen(fen.clone()),
        };
        let moves: Vec<&str> = command.moves.iter().map(UciMove::as_str).collect();
        self.engine.set_position(base, &moves)
    }

    fn search(&mut self, limits: &SearchLimits) -> Result<SearchResult, Self::Error> {
        let outcome = self.engine.search_with(SearchRequest { depth: limits.depth })?;
        let mut result = SearchResult::new(
            outcome
                .best_move
                .map(format_uci_move)
                .map(|mv| UciMove::from_str(&mv).expect("engine moves have valid UCI notation")),
        );
        result.info = SearchInfo {
            depth: Some(outcome.depth),
            score_cp: Some(outcome.score),
            nodes: Some(outcome.nodes),
            elapsed: Some(outcome.elapsed),
        };
        Ok(result)
    }
}
