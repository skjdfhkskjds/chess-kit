use std::fmt::{self, Display};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread::{self, JoinHandle};

use chess_kit_primitives::Move;
use chess_kit_search::{SearchCancellation, SearchControl};

use crate::{ControllableEngine, EngineError, PositionBase, SearchLimits, SearchOutcome};

/// NEXT_ENGINE_SCOPE supplies process-local scopes for threaded task IDs.
static NEXT_ENGINE_SCOPE: AtomicU64 = AtomicU64::new(1);

/// `SearchTaskId` uniquely identifies a search submitted to a threaded engine.
///
/// IDs are local to the asynchronous engine that issued them. The distinct
/// type prevents callers from accidentally substituting an unrelated integer;
/// callers must still return an identifier to its originating engine.
///
/// @type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SearchTaskId {
    scope: u64,
    sequence: u64,
}

impl SearchTaskId {
    /// new creates a task identifier for an asynchronous engine implementation.
    ///
    /// Threaded engine callers normally receive identifiers from
    /// [`AsyncEngine::start_search`]. This constructor lets other asynchronous
    /// engines and test doubles implement the same public contract.
    ///
    /// @param: value - identifier value scoped by the implementing engine
    /// @return: typed search task identifier
    pub const fn new(value: u64) -> Self {
        Self {
            scope: 0,
            sequence: value,
        }
    }

    /// get returns the task's numeric identifier.
    ///
    /// @return: task identifier local to one threaded engine
    pub const fn get(self) -> u64 {
        self.sequence
    }

    const fn scoped(scope: u64, sequence: u64) -> Self {
        Self { scope, sequence }
    }
}

impl Display for SearchTaskId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.scope == 0 {
            self.sequence.fmt(formatter)
        } else {
            write!(formatter, "{}:{}", self.scope, self.sequence)
        }
    }
}

/// `SearchCompletion` couples a finished search with the task that produced it.
///
/// A cancelled search may complete successfully with its last fully completed
/// iteration or return an engine error when no usable iteration completed.
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchCompletion {
    pub task_id: SearchTaskId,                      // completed search task
    pub result: Result<SearchOutcome, EngineError>, // search outcome or engine error
}

impl SearchCompletion {
    /// new creates a completion for a submitted search task.
    ///
    /// @param: task_id - task that produced the result
    /// @param: result - completed outcome or engine error
    /// @return: typed search completion
    const fn new(task_id: SearchTaskId, result: Result<SearchOutcome, EngineError>) -> Self {
        Self { task_id, result }
    }
}

/// `AsyncEngine` is the protocol-neutral asynchronous engine session surface.
///
/// State-changing operations remain synchronous so callers know when the
/// worker has applied them. Search is submitted asynchronously and completes
/// through [`SearchCompletion`] events.
///
/// @trait
pub trait AsyncEngine {
    /// name returns the engine's display name.
    ///
    /// @return: engine display name
    fn name(&self) -> &str;

    /// author returns the engine author's display name.
    ///
    /// @return: engine author display name
    fn author(&self) -> &str;

    /// new_game resets the worker-owned engine for a fresh game.
    ///
    /// An active search is cancelled and its completion discarded before the
    /// worker applies the state change.
    ///
    /// @return: Ok on success, or the engine error
    /// @side-effects: resets engine state on the worker thread
    fn new_game(&mut self) -> Result<(), EngineError>;

    /// set_position replaces the worker-owned engine position.
    ///
    /// An active search is cancelled and its completion discarded before the
    /// worker applies the state change.
    ///
    /// @param: base - root position before applying moves
    /// @param: moves - ordered engine moves to apply
    /// @return: Ok on success, or the engine error
    /// @side-effects: replaces engine state on the worker thread
    fn set_position(&mut self, base: PositionBase, moves: &[Move]) -> Result<(), EngineError>;

    /// start_search submits one search and returns immediately.
    ///
    /// @param: limits - depth and time constraints for the search
    /// @return: submitted task identifier, or the engine error
    /// @side-effects: queues search work on the worker thread
    fn start_search(&mut self, limits: SearchLimits) -> Result<SearchTaskId, EngineError>;

    /// try_search_completion returns the next completed search without blocking.
    ///
    /// @return: ready completion, None when searching, or the engine error
    /// @side-effects: consumes a ready completion event
    fn try_search_completion(&mut self) -> Result<Option<SearchCompletion>, EngineError>;

    /// discard_search cancels and consumes a matching active search.
    ///
    /// Unlike [`Self::stop_search`], discarding waits for the worker and removes
    /// the completion so it cannot be published later.
    ///
    /// @param: task_id - active task to cancel and discard
    /// @return: true when the matching task was discarded, or the engine error
    /// @side-effects: may block until cancellation completes and consumes its result
    fn discard_search(&mut self, task_id: SearchTaskId) -> Result<bool, EngineError>;

    /// stop_search requests cancellation for the matching active search.
    ///
    /// @param: task_id - active task to cancel
    /// @return: true when the task was active and received the stop request
    /// @side-effects: atomically requests cancellation of the matching task
    fn stop_search(&mut self, task_id: SearchTaskId) -> Result<bool, EngineError>;
}

/// `ThreadedEngine` owns an engine on one long-lived standard-library thread.
///
/// Commands are serialized through a typed channel. Cancellation deliberately
/// bypasses that channel because the worker cannot receive commands while it is
/// inside a search; each task instead owns an independent atomic cancellation
/// token. This first implementation accepts one active task at a time; the
/// task and completion boundary leaves scheduling policy private.
///
/// @marker: EngineT - controllable synchronous engine owned by the worker
/// @type
pub struct ThreadedEngine<EngineT>
where
    EngineT: ControllableEngine + Send + 'static,
{
    name: String,
    author: String,
    commands: Option<Sender<Command>>,
    completions: Receiver<SearchCompletion>,
    active: Option<ActiveSearch>,
    task_scope: u64,
    next_task_id: u64,
    worker: Option<JoinHandle<EngineT>>,
}

struct ActiveSearch {
    task_id: SearchTaskId,
    cancellation: SearchCancellation,
}

/// `SearchTask` contains all data needed to execute one scheduled search.
///
/// Keeping the task separate from the command transport leaves room for a
/// future scheduler to queue or distribute tasks without reshaping the engine
/// command protocol.
///
/// @type
struct SearchTask {
    task_id: SearchTaskId,
    limits: SearchLimits,
    control: SearchControl,
}

enum Command {
    NewGame(Sender<Result<(), EngineError>>),
    SetPosition {
        base: PositionBase,
        moves: Vec<Move>,
        reply: Sender<Result<(), EngineError>>,
    },
    Search(SearchTask),
    Shutdown,
}

impl<EngineT> ThreadedEngine<EngineT>
where
    EngineT: ControllableEngine + Send + 'static,
{
    /// new moves an engine onto a long-lived worker thread.
    ///
    /// @param: engine - synchronous engine session to own on the worker
    /// @return: threaded engine facade, or an engine error when spawning fails
    /// @side-effects: spawns one operating-system thread
    pub fn new(engine: EngineT) -> Result<Self, EngineError> {
        let task_scope = NEXT_ENGINE_SCOPE
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |scope| {
                scope.checked_add(1)
            })
            .map_err(|_| EngineError::new("engine search task scopes exhausted"))?;
        let name = engine.name().to_owned();
        let author = engine.author().to_owned();
        let (command_sender, command_receiver) = mpsc::channel();
        let (completion_sender, completions) = mpsc::channel();
        let worker = thread::Builder::new()
            .name("chess-kit-engine".to_owned())
            .spawn(move || run_worker(engine, command_receiver, completion_sender))
            .map_err(|error| EngineError::new(format!("failed to start engine thread: {error}")))?;

        Ok(Self {
            name,
            author,
            commands: Some(command_sender),
            completions,
            active: None,
            task_scope,
            next_task_id: 1,
            worker: Some(worker),
        })
    }

    /// shutdown stops an active search, terminates the worker, and returns the engine.
    ///
    /// Returning the engine makes the ownership transfer explicit and is useful
    /// for callers that need to inspect or reuse the synchronous session.
    ///
    /// @return: worker-owned engine, or an error when the worker panicked
    /// @side-effects: requests cancellation and joins the worker thread
    pub fn shutdown(mut self) -> Result<EngineT, EngineError> {
        self.cancel_active();
        self.send_shutdown();
        self.join_worker()
    }

    fn send(&self, command: Command) -> Result<(), EngineError> {
        self.commands
            .as_ref()
            .ok_or_else(worker_stopped)?
            .send(command)
            .map_err(|_| worker_stopped())
    }

    fn request(
        &self,
        make_command: impl FnOnce(Sender<Result<(), EngineError>>) -> Command,
    ) -> Result<(), EngineError> {
        let (reply_sender, reply_receiver) = mpsc::channel();
        self.send(make_command(reply_sender))?;
        reply_receiver.recv().map_err(|_| worker_stopped())?
    }

    fn cancel_active(&self) {
        if let Some(active) = &self.active {
            active.cancellation.cancel();
        }
    }

    fn finish_active_search(&mut self) -> Result<(), EngineError> {
        let Some(task_id) = self.active.as_ref().map(|active| active.task_id) else {
            return Ok(());
        };
        self.finish_search(task_id).map(|_| ())
    }

    fn finish_search(&mut self, task_id: SearchTaskId) -> Result<bool, EngineError> {
        let Some(active) = self.active.as_ref() else {
            return Ok(false);
        };
        if active.task_id != task_id {
            return Ok(false);
        }

        let active = self
            .active
            .take()
            .expect("active search was checked immediately before it was taken");
        active.cancellation.cancel();
        let completion = self.completions.recv().map_err(|_| worker_stopped())?;
        if completion.task_id != active.task_id {
            return Err(EngineError::new(format!(
                "engine worker completed unexpected search task {} while waiting for {}",
                completion.task_id, active.task_id
            )));
        }
        Ok(true)
    }

    fn send_shutdown(&mut self) {
        if let Some(commands) = self.commands.take() {
            let _ = commands.send(Command::Shutdown);
        }
    }

    fn join_worker(&mut self) -> Result<EngineT, EngineError> {
        self.worker
            .take()
            .ok_or_else(worker_stopped)?
            .join()
            .map_err(|_| EngineError::new("engine worker thread panicked"))
    }
}

impl<EngineT> AsyncEngine for ThreadedEngine<EngineT>
where
    EngineT: ControllableEngine + Send + 'static,
{
    /// @impl: AsyncEngine::name
    fn name(&self) -> &str {
        &self.name
    }

    /// @impl: AsyncEngine::author
    fn author(&self) -> &str {
        &self.author
    }

    /// @impl: AsyncEngine::new_game
    fn new_game(&mut self) -> Result<(), EngineError> {
        self.finish_active_search()?;
        self.request(Command::NewGame)
    }

    /// @impl: AsyncEngine::set_position
    fn set_position(&mut self, base: PositionBase, moves: &[Move]) -> Result<(), EngineError> {
        self.finish_active_search()?;
        self.request(|reply| Command::SetPosition {
            base,
            moves: moves.to_vec(),
            reply,
        })
    }

    /// @impl: AsyncEngine::start_search
    fn start_search(&mut self, limits: SearchLimits) -> Result<SearchTaskId, EngineError> {
        if self.active.is_some() {
            return Err(EngineError::new("an engine search is already active"));
        }

        let task_id = SearchTaskId::scoped(self.task_scope, self.next_task_id);
        let next_task_id = self
            .next_task_id
            .checked_add(1)
            .ok_or_else(|| EngineError::new("engine search task identifiers exhausted"))?;
        let cancellation = SearchCancellation::new();
        let control = SearchControl::with_cancellation(None, cancellation.clone());
        self.send(Command::Search(SearchTask {
            task_id,
            limits,
            control,
        }))?;

        self.next_task_id = next_task_id;
        self.active = Some(ActiveSearch {
            task_id,
            cancellation,
        });
        Ok(task_id)
    }

    /// @impl: AsyncEngine::try_search_completion
    fn try_search_completion(&mut self) -> Result<Option<SearchCompletion>, EngineError> {
        match self.completions.try_recv() {
            Ok(completion) => {
                if self
                    .active
                    .as_ref()
                    .is_some_and(|active| active.task_id == completion.task_id)
                {
                    self.active = None;
                }
                Ok(Some(completion))
            }
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(worker_stopped()),
        }
    }

    /// @impl: AsyncEngine::discard_search
    fn discard_search(&mut self, task_id: SearchTaskId) -> Result<bool, EngineError> {
        self.finish_search(task_id)
    }

    /// @impl: AsyncEngine::stop_search
    fn stop_search(&mut self, task_id: SearchTaskId) -> Result<bool, EngineError> {
        let Some(active) = &self.active else {
            return Ok(false);
        };
        if active.task_id != task_id {
            return Ok(false);
        }

        active.cancellation.cancel();
        Ok(true)
    }
}

impl<EngineT> Drop for ThreadedEngine<EngineT>
where
    EngineT: ControllableEngine + Send + 'static,
{
    fn drop(&mut self) {
        self.cancel_active();
        self.send_shutdown();
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

fn run_worker<EngineT>(
    mut engine: EngineT,
    commands: Receiver<Command>,
    completions: Sender<SearchCompletion>,
) -> EngineT
where
    EngineT: ControllableEngine,
{
    while let Ok(command) = commands.recv() {
        match command {
            Command::NewGame(reply) => {
                let _ = reply.send(engine.new_game());
            }
            Command::SetPosition { base, moves, reply } => {
                let _ = reply.send(engine.set_position(base, &moves));
            }
            Command::Search(SearchTask {
                task_id,
                limits,
                control,
            }) => {
                let result = engine.search_with_control(&limits, &control);
                let _ = completions.send(SearchCompletion::new(task_id, result));
            }
            Command::Shutdown => break,
        }
    }
    engine
}

fn worker_stopped() -> EngineError {
    EngineError::new("engine worker thread stopped")
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::{self, Receiver};
    use std::time::{Duration, Instant};

    use chess_kit_primitives::{SearchDepth, Square};

    use super::*;
    use crate::Engine;

    struct TestEngine {
        searches_started: Sender<()>,
        dropped: Option<Sender<()>>,
        wait_for_cancellation: bool,
        new_games: usize,
        positions: Vec<(PositionBase, Vec<Move>)>,
    }

    impl TestEngine {
        fn new() -> (Self, Receiver<()>, Receiver<()>) {
            Self::with_search_mode(true)
        }

        fn immediate() -> (Self, Receiver<()>, Receiver<()>) {
            Self::with_search_mode(false)
        }

        fn with_search_mode(wait_for_cancellation: bool) -> (Self, Receiver<()>, Receiver<()>) {
            let (started_sender, started) = mpsc::channel();
            let (dropped_sender, dropped) = mpsc::channel();
            (
                Self {
                    searches_started: started_sender,
                    dropped: Some(dropped_sender),
                    wait_for_cancellation,
                    new_games: 0,
                    positions: Vec::new(),
                },
                started,
                dropped,
            )
        }

        fn outcome(limits: &SearchLimits) -> SearchOutcome {
            SearchOutcome {
                best_move: Some(Move::new(Square::E2, Square::E4)),
                depth: limits.maximum_depth,
                score: 12,
                nodes: 42,
                elapsed: Duration::from_millis(1),
            }
        }
    }

    impl Drop for TestEngine {
        fn drop(&mut self) {
            if let Some(dropped) = self.dropped.take() {
                let _ = dropped.send(());
            }
        }
    }

    impl Engine for TestEngine {
        fn name(&self) -> &str {
            "Thread Test"
        }

        fn author(&self) -> &str {
            "Test Author"
        }

        fn new_game(&mut self) -> Result<(), EngineError> {
            self.new_games += 1;
            Ok(())
        }

        fn set_position(&mut self, base: PositionBase, moves: &[Move]) -> Result<(), EngineError> {
            self.positions.push((base, moves.to_vec()));
            Ok(())
        }

        fn play(&mut self, _mv: Move) -> Result<(), EngineError> {
            Ok(())
        }

        fn search(&mut self, limits: &SearchLimits) -> Result<SearchOutcome, EngineError> {
            Ok(Self::outcome(limits))
        }

        fn has_legal_moves(&self) -> bool {
            true
        }
    }

    impl ControllableEngine for TestEngine {
        fn search_with_control(
            &mut self,
            limits: &SearchLimits,
            control: &SearchControl,
        ) -> Result<SearchOutcome, EngineError> {
            let _ = self.searches_started.send(());
            if self.wait_for_cancellation {
                while !control.should_stop() {
                    thread::park_timeout(Duration::from_millis(1));
                }
                return Err(EngineError::new(format!(
                    "search at depth {} cancelled",
                    limits.maximum_depth
                )));
            }
            Ok(Self::outcome(limits))
        }
    }

    fn receive_completion(engine: &mut impl AsyncEngine) -> SearchCompletion {
        let deadline = Instant::now() + Duration::from_secs(1);
        while Instant::now() < deadline {
            if let Some(completion) = engine.try_search_completion().unwrap() {
                return completion;
            }
            thread::yield_now();
        }
        panic!("timed out waiting for search completion");
    }

    #[test]
    fn starts_and_stops_a_search_asynchronously() {
        let (engine, started, _dropped) = TestEngine::new();
        let mut threaded = ThreadedEngine::new(engine).unwrap();
        let limits = SearchLimits::depth(SearchDepth::new(3).unwrap());

        let task_id = threaded.start_search(limits).unwrap();
        started.recv_timeout(Duration::from_secs(1)).unwrap();
        assert_eq!(threaded.name(), "Thread Test");
        assert!(threaded.stop_search(task_id).unwrap());

        let completion = receive_completion(&mut threaded);
        assert_eq!(completion.task_id, task_id);
        assert_eq!(
            completion.result.unwrap_err().to_string(),
            "search at depth 3 cancelled"
        );
        assert!(!threaded.stop_search(task_id).unwrap());
    }

    #[test]
    fn publishes_successful_completion_and_allows_another_task() {
        let (engine, started, _dropped) = TestEngine::immediate();
        let mut threaded = ThreadedEngine::new(engine).unwrap();
        let limits = SearchLimits::depth(SearchDepth::new(3).unwrap());

        let first_task = threaded.start_search(limits).unwrap();
        started.recv_timeout(Duration::from_secs(1)).unwrap();
        let completion = receive_completion(&mut threaded);
        assert_eq!(completion.task_id, first_task);
        assert_eq!(completion.result.unwrap().depth, limits.maximum_depth);

        let second_task = threaded.start_search(limits).unwrap();
        assert_ne!(second_task, first_task);
        started.recv_timeout(Duration::from_secs(1)).unwrap();
        assert_eq!(receive_completion(&mut threaded).task_id, second_task);
    }

    #[test]
    fn scopes_cancellation_to_the_active_task() {
        let (engine, started, _dropped) = TestEngine::new();
        let mut threaded = ThreadedEngine::new(engine).unwrap();
        let limits = SearchLimits::depth(SearchDepth::new(2).unwrap());

        let task_id = threaded.start_search(limits).unwrap();
        started.recv_timeout(Duration::from_secs(1)).unwrap();
        assert!(threaded.start_search(limits).is_err());
        assert!(
            !threaded
                .stop_search(SearchTaskId::new(task_id.get() + 1))
                .unwrap()
        );
        assert!(threaded.stop_search(task_id).unwrap());
        receive_completion(&mut threaded);

        let next_task_id = threaded.start_search(limits).unwrap();
        assert_ne!(next_task_id, task_id);
        started.recv_timeout(Duration::from_secs(1)).unwrap();
        assert!(threaded.stop_search(next_task_id).unwrap());
        receive_completion(&mut threaded);
    }

    #[test]
    fn task_ids_do_not_cancel_searches_owned_by_another_worker() {
        let (first_engine, first_started, _first_dropped) = TestEngine::new();
        let (second_engine, second_started, _second_dropped) = TestEngine::new();
        let mut first = ThreadedEngine::new(first_engine).unwrap();
        let mut second = ThreadedEngine::new(second_engine).unwrap();
        let limits = SearchLimits::depth(SearchDepth::new(2).unwrap());

        let first_task = first.start_search(limits).unwrap();
        let second_task = second.start_search(limits).unwrap();
        first_started.recv_timeout(Duration::from_secs(1)).unwrap();
        second_started.recv_timeout(Duration::from_secs(1)).unwrap();

        assert_ne!(first_task, second_task);
        assert!(!first.stop_search(second_task).unwrap());
        assert!(!second.stop_search(first_task).unwrap());
        assert!(first.stop_search(first_task).unwrap());
        assert!(second.stop_search(second_task).unwrap());
        receive_completion(&mut first);
        receive_completion(&mut second);
    }

    #[test]
    fn discarding_search_consumes_only_the_matching_completion() {
        let (engine, started, _dropped) = TestEngine::new();
        let mut threaded = ThreadedEngine::new(engine).unwrap();
        let limits = SearchLimits::depth(SearchDepth::new(2).unwrap());
        let first_task = threaded.start_search(limits).unwrap();
        started.recv_timeout(Duration::from_secs(1)).unwrap();

        assert!(
            !threaded
                .discard_search(SearchTaskId::new(first_task.get()))
                .unwrap()
        );
        assert!(threaded.discard_search(first_task).unwrap());
        assert!(threaded.try_search_completion().unwrap().is_none());

        let second_task = threaded.start_search(limits).unwrap();
        started.recv_timeout(Duration::from_secs(1)).unwrap();
        assert!(threaded.stop_search(second_task).unwrap());
        assert_eq!(receive_completion(&mut threaded).task_id, second_task);
    }

    #[test]
    fn state_change_finishes_active_search_before_starting_the_next() {
        let (engine, started, _dropped) = TestEngine::new();
        let mut threaded = ThreadedEngine::new(engine).unwrap();
        let limits = SearchLimits::depth(SearchDepth::new(2).unwrap());
        let first_task = threaded.start_search(limits).unwrap();
        started.recv_timeout(Duration::from_secs(1)).unwrap();
        let mv = Move::new(Square::E2, Square::E4);

        threaded
            .set_position(PositionBase::StartPos, &[mv])
            .unwrap();
        assert!(threaded.try_search_completion().unwrap().is_none());

        let second_task = threaded.start_search(limits).unwrap();
        assert_ne!(second_task, first_task);
        started.recv_timeout(Duration::from_secs(1)).unwrap();
        threaded.stop_search(second_task).unwrap();
        receive_completion(&mut threaded);

        assert_eq!(
            threaded.shutdown().unwrap().positions,
            vec![(PositionBase::StartPos, vec![mv])]
        );
    }

    #[test]
    fn serializes_state_updates_on_the_worker() {
        let (engine, _started, _dropped) = TestEngine::new();
        let mut threaded = ThreadedEngine::new(engine).unwrap();
        let mv = Move::new(Square::E2, Square::E4);

        threaded.new_game().unwrap();
        threaded
            .set_position(PositionBase::StartPos, &[mv])
            .unwrap();
        let engine = threaded.shutdown().unwrap();

        assert_eq!(engine.new_games, 1);
        assert_eq!(engine.positions, vec![(PositionBase::StartPos, vec![mv])]);
    }

    #[test]
    fn drop_cancels_search_and_joins_the_worker() {
        let (engine, started, dropped) = TestEngine::new();
        let mut threaded = ThreadedEngine::new(engine).unwrap();
        threaded
            .start_search(SearchLimits::depth(SearchDepth::new(2).unwrap()))
            .unwrap();
        started.recv_timeout(Duration::from_secs(1)).unwrap();

        drop(threaded);

        dropped.recv_timeout(Duration::from_secs(1)).unwrap();
    }
}
