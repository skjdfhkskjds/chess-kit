use std::fmt::Display;
use std::io::{self, Write};

use super::{Command, PositionCommand, SearchInfo, SearchLimits, SearchResult, UciEngine};

/// `CommandFlow` describes whether the protocol loop should continue after a
/// command has been handled
///
/// @type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum CommandFlow {
    Continue,
    Quit,
}

/// `UciHandler` is a type that adapts parsed UCI commands to an engine and
/// output stream
///
/// @marker: EngineT - UCI engine implementation
/// @marker: WriterT - protocol output stream type
/// @type
pub(super) struct UciHandler<'a, EngineT, WriterT>
where
    EngineT: UciEngine + ?Sized,
    WriterT: Write + ?Sized,
{
    engine: &'a mut EngineT,
    writer: &'a mut WriterT,
}

impl<'a, EngineT, WriterT> UciHandler<'a, EngineT, WriterT>
where
    EngineT: UciEngine + ?Sized,
    WriterT: Write + ?Sized,
{
    /// new creates a protocol adapter over an engine and output stream
    ///
    /// @param: engine - engine implementation receiving command operations
    /// @param: writer - stream that receives command responses
    /// @return: new UCI handler
    pub(super) fn new(engine: &'a mut EngineT, writer: &'a mut WriterT) -> Self {
        Self { engine, writer }
    }

    /// handle routes one parsed command to its command-specific adapter
    ///
    /// @param: command - parsed command to consume
    /// @return: protocol loop flow after handling the command, or an I/O error
    /// @side-effects: may write output and modify engine state
    pub(super) fn handle(&mut self, command: Command) -> io::Result<CommandFlow> {
        match command {
            Command::Uci => self.handle_uci()?,
            Command::Debug(enabled) => self.handle_debug(enabled),
            Command::IsReady => self.handle_is_ready()?,
            Command::UciNewGame => self.handle_uci_new_game()?,
            Command::Position(position) => self.handle_position(&position)?,
            Command::Go(limits) => self.handle_go(&limits)?,
            Command::Stop => self.handle_stop()?,
            Command::PonderHit => self.handle_ponder_hit(),
            Command::Quit => return Ok(self.handle_quit()),
            Command::Unknown => self.handle_unknown(),
        }

        Ok(CommandFlow::Continue)
    }

    /// handle_uci writes the engine identity and completes UCI initialization
    ///
    /// @return: Ok on success, or an I/O error
    /// @side-effects: writes the engine identity to the output stream
    fn handle_uci(&mut self) -> io::Result<()> {
        writeln!(self.writer, "id name {}", sanitize(self.engine.name()))?;
        writeln!(self.writer, "id author {}", sanitize(self.engine.author()))?;
        writeln!(self.writer, "uciok")
    }

    /// handle_debug updates the engine's diagnostic output mode
    ///
    /// @param: enabled - whether diagnostic output should be enabled
    /// @return: void
    /// @side-effects: modifies the engine's diagnostic output mode
    fn handle_debug(&mut self, enabled: bool) {
        self.engine.set_debug(enabled);
    }

    /// handle_is_ready acknowledges that the engine is ready for another command
    ///
    /// @return: Ok on success, or an I/O error
    /// @side-effects: writes the readiness response to the output stream
    fn handle_is_ready(&mut self) -> io::Result<()> {
        writeln!(self.writer, "readyok")
    }

    /// handle_uci_new_game resets engine state for a new game
    ///
    /// @return: Ok after handling the command, or an I/O error
    /// @side-effects: resets engine state and may write an error response
    fn handle_uci_new_game(&mut self) -> io::Result<()> {
        if let Err(error) = self.engine.new_game() {
            self.write_error(error)?;
        }
        Ok(())
    }

    /// handle_position replaces the engine's current position
    ///
    /// @param: position - base position and move history to apply
    /// @return: Ok after handling the command, or an I/O error
    /// @side-effects: modifies engine state and may write an error response
    fn handle_position(&mut self, position: &PositionCommand) -> io::Result<()> {
        if let Err(error) = self.engine.set_position(position) {
            self.write_error(error)?;
        }
        Ok(())
    }

    /// handle_go starts a search and writes its result
    ///
    /// @param: limits - constraints to apply to the search
    /// @return: Ok after handling the command, or an I/O error
    /// @side-effects: searches with the engine and writes the search response
    fn handle_go(&mut self, limits: &SearchLimits) -> io::Result<()> {
        match self.engine.search(limits) {
            Ok(result) => self.write_search_result(&result),
            Err(error) => {
                self.write_error(error)?;
                writeln!(self.writer, "bestmove 0000")
            }
        }
    }

    /// handle_stop stops an active search and writes a result when one is ready
    ///
    /// @return: Ok after handling the command, or an I/O error
    /// @side-effects: may stop engine search state and write a search response
    fn handle_stop(&mut self) -> io::Result<()> {
        match self.engine.stop() {
            Ok(Some(result)) => self.write_search_result(&result),
            Ok(None) => Ok(()),
            Err(error) => self.write_error(error),
        }
    }

    /// handle_ponder_hit promotes a ponder search to a normal search
    ///
    /// @return: void
    /// @side-effects: may modify engine search state
    fn handle_ponder_hit(&mut self) {
        self.engine.ponder_hit();
    }

    /// handle_quit requests that the protocol loop exit
    ///
    /// @return: protocol flow requesting that the loop exit
    fn handle_quit(&mut self) -> CommandFlow {
        CommandFlow::Quit
    }

    /// handle_unknown intentionally ignores commands outside the supported subset
    ///
    /// @return: void
    fn handle_unknown(&mut self) {}

    /// write_search_result writes search information followed by `bestmove`
    ///
    /// @param: result - completed search result to serialize
    /// @return: Ok on success, or an I/O error
    /// @side-effects: writes to the output stream
    fn write_search_result(&mut self, result: &SearchResult) -> io::Result<()> {
        self.write_search_info(&result.info)?;

        // UCI represents the absence of a legal best move with the null move
        let best_move = result
            .best_move
            .as_ref()
            .map_or_else(|| "0000".to_owned(), ToString::to_string);
        write!(self.writer, "bestmove {best_move}")?;
        if let Some(ponder) = &result.ponder {
            write!(self.writer, " ponder {ponder}")?;
        }
        writeln!(self.writer)
    }

    /// write_search_info writes the available fields of a UCI `info` response
    ///
    /// @param: info - optional search information to serialize
    /// @return: Ok on success, or an I/O error
    /// @side-effects: writes to the output stream when information is available
    fn write_search_info(&mut self, info: &SearchInfo) -> io::Result<()> {
        // omit the info line when the engine did not report any search details
        if info == &SearchInfo::default() {
            return Ok(());
        }

        write!(self.writer, "info")?;
        if let Some(depth) = info.depth {
            write!(self.writer, " depth {depth}")?;
        }
        if let Some(score) = info.score_cp {
            write!(self.writer, " score cp {score}")?;
        }
        if let Some(nodes) = info.nodes {
            write!(self.writer, " nodes {nodes}")?;
        }
        if let Some(elapsed) = info.elapsed {
            write!(self.writer, " time {}", elapsed.as_millis())?;
        }
        writeln!(self.writer)
    }

    /// write_error writes a sanitized UCI `info string`
    ///
    /// @param: error - error to report to the GUI
    /// @return: Ok on success, or an I/O error
    /// @side-effects: writes to the output stream
    pub(super) fn write_error(&mut self, error: impl Display) -> io::Result<()> {
        writeln!(
            self.writer,
            "info string error: {}",
            sanitize(&error.to_string())
        )
    }

    /// flush makes command responses immediately visible to the caller
    ///
    /// @return: Ok on success, or an I/O error
    /// @side-effects: flushes the output stream
    pub(super) fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

/// sanitize removes line breaks that could inject additional protocol responses
///
/// @param: value - engine-provided text to sanitize
/// @return: text with carriage returns and newlines replaced by spaces
fn sanitize(value: &str) -> String {
    value.replace(['\r', '\n'], " ")
}
