use std::fmt::Display;
use std::io::{self, BufRead, Write};
use std::str::FromStr;

use super::{Command, SearchInfo, SearchResult, UciEngine};

/// Runs a UCI engine over standard input and standard output.
pub fn run<EngineT: UciEngine>(engine: &mut EngineT) -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    run_with_io(engine, stdin.lock(), stdout.lock())
}

/// Runs a UCI engine over caller-provided streams.
///
/// Supplying streams makes complete protocol sessions testable without spawning
/// a child process.
pub fn run_with_io<EngineT, ReaderT, WriterT>(
    engine: &mut EngineT,
    mut reader: ReaderT,
    mut writer: WriterT,
) -> io::Result<()>
where
    EngineT: UciEngine,
    ReaderT: BufRead,
    WriterT: Write,
{
    let mut line = String::new();
    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            break;
        }

        let command = match Command::from_str(&line) {
            Ok(command) => command,
            Err(error) => {
                write_error(&mut writer, error)?;
                writer.flush()?;
                continue;
            }
        };

        match command {
            Command::Uci => {
                writeln!(writer, "id name {}", sanitize(engine.name()))?;
                writeln!(writer, "id author {}", sanitize(engine.author()))?;
                writeln!(writer, "uciok")?;
            }
            Command::Debug(enabled) => engine.set_debug(enabled),
            Command::IsReady => writeln!(writer, "readyok")?,
            Command::UciNewGame => {
                if let Err(error) = engine.new_game() {
                    write_error(&mut writer, error)?;
                }
            }
            Command::Position(position) => {
                if let Err(error) = engine.set_position(&position) {
                    write_error(&mut writer, error)?;
                }
            }
            Command::Go(limits) => match engine.search(&limits) {
                Ok(result) => write_search_result(&mut writer, &result)?,
                Err(error) => {
                    write_error(&mut writer, error)?;
                    writeln!(writer, "bestmove 0000")?;
                }
            },
            Command::Stop => match engine.stop() {
                Ok(Some(result)) => write_search_result(&mut writer, &result)?,
                Ok(None) => {}
                Err(error) => write_error(&mut writer, error)?,
            },
            Command::PonderHit => engine.ponder_hit(),
            Command::Quit => break,
            Command::Unknown => {}
        }

        writer.flush()?;
    }

    Ok(())
}

fn write_search_result(writer: &mut impl Write, result: &SearchResult) -> io::Result<()> {
    write_search_info(writer, &result.info)?;

    let best_move = result
        .best_move
        .as_ref()
        .map_or_else(|| "0000".to_owned(), ToString::to_string);
    write!(writer, "bestmove {best_move}")?;
    if let Some(ponder) = &result.ponder {
        write!(writer, " ponder {ponder}")?;
    }
    writeln!(writer)
}

fn write_search_info(writer: &mut impl Write, info: &SearchInfo) -> io::Result<()> {
    if info == &SearchInfo::default() {
        return Ok(());
    }

    write!(writer, "info")?;
    if let Some(depth) = info.depth {
        write!(writer, " depth {depth}")?;
    }
    if let Some(score) = info.score_cp {
        write!(writer, " score cp {score}")?;
    }
    if let Some(nodes) = info.nodes {
        write!(writer, " nodes {nodes}")?;
    }
    if let Some(elapsed) = info.elapsed {
        write!(writer, " time {}", elapsed.as_millis())?;
    }
    writeln!(writer)
}

fn write_error(writer: &mut impl Write, error: impl Display) -> io::Result<()> {
    writeln!(
        writer,
        "info string error: {}",
        sanitize(&error.to_string())
    )
}

fn sanitize(value: &str) -> String {
    value.replace(['\r', '\n'], " ")
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;
    use std::io::Cursor;

    use super::*;
    use crate::uci::{PositionCommand, SearchLimits, UciMove};

    #[derive(Default)]
    struct TestEngine {
        positions: Vec<PositionCommand>,
        new_games: usize,
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
}
