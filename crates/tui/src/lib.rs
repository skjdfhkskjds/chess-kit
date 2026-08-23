//! Interactive terminal client for Universal Chess Interface (UCI) engines.
//!
//! The crate separates child-process communication, application state, and
//! terminal rendering so protocol and interaction behavior remain testable
//! without a live terminal.

mod app;
mod runner;
mod terminal;
mod uci;
mod ui;

use std::io;

use chess_kit_engine::{Engine, EngineError, PositionProvider, PositionSnapshot};
use chess_kit_primitives::Move;

pub use app::{
    Action, Analysis, App, ConnectionState, Direction, Effect, EngineIdentity, GameMode,
    ProtocolDirection, ProtocolEntry,
};
pub use runner::{ProcessRunner, RunnerEvent};
pub use terminal::{ConfigError, TerminalConfig, run_terminal, run_terminal_with_piece_set};
pub use uci::{
    EngineMessage, EngineMessageKind, IdentityField, Score, ScoreBound, ScoreValue, SearchInfo,
    SearchRequest, UciCommand, UciOption, UciPosition,
};
pub use ui::{PieceSet, render, render_with_piece_set};

/// `GameSession` defines the local rules boundary used by the presentation.
///
/// The UCI engine remains the source of analysis and automatic replies. This
/// session validates every applied move and provides a protocol-neutral board
/// snapshot for rendering.
///
/// @trait
pub trait GameSession {
    /// new_game resets the local board to the standard starting position.
    ///
    /// @return: Ok on success, or an engine error
    /// @side-effects: replaces local game state
    fn new_game(&mut self) -> Result<(), EngineError>;

    /// play validates and applies a coordinate move.
    ///
    /// @param: chess_move - move to validate and apply
    /// @return: Ok on success, or an engine error
    /// @side-effects: updates local game state on success
    fn play(&mut self, chess_move: Move) -> Result<(), EngineError>;

    /// position returns an owned board snapshot.
    ///
    /// @return: current position snapshot
    fn position(&self) -> PositionSnapshot;

    /// has_legal_moves reports whether the side to move can make a legal move.
    ///
    /// @return: true when at least one legal move is available
    fn has_legal_moves(&self) -> bool;
}

impl<EngineT> GameSession for EngineT
where
    EngineT: Engine + PositionProvider,
{
    /// @impl: GameSession::new_game
    fn new_game(&mut self) -> Result<(), EngineError> {
        Engine::new_game(self)
    }

    /// @impl: GameSession::play
    fn play(&mut self, chess_move: Move) -> Result<(), EngineError> {
        Engine::play(self, chess_move)
    }

    /// @impl: GameSession::position
    fn position(&self) -> PositionSnapshot {
        PositionProvider::position(self)
    }

    /// @impl: GameSession::has_legal_moves
    fn has_legal_moves(&self) -> bool {
        Engine::has_legal_moves(self)
    }
}

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
