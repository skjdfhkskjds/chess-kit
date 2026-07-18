use chess_kit_engine::{Engine, EngineError, PositionBase};
use chess_kit_primitives::{Depth, Move};

use super::{
    BasePosition, PositionCommand, SearchInfo, SearchLimits, SearchResult, UciEngine, UciMove,
};

/// `UciAdapter` translates UCI protocol values to the protocol-neutral engine
/// boundary and translates engine search results back to UCI values.
///
/// The adapter lives in the communication crate so callers can use UCI without
/// implementing their own bridge to [`Engine`]
///
/// @marker: EngineT - protocol-neutral engine implementation
/// @type
pub struct UciAdapter<EngineT> {
    engine: EngineT,             // engine receiving translated UCI operations
    default_search_depth: Depth, // depth used when a go command has no depth
}

impl<EngineT> UciAdapter<EngineT> {
    /// new wraps an engine session for UCI presentation
    ///
    /// @param: engine - protocol-neutral engine session
    /// @param: default_search_depth - fallback depth for unconstrained searches
    /// @return: UCI adapter
    pub const fn new(engine: EngineT, default_search_depth: Depth) -> Self {
        Self {
            engine,
            default_search_depth,
        }
    }

    /// engine returns a shared reference to the wrapped engine
    ///
    /// @return: shared reference to the wrapped engine
    pub const fn engine(&self) -> &EngineT {
        &self.engine
    }

    /// engine_mut returns a mutable reference to the wrapped engine
    ///
    /// @return: mutable reference to the wrapped engine
    pub const fn engine_mut(&mut self) -> &mut EngineT {
        &mut self.engine
    }

    /// into_inner consumes the adapter and returns the wrapped engine
    ///
    /// @return: wrapped engine
    pub fn into_inner(self) -> EngineT {
        self.engine
    }
}

impl<EngineT> UciEngine for UciAdapter<EngineT>
where
    EngineT: Engine,
{
    type Error = EngineError;

    /// @impl: UciEngine::name
    fn name(&self) -> &str {
        self.engine.name()
    }

    /// @impl: UciEngine::author
    fn author(&self) -> &str {
        self.engine.author()
    }

    /// @impl: UciEngine::new_game
    fn new_game(&mut self) -> Result<(), Self::Error> {
        self.engine.new_game()
    }

    /// @impl: UciEngine::set_position
    fn set_position(&mut self, command: &PositionCommand) -> Result<(), Self::Error> {
        let base = match &command.base {
            BasePosition::StartPos => PositionBase::StartPos,
            BasePosition::Fen(fen) => PositionBase::Fen(fen.clone()),
        };
        let moves = command
            .moves
            .iter()
            .map(Move::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| EngineError::new(error.to_string()))?;
        self.engine.set_position(base, &moves)
    }

    /// @impl: UciEngine::search
    fn search(&mut self, limits: &SearchLimits) -> Result<SearchResult, Self::Error> {
        let outcome = self
            .engine
            .search(limits.depth.unwrap_or(self.default_search_depth))?;
        let mut result = SearchResult::new(outcome.best_move.map(UciMove::from));
        result.info = SearchInfo {
            depth: Some(outcome.depth),
            score_cp: Some(outcome.score),
            nodes: Some(outcome.nodes),
            elapsed: Some(outcome.elapsed),
        };
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use chess_kit_engine::SearchOutcome;
    use chess_kit_primitives::{Pieces, Square};

    use super::*;

    #[derive(Default)]
    struct TestEngine {
        positions: Vec<(PositionBase, Vec<Move>)>,
    }

    impl Engine for TestEngine {
        fn name(&self) -> &str {
            "Adapter Test"
        }

        fn author(&self) -> &str {
            "Test Author"
        }

        fn new_game(&mut self) -> Result<(), EngineError> {
            Ok(())
        }

        fn set_position(&mut self, base: PositionBase, moves: &[Move]) -> Result<(), EngineError> {
            self.positions.push((base, moves.to_vec()));
            Ok(())
        }

        fn play(&mut self, _mv: Move) -> Result<(), EngineError> {
            Ok(())
        }

        fn search(&mut self, depth: Depth) -> Result<SearchOutcome, EngineError> {
            Ok(SearchOutcome {
                best_move: Some(Move::new(Square::A7, Square::A8).with_promotion(Pieces::Queen)),
                depth,
                score: 15,
                nodes: 23,
                elapsed: Duration::from_millis(4),
            })
        }

        fn has_legal_moves(&self) -> bool {
            true
        }
    }

    #[test]
    fn converts_uci_commands_and_engine_results_at_the_adapter_boundary() {
        let engine = TestEngine::default();
        let mut adapter = UciAdapter::new(engine, 4);
        let position = PositionCommand {
            base: BasePosition::StartPos,
            moves: vec!["e2e4".parse().unwrap()],
        };

        adapter.set_position(&position).unwrap();
        let result = adapter
            .search(&SearchLimits {
                depth: Some(3),
                ..SearchLimits::default()
            })
            .unwrap();

        assert_eq!(
            UciMove::from(adapter.engine().positions[0].1[0]).as_str(),
            "e2e4"
        );
        assert_eq!(result.best_move.unwrap().as_str(), "a7a8q");
        assert_eq!(result.info.depth, Some(3));

        let default_result = adapter.search(&SearchLimits::default()).unwrap();
        assert_eq!(default_result.info.depth, Some(4));
    }
}
