use std::io::{self, BufRead, Write};
use std::str::FromStr;

use super::handler::{CommandFlow, UciHandler};
use super::{Command, UciEngine};

/// run runs a UCI engine over standard input and standard output
///
/// @param: engine - mutable reference to the engine handling UCI commands
/// @return: Ok when the protocol loop exits, or an I/O error
/// @side-effects: reads standard input, writes standard output, and modifies
///                engine state
pub fn run<EngineT>(engine: &mut EngineT) -> io::Result<()>
where
    EngineT: UciEngine + ?Sized,
{
    let stdin = io::stdin();
    let stdout = io::stdout();
    run_with_io(engine, stdin.lock(), stdout.lock())
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
    EngineT: UciEngine + ?Sized,
    ReaderT: BufRead,
    WriterT: Write,
{
    let mut line = String::new();
    let mut handler = UciHandler::new(engine, &mut writer);
    loop {
        // read exactly one newline-delimited command from the GUI
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            break;
        }

        // report malformed known commands as UCI info while keeping the
        // protocol session alive for subsequent commands
        let command = match Command::from_str(&line) {
            Ok(command) => command,
            Err(error) => {
                handler.write_error(error)?;
                handler.flush()?;
                continue;
            }
        };

        // consume the parsed command through the protocol-to-engine adapter
        if handler.handle(command)? == CommandFlow::Quit {
            break;
        }

        // flush after every command so synchronous GUIs receive responses
        // without waiting for another input line
        handler.flush()?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;
    use std::io::Cursor;

    use super::*;
    use crate::uci::{PositionCommand, SearchInfo, SearchLimits, SearchResult, UciMove};

    #[derive(Default)]
    struct TestEngine {
        positions: Vec<PositionCommand>,
        new_games: usize,
        debug: Option<bool>,
        stops: usize,
        ponder_hits: usize,
    }

    impl UciEngine for TestEngine {
        type Error = Infallible;

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

        fn search(&mut self, _: &SearchLimits) -> Result<SearchResult, Self::Error> {
            let mut result = SearchResult::new(Some(UciMove::from_str("e2e4").unwrap()));
            result.info = SearchInfo {
                depth: Some(3),
                score_cp: Some(12),
                nodes: Some(42),
                elapsed: Some(std::time::Duration::from_millis(2)),
            };
            Ok(result)
        }

        fn stop(&mut self) -> Result<Option<SearchResult>, Self::Error> {
            self.stops += 1;
            Ok(None)
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
        let mut engine = TestEngine::default();

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
        let input = Cursor::new(b"debug on\nstop\nponderhit\nunsupported\nquit\nisready\n");
        let mut output = Vec::new();
        let mut engine = TestEngine::default();

        {
            let engine: &mut dyn UciEngine<Error = Infallible> = &mut engine;
            run_with_io(engine, input, &mut output).unwrap();
        }

        assert_eq!(engine.debug, Some(true));
        assert_eq!(engine.stops, 1);
        assert_eq!(engine.ponder_hits, 1);
        assert!(output.is_empty());
    }
}
