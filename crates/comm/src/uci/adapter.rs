use chess_kit_engine::{
    AsyncEngine, EngineError, SearchLimits as EngineSearchLimits, SearchTaskId,
};
use chess_kit_primitives::{Move, SearchDepth};

use super::{PositionCommand, SearchLimits, SearchResult, UciEngine};

/// `UciAdapter` translates UCI protocol values to the protocol-neutral engine
/// boundary and translates engine search results back to UCI values.
///
/// The adapter lives in the communication crate so callers can use UCI without
/// implementing their own bridge to [`AsyncEngine`]
///
/// @marker: EngineT - protocol-neutral engine implementation
/// @type
pub struct UciAdapter<EngineT> {
    engine: EngineT,                     // engine receiving translated UCI operations
    default_search_depth: SearchDepth,   // depth used when a go command has no depth
    active_search: Option<SearchTaskId>, // search whose completion is expected
}

impl<EngineT> UciAdapter<EngineT> {
    /// new wraps an engine session for UCI presentation
    ///
    /// @param: engine - protocol-neutral engine session
    /// @param: default_search_depth - fallback depth for unconstrained searches
    /// @return: UCI adapter
    pub const fn new(engine: EngineT, default_search_depth: SearchDepth) -> Self {
        Self {
            engine,
            default_search_depth,
            active_search: None,
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
    EngineT: AsyncEngine,
{
    type Error = EngineError;
    type SearchTaskId = SearchTaskId;

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
        self.active_search = None;
        self.engine.new_game()
    }

    /// @impl: UciEngine::set_position
    fn set_position(&mut self, command: &PositionCommand) -> Result<(), Self::Error> {
        let moves = command
            .moves
            .iter()
            .map(Move::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| EngineError::new(error.to_string()))?;
        self.active_search = None;
        self.engine.set_position(command.base.clone(), &moves)
    }

    /// @impl: UciEngine::start_search
    fn start_search(&mut self, limits: &SearchLimits) -> Result<Self::SearchTaskId, Self::Error> {
        let has_clock = limits.move_time.is_some()
            || (limits.white_time.is_some() && limits.black_time.is_some());
        let maximum_depth = limits.depth.unwrap_or(if has_clock || limits.infinite {
            SearchDepth::MAX
        } else {
            self.default_search_depth
        });
        let engine_limits = EngineSearchLimits {
            maximum_depth,
            move_time: limits.move_time,
            white_time: limits.white_time,
            black_time: limits.black_time,
            white_increment: limits.white_increment,
            black_increment: limits.black_increment,
            moves_to_go: limits.moves_to_go,
        };
        let task_id = self.engine.start_search(engine_limits)?;
        self.active_search = Some(task_id);
        Ok(task_id)
    }

    /// @impl: UciEngine::poll_search
    fn poll_search(&mut self) -> Result<Option<SearchResult>, Self::Error> {
        loop {
            let Some(completion) = self.engine.try_search_completion()? else {
                return Ok(None);
            };
            if self.active_search != Some(completion.task_id) {
                continue;
            }

            self.active_search = None;
            return completion
                .result
                .map(|outcome| Some(SearchResult::from(outcome)));
        }
    }

    /// @impl: UciEngine::stop_search
    fn stop_search(&mut self, task_id: Self::SearchTaskId) -> Result<bool, Self::Error> {
        if self.active_search != Some(task_id) {
            return Ok(false);
        }
        self.engine.stop_search(task_id)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::time::Duration;

    use chess_kit_engine::{PositionBase, SearchCompletion, SearchOutcome};
    use chess_kit_primitives::{Pieces, Square};

    use super::*;
    use crate::uci::UciMove;

    struct TestEngine {
        positions: Vec<(PositionBase, Vec<Move>)>,
        started_limits: Vec<EngineSearchLimits>,
        completions: VecDeque<SearchCompletion>,
        next_task_id: u64,
        complete_on_start: bool,
    }

    impl Default for TestEngine {
        fn default() -> Self {
            Self {
                positions: Vec::new(),
                started_limits: Vec::new(),
                completions: VecDeque::new(),
                next_task_id: 0,
                complete_on_start: true,
            }
        }
    }

    fn completion(task_id: SearchTaskId, depth: SearchDepth) -> SearchCompletion {
        SearchCompletion {
            task_id,
            result: Ok(SearchOutcome {
                best_move: Some(Move::new(Square::A7, Square::A8).with_promotion(Pieces::Queen)),
                depth,
                score: 15,
                nodes: 23,
                elapsed: Duration::from_millis(4),
            }),
        }
    }

    impl AsyncEngine for TestEngine {
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

        fn start_search(
            &mut self,
            limits: EngineSearchLimits,
        ) -> Result<SearchTaskId, EngineError> {
            self.next_task_id += 1;
            let task_id = SearchTaskId::new(self.next_task_id);
            let depth = limits.maximum_depth;
            self.started_limits.push(limits);
            if self.complete_on_start {
                self.completions.push_back(completion(task_id, depth));
            }
            Ok(task_id)
        }

        fn try_search_completion(&mut self) -> Result<Option<SearchCompletion>, EngineError> {
            Ok(self.completions.pop_front())
        }

        fn stop_search(&mut self, _task_id: SearchTaskId) -> Result<bool, EngineError> {
            Ok(true)
        }
    }

    #[test]
    fn converts_uci_commands_and_engine_results_at_the_adapter_boundary() {
        let engine = TestEngine::default();
        let mut adapter = UciAdapter::new(engine, SearchDepth::new(4).unwrap());
        let position = PositionCommand {
            base: PositionBase::StartPos,
            moves: vec!["e2e4".parse().unwrap()],
        };

        adapter.set_position(&position).unwrap();
        adapter
            .start_search(&SearchLimits {
                depth: Some(SearchDepth::new(3).unwrap()),
                ..SearchLimits::default()
            })
            .unwrap();
        let result = adapter.poll_search().unwrap().unwrap();

        assert_eq!(
            UciMove::from(adapter.engine().positions[0].1[0]).to_string(),
            "e2e4"
        );
        assert_eq!(result.best_move.unwrap().to_string(), "a7a8q");
        assert_eq!(result.info.depth.map(SearchDepth::get), Some(3));

        adapter.start_search(&SearchLimits::default()).unwrap();
        let default_result = adapter.poll_search().unwrap().unwrap();
        assert_eq!(default_result.info.depth.map(SearchDepth::get), Some(4));
    }

    #[test]
    fn time_controls_search_to_the_engines_maximum_supported_depth() {
        let engine = TestEngine::default();
        let mut adapter = UciAdapter::new(engine, SearchDepth::new(4).unwrap());
        adapter
            .start_search(&SearchLimits {
                move_time: Some(Duration::from_millis(50)),
                ..SearchLimits::default()
            })
            .unwrap();
        let result = adapter.poll_search().unwrap().unwrap();

        assert_eq!(result.info.depth, Some(SearchDepth::MAX));
    }

    #[test]
    fn infinite_search_uses_the_engines_maximum_supported_depth() {
        let engine = TestEngine::default();
        let mut adapter = UciAdapter::new(engine, SearchDepth::new(4).unwrap());
        adapter
            .start_search(&SearchLimits {
                infinite: true,
                ..SearchLimits::default()
            })
            .unwrap();
        let result = adapter.poll_search().unwrap().unwrap();

        assert_eq!(result.info.depth, Some(SearchDepth::MAX));
    }

    #[test]
    fn stopped_search_completion_cannot_overtake_a_new_position_search() {
        let engine = TestEngine {
            complete_on_start: false,
            ..TestEngine::default()
        };
        let mut adapter = UciAdapter::new(engine, SearchDepth::new(4).unwrap());
        let old_task = adapter.start_search(&SearchLimits::default()).unwrap();
        assert!(adapter.stop_search(old_task).unwrap());
        adapter
            .set_position(&PositionCommand {
                base: PositionBase::StartPos,
                moves: Vec::new(),
            })
            .unwrap();

        adapter
            .engine_mut()
            .completions
            .push_back(completion(old_task, SearchDepth::new(1).unwrap()));
        adapter.engine_mut().complete_on_start = true;
        adapter
            .start_search(&SearchLimits {
                depth: Some(SearchDepth::new(2).unwrap()),
                ..SearchLimits::default()
            })
            .unwrap();

        let result = adapter.poll_search().unwrap().unwrap();
        assert_eq!(result.info.depth.map(SearchDepth::get), Some(2));
        assert!(adapter.poll_search().unwrap().is_none());
    }
}
