use std::env;
use std::fmt::Display;
use std::io::{self, BufRead, Write};
use std::str::FromStr;

use chess_kit::comm::uci::UciMove;
use chess_kit::engine::{
    DefaultEngine, Engine, MAX_SEARCH_DEPTH, SearchOutcome, format_uci_move,
};

/// DEFAULT_INTERACTIVE_SEARCH_DEPTH is the search depth used when `--depth` is
/// not supplied
pub const DEFAULT_INTERACTIVE_SEARCH_DEPTH: u8 = 6;

const USAGE: &str = "Usage: game [OPTIONS]\n\
\n\
Options:\n\
  -d, --depth <PLIES>  Search depth for engine moves (default: 6, range: 1-8)\n\
  -h, --help           Print help";

struct GameOptions {
    depth: u8,
}

fn parse_options(
    arguments: impl IntoIterator<Item = String>,
) -> Result<Option<GameOptions>, String> {
    let mut arguments = arguments.into_iter();
    let mut depth = None;

    while let Some(argument) = arguments.next() {
        let value = match argument.as_str() {
            "-h" | "--help" => return Ok(None),
            "-d" | "--depth" => Some(
                arguments
                    .next()
                    .ok_or_else(|| format!("{argument} requires a depth"))?,
            ),
            _ => argument.strip_prefix("--depth=").map(str::to_owned),
        };

        let Some(value) = value else {
            return Err(format!("unrecognized argument: {argument}"));
        };
        if depth.is_some() {
            return Err("depth may only be specified once".to_owned());
        }
        depth = Some(parse_depth(&value)?);
    }

    Ok(Some(GameOptions {
        depth: depth.unwrap_or(DEFAULT_INTERACTIVE_SEARCH_DEPTH),
    }))
}

fn parse_depth(value: &str) -> Result<u8, String> {
    value
        .parse::<u8>()
        .ok()
        .filter(|depth| (1..=MAX_SEARCH_DEPTH).contains(depth))
        .ok_or_else(|| {
            format!("depth must be an integer from 1 to {MAX_SEARCH_DEPTH} (got {value:?})")
        })
}

/// `InteractiveGame` is a human-friendly command-line façade over [`EngineApi`]
///
/// The façade owns only presentation concerns: prompts, move parsing UX, board
/// display, and game-over messaging. Game state and search live behind the
/// engine API
///
/// @type
pub struct InteractiveGame<EngineT> {
    engine: EngineT,
    search_depth: u8,
}

impl<EngineT> InteractiveGame<EngineT>
where
    EngineT: Engine,
{
    /// new creates an interactive game around the given engine
    ///
    /// @param: engine - engine used to play and display the game
    /// @param: search_depth - search depth used for engine moves
    /// @return: new interactive game
    pub fn new(engine: EngineT, search_depth: u8) -> Self {
        Self {
            engine,
            search_depth,
        }
    }

    /// run plays a game over standard input and standard output
    ///
    /// @return: Ok when the session ends, or an I/O error
    /// @side-effects: reads standard input, writes standard output, and modifies
    ///                the engine game state
    pub fn run(&mut self) -> io::Result<()> {
        let stdin = io::stdin();
        let stdout = io::stdout();
        self.run_with_io(stdin.lock(), stdout.lock())
    }

    /// run_with_io plays a game over caller-provided streams
    ///
    /// note: caller-provided streams make complete interactive sessions testable
    ///       without spawning a child process
    ///
    /// @marker: ReaderT - buffered input stream type
    /// @marker: WriterT - output stream type
    /// @param: reader - stream containing player moves
    /// @param: writer - stream that receives prompts, moves, and board displays
    /// @return: Ok when the session ends, or an I/O error
    /// @side-effects: reads input, writes output, and modifies the engine game state
    pub fn run_with_io<ReaderT, WriterT>(
        &mut self,
        mut reader: ReaderT,
        mut writer: WriterT,
    ) -> io::Result<()>
    where
        ReaderT: BufRead,
        WriterT: Write,
    {
        self.engine.new_game().map_err(engine_error)?;

        writeln!(
            writer,
            "You are playing White. Enter moves in UCI notation (for example, e2e4)."
        )?;
        writeln!(writer, "Enter `quit` or `exit` to stop.\n")?;
        writeln!(writer, "{}", self.engine)?;

        let mut line = String::new();
        loop {
            write!(writer, "Your move: ")?;
            writer.flush()?;

            line.clear();
            if reader.read_line(&mut line)? == 0 {
                writeln!(writer)?;
                break;
            }

            let input = line.trim();
            if matches!(input, "quit" | "exit") {
                break;
            }

            // Parse notation here; legality is still the engine's job.
            let player_move = match UciMove::from_str(input) {
                Ok(mv) if mv.as_str() != "0000" => mv,
                Ok(_) => {
                    writeln!(writer, "Invalid move: the null move cannot be played.")?;
                    continue;
                }
                Err(error) => {
                    writeln!(writer, "Invalid move: {error}")?;
                    continue;
                }
            };

            if let Err(error) = self.engine.play_uci(player_move.as_str()) {
                writeln!(writer, "Invalid move: {error}")?;
                continue;
            }

            let outcome = self.engine.search(self.search_depth).map_err(engine_error)?;
            let Some(engine_move) = outcome.best_move else {
                writeln!(writer, "\n{}", self.engine)?;
                writeln!(writer, "Game over: the engine has no legal moves.")?;
                break;
            };

            writeln!(
                writer,
                "Engine plays: {}{}",
                format_uci_move(engine_move),
                format_search_info(&outcome)
            )?;
            self.engine.apply(engine_move).map_err(engine_error)?;
            writeln!(writer, "\n{}", self.engine)?;

            if !self.engine.has_legal_moves() {
                writeln!(writer, "Game over: you have no legal moves.")?;
                break;
            }
        }

        Ok(())
    }
}

/// format_search_info formats the available search details for interactive output
///
/// @param: outcome - search outcome containing the details to format
/// @return: parenthesized search details, or an empty string when no details exist
fn format_search_info(outcome: &SearchOutcome) -> String {
    format!(
        " (depth {}, score {} cp, {} nodes)",
        outcome.depth, outcome.score, outcome.nodes
    )
}

/// engine_error converts a displayable engine error into an I/O error
///
/// @param: error - engine error to convert
/// @return: I/O error containing the engine error message
fn engine_error(error: impl Display) -> io::Error {
    io::Error::other(error.to_string())
}

fn run() -> Result<(), String> {
    let Some(options) = parse_options(env::args().skip(1))? else {
        println!("{USAGE}");
        return Ok(());
    };

    let engine = DefaultEngine::new().map_err(|error| error.to_string())?;
    InteractiveGame::new(engine, options.depth)
        .run()
        .map_err(|error| error.to_string())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("chess-kit game example: {error}");
        eprintln!("\n{USAGE}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::{self, Display};
    use std::io::Cursor;

    use chess_kit::engine::EngineError;
    use chess_kit::primitives::{Move, Square};

    use super::*;

    #[derive(Default)]
    struct TestEngine {
        moves: Vec<String>,
        searches: usize,
        search_depths: Vec<u8>,
        legal_after_engine: bool,
    }

    impl Display for TestEngine {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "board after: {}", self.moves.join(" "))
        }
    }

    impl Engine for TestEngine {
        fn new_game(&mut self) -> Result<(), EngineError> {
            self.moves.clear();
            self.legal_after_engine = true;
            Ok(())
        }

        fn set_position(
            &mut self,
            _base: chess_kit::engine::PositionBase,
            moves: &[&str],
        ) -> Result<(), EngineError> {
            self.moves = moves.iter().map(|mv| (*mv).to_owned()).collect();
            Ok(())
        }

        fn play_uci(&mut self, uci: &str) -> Result<(), EngineError> {
            if uci == "e2e5" {
                return Err(EngineError::new("illegal move: e2e5"));
            }
            self.moves.push(uci.to_owned());
            Ok(())
        }

        fn apply(&mut self, mv: Move) -> Result<(), EngineError> {
            self.moves.push(format_uci_move(mv));
            Ok(())
        }

        fn search(&mut self, depth: u8) -> Result<SearchOutcome, EngineError> {
            self.searches += 1;
            self.search_depths.push(depth);
            let best_move = match self.searches {
                1 => Some(Move::new(Square::E7, Square::E5)),
                _ => None,
            };
            if best_move.is_some() {
                self.legal_after_engine = false;
            }
            Ok(SearchOutcome {
                best_move,
                depth,
                score: 10,
                nodes: 20,
                elapsed: std::time::Duration::default(),
            })
        }

        fn has_legal_moves(&self) -> bool {
            self.legal_after_engine
        }
    }

    #[test]
    fn wraps_moves_and_configured_search_depth_into_a_human_session() {
        let input = Cursor::new(b"not-a-move\ne2e4\nquit\n");
        let mut output = Vec::new();
        let mut game = InteractiveGame::new(TestEngine::default(), 7);

        game.run_with_io(input, &mut output).unwrap();

        let output = String::from_utf8(output).unwrap();
        assert!(output.contains("board after: "));
        assert!(output.contains("Invalid move:"));
        assert!(output.contains("Engine plays: e7e5 (depth 7, score 10 cp, 20 nodes)"));
        assert!(output.contains("board after: e2e4 e7e5"));
        assert!(output.contains("Game over: you have no legal moves."));
        assert_eq!(game.engine.searches, 1);
        assert_eq!(game.engine.search_depths, [7]);
    }

    #[test]
    fn parses_default_and_explicit_search_depths() {
        let default = parse_options(Vec::new()).unwrap().unwrap();
        let explicit = parse_options(["--depth".to_owned(), "7".to_owned()])
            .unwrap()
            .unwrap();
        let equals = parse_options(["--depth=3".to_owned()]).unwrap().unwrap();

        assert_eq!(default.depth, 6);
        assert_eq!(explicit.depth, 7);
        assert_eq!(equals.depth, 3);
    }

    #[test]
    fn rejects_unsupported_search_depths() {
        assert!(parse_options(["--depth".to_owned(), "0".to_owned()]).is_err());
        assert!(parse_options(["--depth=9".to_owned()]).is_err());
        assert!(parse_options(["--depth=nope".to_owned()]).is_err());
    }
}
