//! Human-oriented interactive command-line adapter.

use std::fmt::Display;
use std::io::{self, BufRead, Write};
use std::str::FromStr;

use chess_kit_engine::{Board, Engine, SearchOutcome};
use chess_kit_primitives::{Black, Depth, Move, Pieces, Sides, Square, White};

use crate::uci::UciMove;

/// `InteractiveGame` is a human-friendly command-line facade over [`Engine`].
///
/// It owns prompts, notation conversion, board display, and game-over
/// messaging. Position state, legality, move application, and search remain in
/// the engine
///
/// @marker: EngineT - protocol-neutral engine implementation
/// @type
pub struct InteractiveGame<EngineT> {
    engine: EngineT,     // engine used to play the game
    search_depth: Depth, // fixed depth used for engine replies
}

impl<EngineT> InteractiveGame<EngineT>
where
    EngineT: Engine,
{
    /// new creates an interactive game around the given engine
    ///
    /// @param: engine - protocol-neutral engine session
    /// @param: search_depth - fixed search depth used for engine replies
    /// @return: interactive game adapter
    pub const fn new(engine: EngineT, search_depth: Depth) -> Self {
        Self {
            engine,
            search_depth,
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

    /// run plays a game over standard input and standard output
    ///
    /// @return: Ok when the session exits, or an I/O error
    /// @side-effects: reads standard input, writes standard output, and modifies
    ///                engine game state
    pub fn run(&mut self) -> io::Result<()> {
        let stdin = io::stdin();
        let stdout = io::stdout();
        self.run_with_io(stdin.lock(), stdout.lock())
    }

    /// run_with_io plays a game over caller-provided streams
    ///
    /// @marker: ReaderT - buffered input stream type
    /// @marker: WriterT - output stream type
    /// @param: reader - stream containing player moves
    /// @param: writer - stream receiving prompts, moves, and board displays
    /// @return: Ok when the session exits, or an I/O error
    /// @side-effects: reads input, writes output, and modifies engine game state
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
        write_board(&mut writer, &self.engine.board())?;

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
                Ok(mv) => match Move::try_from(&mv) {
                    Ok(mv) => mv,
                    Err(_) => {
                        writeln!(writer, "Invalid move: the null move cannot be played.")?;
                        continue;
                    }
                },
                Err(error) => {
                    writeln!(writer, "Invalid move: {error}")?;
                    continue;
                }
            };

            if let Err(error) = self.engine.play(player_move) {
                writeln!(writer, "Invalid move: {error}")?;
                continue;
            }

            let outcome = self
                .engine
                .search(self.search_depth)
                .map_err(engine_error)?;
            let Some(engine_move) = outcome.best_move else {
                write_board(&mut writer, &self.engine.board())?;
                writeln!(writer, "Game over: the engine has no legal moves.")?;
                break;
            };

            writeln!(
                writer,
                "Engine plays: {}{}",
                UciMove::from(engine_move),
                format_search_info(&outcome)
            )?;
            self.engine.play(engine_move).map_err(engine_error)?;
            write_board(&mut writer, &self.engine.board())?;

            if !self.engine.has_legal_moves() {
                writeln!(writer, "Game over: you have no legal moves.")?;
                break;
            }
        }

        Ok(())
    }
}

/// format_search_info formats search details for interactive output
///
/// @param: outcome - completed engine search outcome
/// @return: formatted parenthesized search details
fn format_search_info(outcome: &SearchOutcome) -> String {
    format!(
        " (depth {}, score {} cp, {} nodes)",
        outcome.depth, outcome.score, outcome.nodes
    )
}

/// write_board renders a protocol-neutral board snapshot for the human CLI
///
/// @param: writer - output stream receiving the rendered board
/// @param: board - board snapshot to render
/// @return: Ok after writing, or an I/O error
/// @side-effects: writes the board to the output stream
fn write_board(writer: &mut impl Write, board: &Board) -> io::Result<()> {
    writeln!(writer)?;
    for rank in (0..8).rev() {
        write!(writer, "{}", rank + 1)?;
        for file in 0..8 {
            let square = Square::from_idx((rank * 8 + file) as usize);
            let symbol = board
                .piece_at(square)
                .map_or('.', |(side, piece)| piece_symbol(side, piece));
            write!(writer, " {symbol}")?;
        }
        writeln!(writer)?;
    }
    writeln!(writer, "  A B C D E F G H")?;
    writeln!(
        writer,
        "{} to move",
        match board.side_to_move() {
            Sides::White => "White",
            Sides::Black => "Black",
        }
    )
}

/// piece_symbol returns the side-aware display symbol for a primitive piece
///
/// @param: side - side that owns the piece
/// @param: piece - piece to display
/// @return: Unicode piece symbol
fn piece_symbol(side: Sides, piece: Pieces) -> char {
    let display = match side {
        Sides::White => piece.display::<White>().to_string(),
        Sides::Black => piece.display::<Black>().to_string(),
    };
    display
        .chars()
        .next()
        .expect("primitive piece displays are never empty")
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
    use std::io::Cursor;
    use std::time::Duration;

    use chess_kit_engine::{EngineError, PositionBase};

    use super::*;

    #[derive(Default)]
    struct TestEngine {
        moves: Vec<Move>,
        searches: usize,
        search_depths: Vec<Depth>,
        legal_after_engine: bool,
    }

    impl Engine for TestEngine {
        fn name(&self) -> &str {
            "CLI Test"
        }

        fn author(&self) -> &str {
            "Test Author"
        }

        fn board(&self) -> Board {
            Board::empty(Sides::White)
        }

        fn new_game(&mut self) -> Result<(), EngineError> {
            self.moves.clear();
            self.legal_after_engine = true;
            Ok(())
        }

        fn set_position(&mut self, _base: PositionBase, moves: &[Move]) -> Result<(), EngineError> {
            self.moves = moves.to_vec();
            Ok(())
        }

        fn play(&mut self, mv: Move) -> Result<(), EngineError> {
            if UciMove::from(mv).as_str() == "e2e5" {
                return Err(EngineError::new("illegal move: e2e5"));
            }
            self.moves.push(mv);
            Ok(())
        }

        fn search(&mut self, depth: Depth) -> Result<SearchOutcome, EngineError> {
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
                elapsed: Duration::default(),
            })
        }

        fn has_legal_moves(&self) -> bool {
            self.legal_after_engine
        }
    }

    #[test]
    fn wraps_moves_and_search_into_a_human_session() {
        let input = Cursor::new(b"not-a-move\ne2e4\nquit\n");
        let mut output = Vec::new();
        let mut game = InteractiveGame::new(TestEngine::default(), 7);

        game.run_with_io(input, &mut output).unwrap();

        let output = String::from_utf8(output).unwrap();
        assert!(output.contains("Invalid move:"));
        assert!(output.contains("Engine plays: e7e5 (depth 7, score 10 cp, 20 nodes)"));
        let moves = game
            .engine()
            .moves
            .iter()
            .copied()
            .map(UciMove::from)
            .map(|mv| mv.to_string())
            .collect::<Vec<_>>();
        assert_eq!(moves, ["e2e4", "e7e5"]);
        assert!(output.contains("Game over: you have no legal moves."));
        assert_eq!(game.engine().search_depths, [7]);
    }
}
