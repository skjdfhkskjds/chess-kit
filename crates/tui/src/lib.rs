//! Interactive terminal client for Universal Chess Interface (UCI) engines.
//!
//! The crate separates child-process communication, application state, and
//! terminal rendering so protocol and interaction behavior remain testable
//! without a live terminal.

mod runner;
mod uci;

use std::io;

pub use runner::{ProcessRunner, RunnerEvent};
pub use uci::{
    EngineMessage, EngineMessageKind, IdentityField, Score, ScoreBound, ScoreValue, SearchInfo,
    SearchRequest, UciCommand, UciOption, UciPosition,
};

/// `UciRunner` defines the command and event boundary for a UCI engine.
///
/// Implementations may use a local process, socket, or remote procedure call.
///
/// @trait
pub trait UciRunner {
    /// send writes one complete command to the engine.
    ///
    /// @param: command - typed UCI command to write
    /// @return: Ok after the command is flushed, or an I/O error
    /// @side-effects: writes to the engine transport
    fn send(&mut self, command: &UciCommand) -> io::Result<()>;

    /// try_recv returns the next available engine event without blocking.
    ///
    /// @return: next event when available, None otherwise, or an I/O error
    fn try_recv(&mut self) -> io::Result<Option<RunnerEvent>>;

    /// shutdown asks the engine to exit and releases transport resources.
    ///
    /// @return: Ok after the engine exits, or an I/O error
    /// @side-effects: may terminate the engine process after a grace period
    fn shutdown(&mut self) -> io::Result<()>;
}
