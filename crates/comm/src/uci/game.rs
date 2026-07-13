use std::fmt::Display;
use std::io::{self, BufRead, Write};
use std::str::FromStr;

use super::{BasePosition, PositionCommand, SearchLimits, SearchResult, UciEngine, UciMove};

/// INTERACTIVE_SEARCH_DEPTH is the fixed search depth used by the interactive
/// command-line game
pub const INTERACTIVE_SEARCH_DEPTH: u8 = 4;

/// `InteractiveGame` is a human-friendly command-line façade over a [`UciEngine`]
///
/// The façade owns the move history normally maintained by a UCI GUI. A player
/// only needs to enter moves such as `e2e4`; the façade constructs `position`
/// and `go depth 4` equivalents and displays the engine after every move
///
/// @type
pub struct InteractiveGame<EngineT> {
    engine: EngineT,     // engine used to validate positions and select moves
    moves: Vec<UciMove>, // move history from the initial position
}

impl<EngineT> InteractiveGame<EngineT>
where
    EngineT: UciEngine + Display,
{
    /// new creates an interactive game around the given engine
    ///
    /// @param: engine - engine used to play and display the game
    /// @return: new interactive game with an empty move history
    pub fn new(engine: EngineT) -> Self {
        Self {
            engine,
            moves: Vec::new(),
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
        // initialize the engine and send it an empty starting position before
        // accepting the first player move
        self.start_new_game()?;

        // print the session instructions and initial board
        writeln!(
            writer,
            "You are playing White. Enter moves in UCI notation (for example, e2e4)."
        )?;
        writeln!(writer, "Enter `quit` or `exit` to stop.\n")?;
        writeln!(writer, "{}", self.engine)?;

        let mut line = String::new();
        loop {
            // prompt for and read the next player move
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

            // parse the move and reject the UCI null move, which is valid
            // protocol notation but cannot be played by a human
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

            // let the engine validate the prospective history before recording
            // the player's move in the session
            if let Err(error) = self.try_player_move(player_move.clone()) {
                writeln!(writer, "Invalid move: {error}")?;
                continue;
            }
            self.moves.push(player_move);

            // search the accepted position for the engine's reply
            let result = self.search(INTERACTIVE_SEARCH_DEPTH)?;
            let Some(engine_move) = result.best_move.clone() else {
                writeln!(writer, "\n{}", self.engine)?;
                writeln!(writer, "Game over: the engine has no legal moves.")?;
                break;
            };

            // record the engine move and synchronize the engine with the full
            // move history before displaying the resulting board
            writeln!(
                writer,
                "Engine plays: {}{}",
                engine_move,
                format_search_info(&result)
            )?;
            self.moves.push(engine_move);
            self.set_position()?;
            writeln!(writer, "\n{}", self.engine)?;

            // a depth-one probe cheaply determines whether the player has any
            // legal reply after the engine's move
            if self.search(1)?.best_move.is_none() {
                writeln!(writer, "Game over: you have no legal moves.")?;
                break;
            }
        }

        Ok(())
    }

    /// start_new_game resets the game and loads the initial position
    ///
    /// @return: Ok on success, or an I/O-wrapped engine error
    /// @side-effects: clears the move history and resets the engine game state
    fn start_new_game(&mut self) -> io::Result<()> {
        self.moves.clear();
        self.engine.new_game().map_err(engine_error)?;
        self.set_position()
    }

    /// try_player_move asks the engine to validate a prospective player move
    /// without modifying the stored move history
    ///
    /// @param: player_move - prospective move to append to the current history
    /// @return: Ok if the resulting position is valid, or the engine error
    /// @side-effects: updates the engine to the prospective position
    fn try_player_move(&mut self, player_move: UciMove) -> Result<(), EngineT::Error> {
        let mut moves = self.moves.clone();
        moves.push(player_move);
        self.engine.set_position(&PositionCommand {
            base: BasePosition::StartPos,
            moves,
        })
    }

    /// set_position synchronizes the engine with the stored move history
    ///
    /// @return: Ok on success, or an I/O-wrapped engine error
    /// @side-effects: replaces the current engine position
    fn set_position(&mut self) -> io::Result<()> {
        self.engine
            .set_position(&PositionCommand {
                base: BasePosition::StartPos,
                moves: self.moves.clone(),
            })
            .map_err(engine_error)
    }

    /// search searches the current engine position to the requested depth
    ///
    /// @param: depth - maximum search depth in plies
    /// @return: completed search result, or an I/O-wrapped engine error
    /// @side-effects: may modify engine search state
    fn search(&mut self, depth: u8) -> io::Result<SearchResult> {
        self.engine
            .search(&SearchLimits {
                depth: Some(depth),
                ..SearchLimits::default()
            })
            .map_err(engine_error)
    }
}

/// format_search_info formats the available search details for interactive output
///
/// @param: result - search result containing the details to format
/// @return: parenthesized search details, or an empty string when no details exist
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

/// engine_error converts a displayable engine error into an I/O error
///
/// @param: error - engine error to convert
/// @return: I/O error containing the engine error message
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
