use std::io::{self, BufRead, Write};
use std::str::FromStr;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::thread;
use std::time::Duration;

use super::handler::{CommandFlow, UciHandler};
use super::{Command, UciEngine};

/// SEARCH_POLL_INTERVAL bounds completion and command handling latency.
const SEARCH_POLL_INTERVAL: Duration = Duration::from_millis(1);

/// `ProtocolInput` carries parsed caller-thread input to the handler thread.
///
/// @type
enum ProtocolInput {
    Command(Command),
    ParseError(String),
    EndOfInput,
}

/// run runs a UCI engine over standard input and standard output
///
/// @param: engine - mutable reference to the engine handling UCI commands
/// @return: Ok when the protocol loop exits, or an I/O error
/// @side-effects: reads standard input, writes standard output, and modifies
///                engine state
pub fn run<EngineT>(engine: &mut EngineT) -> io::Result<()>
where
    EngineT: UciEngine + Send + ?Sized,
{
    let stdin = io::stdin();
    let stdout = io::stdout();
    run_with_io(engine, stdin.lock(), stdout)
}

/// run_with_io runs a UCI engine over caller-provided streams
///
/// note: caller-provided streams make complete protocol sessions testable
///       without spawning a child process
///
/// @marker: EngineT - UCI engine implementation
/// @marker: ReaderT - buffered command input stream type
/// @marker: WriterT - protocol output stream type
/// @param: engine - mutable reference to the engine handling UCI commands
/// @param: reader - stream containing newline-delimited UCI commands
/// @param: writer - stream that receives UCI responses
/// @return: Ok when the protocol loop exits, or an I/O error
/// @side-effects: reads input, writes output, and modifies engine state
pub fn run_with_io<EngineT, ReaderT, WriterT>(
    engine: &mut EngineT,
    mut reader: ReaderT,
    mut writer: WriterT,
) -> io::Result<()>
where
    EngineT: UciEngine + Send + ?Sized,
    ReaderT: BufRead,
    WriterT: Write + Send,
{
    thread::scope(|scope| {
        let (sender, inputs) = mpsc::channel();
        let handler_thread = scope.spawn(move || run_handler(engine, &mut writer, inputs));
        let input_result = read_commands(&mut reader, &sender);
        drop(sender);

        let handler_result = handler_thread
            .join()
            .map_err(|_| io::Error::other("UCI handler thread panicked"))?;
        handler_result?;
        input_result
    })
}

/// read_commands parses input on the caller thread and forwards typed events.
///
/// @param: reader - buffered UCI input stream
/// @param: sender - destination for parsed protocol input
/// @return: Ok at end of input, or an I/O error
/// @side-effects: reads input and sends protocol events
fn read_commands<ReaderT>(
    reader: &mut ReaderT,
    sender: &mpsc::Sender<ProtocolInput>,
) -> io::Result<()>
where
    ReaderT: BufRead,
{
    let mut line = String::new();
    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            let _ = sender.send(ProtocolInput::EndOfInput);
            return Ok(());
        }

        let input = match Command::from_str(&line) {
            Ok(command) => ProtocolInput::Command(command),
            Err(error) => ProtocolInput::ParseError(error.to_string()),
        };
        let should_quit = matches!(input, ProtocolInput::Command(Command::Quit));
        if sender.send(input).is_err() {
            return Ok(());
        }
        // The handler thread owns engine shutdown, but the caller must avoid
        // blocking on another read after it has forwarded `quit`.
        if should_quit {
            return Ok(());
        }
    }
}

/// run_handler owns engine state and output while servicing input and search.
///
/// @param: engine - engine receiving parsed commands
/// @param: writer - output stream receiving UCI responses
/// @param: inputs - parsed input receiver
/// @return: Ok after quit or input closure, or an I/O error
/// @side-effects: handles engine commands and writes protocol output
fn run_handler<EngineT, WriterT>(
    engine: &mut EngineT,
    writer: &mut WriterT,
    inputs: Receiver<ProtocolInput>,
) -> io::Result<()>
where
    EngineT: UciEngine + ?Sized,
    WriterT: Write + ?Sized,
{
    let mut handler = UciHandler::new(engine, writer);
    loop {
        handler.poll_search()?;
        handler.flush()?;

        match inputs.recv_timeout(SEARCH_POLL_INTERVAL) {
            Ok(ProtocolInput::Command(command)) => {
                if handler.handle(command)? == CommandFlow::Quit {
                    return Ok(());
                }
                handler.flush()?;
            }
            Ok(ProtocolInput::ParseError(error)) => {
                handler.write_error(error)?;
                handler.flush()?;
            }
            Ok(ProtocolInput::EndOfInput) => return Ok(()),
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => return Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;
    use std::io::{Cursor, Read};
    use std::sync::{Arc, Condvar, Mutex};

    use chess_kit_primitives::SearchDepth;

    use super::*;
    use crate::uci::{PositionCommand, SearchInfo, SearchLimits, SearchResult, UciMove};

    #[derive(Default)]
    struct TestEngine {
        positions: Vec<PositionCommand>,
        new_games: usize,
        debug: Option<bool>,
        started_limits: Vec<SearchLimits>,
        stopped_tasks: Vec<usize>,
        completion: Option<SearchResult>,
        complete_on_start: bool,
        completion_signal: Option<Arc<(Mutex<bool>, Condvar)>>,
        ponder_hits: usize,
    }

    fn search_result() -> SearchResult {
        let mut result = SearchResult::new(Some(UciMove::from_str("e2e4").unwrap()));
        result.info = SearchInfo {
            depth: Some(SearchDepth::new(3).unwrap()),
            score_cp: Some(12),
            nodes: Some(42),
            elapsed: Some(std::time::Duration::from_millis(2)),
        };
        result
    }

    impl UciEngine for TestEngine {
        type Error = Infallible;
        type SearchTaskId = usize;

        fn name(&self) -> &str {
            "Test Engine"
        }

        fn author(&self) -> &str {
            "Test Author"
        }

        fn new_game(&mut self) -> Result<(), Self::Error> {
            self.new_games += 1;
            Ok(())
        }

        fn set_position(&mut self, position: &PositionCommand) -> Result<(), Self::Error> {
            self.positions.push(position.clone());
            Ok(())
        }

        fn start_search(
            &mut self,
            limits: &SearchLimits,
        ) -> Result<Self::SearchTaskId, Self::Error> {
            let task_id = self.started_limits.len();
            self.started_limits.push(limits.clone());
            if self.complete_on_start {
                self.completion = Some(search_result());
            }
            Ok(task_id)
        }

        fn poll_search(&mut self) -> Result<Option<SearchResult>, Self::Error> {
            let completion = self.completion.take();
            if completion.is_some()
                && let Some(signal) = &self.completion_signal
            {
                let (completed, wake_reader) = &**signal;
                *completed.lock().unwrap() = true;
                wake_reader.notify_one();
            }
            Ok(completion)
        }

        fn stop_search(&mut self, task_id: Self::SearchTaskId) -> Result<bool, Self::Error> {
            self.stopped_tasks.push(task_id);
            self.completion = Some(search_result());
            Ok(true)
        }

        fn ponder_hit(&mut self) {
            self.ponder_hits += 1;
        }

        fn set_debug(&mut self, enabled: bool) {
            self.debug = Some(enabled);
        }
    }

    #[test]
    fn runs_a_minimal_uci_session() {
        let input = Cursor::new(
            b"uci\nisready\nucinewgame\nposition startpos moves e2e4\ngo wtime 1000 btime 1000\nquit\n",
        );
        let mut output = Vec::new();
        let mut engine = TestEngine {
            complete_on_start: true,
            ..TestEngine::default()
        };

        run_with_io(&mut engine, input, &mut output).unwrap();

        assert_eq!(engine.new_games, 1);
        assert_eq!(engine.positions.len(), 1);
        assert_eq!(
            String::from_utf8(output).unwrap(),
            concat!(
                "id name Test Engine\n",
                "id author Test Author\n",
                "uciok\n",
                "readyok\n",
                "info depth 3 score cp 12 nodes 42 time 2\n",
                "bestmove e2e4\n",
            )
        );
    }

    #[test]
    fn routes_commands_through_a_uci_engine_trait_object() {
        let input = Cursor::new(
            b"debug on\ngo infinite\nisready\nstop\nponderhit\nunsupported\nquit\nisready\n",
        );
        let mut output = Vec::new();
        let mut engine = TestEngine::default();

        {
            let engine: &mut (dyn UciEngine<Error = Infallible, SearchTaskId = usize> + Send) =
                &mut engine;
            run_with_io(engine, input, &mut output).unwrap();
        }

        assert_eq!(engine.debug, Some(true));
        assert_eq!(engine.started_limits.len(), 1);
        assert!(engine.started_limits[0].infinite);
        assert_eq!(engine.stopped_tasks, [0]);
        assert_eq!(engine.ponder_hits, 1);
        assert_eq!(
            String::from_utf8(output).unwrap(),
            concat!(
                "readyok\n",
                "info depth 3 score cp 12 nodes 42 time 2\n",
                "bestmove e2e4\n",
            )
        );
    }

    #[test]
    fn natural_completion_writes_bestmove_without_another_command() {
        let completion_signal = Arc::new((Mutex::new(false), Condvar::new()));
        let input =
            WaitForCompletionReader::new(b"go depth 3\n".to_vec(), Arc::clone(&completion_signal));
        let mut output = Vec::new();
        let mut engine = TestEngine {
            complete_on_start: true,
            completion_signal: Some(completion_signal),
            ..TestEngine::default()
        };

        run_with_io(&mut engine, input, &mut output).unwrap();

        assert_eq!(output.iter().filter(|&&byte| byte == b'\n').count(), 2);
        assert!(
            String::from_utf8(output)
                .unwrap()
                .contains("bestmove e2e4\n")
        );
    }

    /// `WaitForCompletionReader` blocks EOF until the handler polls a result.
    struct WaitForCompletionReader {
        input: Vec<u8>,
        consumed: usize,
        completion: Arc<(Mutex<bool>, Condvar)>,
    }

    impl WaitForCompletionReader {
        fn new(input: Vec<u8>, completion: Arc<(Mutex<bool>, Condvar)>) -> Self {
            Self {
                input,
                consumed: 0,
                completion,
            }
        }
    }

    impl Read for WaitForCompletionReader {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            let available = self.fill_buf()?;
            let read = available.len().min(buffer.len());
            buffer[..read].copy_from_slice(&available[..read]);
            self.consume(read);
            Ok(read)
        }
    }

    impl BufRead for WaitForCompletionReader {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            if self.consumed == self.input.len() {
                let (completed, wake_reader) = &*self.completion;
                let completed = completed.lock().unwrap();
                let (completed, timeout) = wake_reader
                    .wait_timeout_while(completed, Duration::from_secs(2), |completed| !*completed)
                    .unwrap();
                if timeout.timed_out() && !*completed {
                    return Err(io::Error::new(
                        io::ErrorKind::TimedOut,
                        "timed out waiting for asynchronous search completion",
                    ));
                }
            }
            Ok(&self.input[self.consumed..])
        }

        fn consume(&mut self, amount: usize) {
            self.consumed = (self.consumed + amount).min(self.input.len());
        }
    }
}
