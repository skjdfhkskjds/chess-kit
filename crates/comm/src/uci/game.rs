use std::fmt::Display;
use std::io::{self, BufRead, Write};
use std::str::FromStr;

use super::{BasePosition, PositionCommand, SearchLimits, SearchResult, UciEngine, UciMove};

/// The fixed search depth used by the interactive command-line game.
pub const INTERACTIVE_SEARCH_DEPTH: u8 = 4;

/// A human-friendly command-line façade over a [`UciEngine`].
///
/// The façade owns the move history normally maintained by a UCI GUI. A player
/// only needs to enter moves such as `e2e4`; the façade constructs `position`
/// and `go depth 4` equivalents and displays the engine after every move.
pub struct InteractiveGame<EngineT> {
    engine: EngineT,
    moves: Vec<UciMove>,
}

impl<EngineT> InteractiveGame<EngineT>
where
    EngineT: UciEngine + Display,
{
    pub fn new(engine: EngineT) -> Self {
        Self {
            engine,
            moves: Vec::new(),
        }
    }

    /// Plays a game over standard input and standard output.
    pub fn run(&mut self) -> io::Result<()> {
        let stdin = io::stdin();
        let stdout = io::stdout();
        self.run_with_io(stdin.lock(), stdout.lock())
    }

    /// Plays a game over caller-provided streams.
    ///
    /// This is primarily useful for embedding the façade and testing complete
    /// interactive sessions.
    pub fn run_with_io<ReaderT, WriterT>(
        &mut self,
        mut reader: ReaderT,
        mut writer: WriterT,
    ) -> io::Result<()>
    where
        ReaderT: BufRead,
        WriterT: Write,
    {
        self.start_new_game()?;

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

            if let Err(error) = self.try_player_move(player_move.clone()) {
                writeln!(writer, "Invalid move: {error}")?;
                continue;
            }
            self.moves.push(player_move);

            let result = self.search(INTERACTIVE_SEARCH_DEPTH)?;
            let Some(engine_move) = result.best_move.clone() else {
                writeln!(writer, "\n{}", self.engine)?;
                writeln!(writer, "Game over: the engine has no legal moves.")?;
                break;
            };

            writeln!(
                writer,
                "Engine plays: {}{}",
                engine_move,
                format_search_info(&result)
            )?;
            self.moves.push(engine_move);
            self.set_position()?;
            writeln!(writer, "\n{}", self.engine)?;

            // A depth-one probe cheaply determines whether the player has any
            // legal reply after the engine's move.
            if self.search(1)?.best_move.is_none() {
                writeln!(writer, "Game over: you have no legal moves.")?;
                break;
            }
        }

        Ok(())
    }

    fn start_new_game(&mut self) -> io::Result<()> {
        self.moves.clear();
        self.engine.new_game().map_err(engine_error)?;
        self.set_position()
    }

    fn try_player_move(&mut self, player_move: UciMove) -> Result<(), EngineT::Error> {
        let mut moves = self.moves.clone();
        moves.push(player_move);
        self.engine.set_position(&PositionCommand {
            base: BasePosition::StartPos,
            moves,
        })
    }

    fn set_position(&mut self) -> io::Result<()> {
        self.engine
            .set_position(&PositionCommand {
                base: BasePosition::StartPos,
                moves: self.moves.clone(),
            })
            .map_err(engine_error)
    }

    fn search(&mut self, depth: u8) -> io::Result<SearchResult> {
        self.engine
            .search(&SearchLimits {
                depth: Some(depth),
                ..SearchLimits::default()
            })
            .map_err(engine_error)
    }
}

fn format_search_info(result: &SearchResult) -> String {
    let mut fields = Vec::new();
    if let Some(depth) = result.info.depth {
        fields.push(format!("depth {depth}"));
    }
    if let Some(score) = result.info.score_cp {
        fields.push(format!("score {score} cp"));
    }
    if let Some(nodes) = result.info.nodes {
        fields.push(format!("{nodes} nodes"));
    }

    if fields.is_empty() {
        String::new()
    } else {
        format!(" ({})", fields.join(", "))
    }
}

fn engine_error(error: impl Display) -> io::Error {
    io::Error::other(error.to_string())
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;
    use std::fmt;
    use std::io::Cursor;

    use super::*;
    use crate::uci::SearchInfo;

    #[derive(Default)]
    struct TestEngine {
        moves: Vec<UciMove>,
        searches: usize,
    }

    impl Display for TestEngine {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let moves = self
                .moves
                .iter()
                .map(UciMove::as_str)
                .collect::<Vec<_>>()
                .join(" ");
            write!(f, "board after: {moves}")
        }
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
            self.moves.clear();
            Ok(())
        }

        fn set_position(&mut self, position: &PositionCommand) -> Result<(), Self::Error> {
            self.moves.clone_from(&position.moves);
            Ok(())
        }

        fn search(&mut self, limits: &SearchLimits) -> Result<SearchResult, Self::Error> {
            self.searches += 1;
            let best_move = match self.searches {
                1 => Some("e7e5".parse().unwrap()),
                2 => Some("g1f3".parse().unwrap()),
                _ => None,
            };
            let mut result = SearchResult::new(best_move);
            result.info = SearchInfo {
                depth: limits.depth,
                score_cp: Some(10),
                nodes: Some(20),
                elapsed: None,
            };
            Ok(result)
        }
    }

    #[test]
    fn wraps_moves_and_depth_four_search_into_a_human_session() {
        let input = Cursor::new(b"not-a-move\ne2e4\nquit\n");
        let mut output = Vec::new();
        let mut game = InteractiveGame::new(TestEngine::default());

        game.run_with_io(input, &mut output).unwrap();

        let output = String::from_utf8(output).unwrap();
        assert!(output.contains("board after: "));
        assert!(output.contains("Invalid move:"));
        assert!(output.contains("Engine plays: e7e5 (depth 4, score 10 cp, 20 nodes)"));
        assert!(output.contains("board after: e2e4 e7e5"));
        assert_eq!(game.engine.searches, 2);
    }
}
