//! Protocol-agnostic chess engine session built from the toolkit crates.
//!
//! Presentation entry points such as UCI and the interactive CLI should be thin
//! adapters over [`Engine`]. They own I/O and protocol shaping;
//! this crate owns position setup, move application, and search.

mod error;
mod moves;
mod engine;
mod types;

pub use error::EngineError;
pub use moves::format_uci_move;
pub use engine::DefaultEngine;
pub use types::{
    DEFAULT_SEARCH_DEPTH, DEFAULT_TRANSPOSITION_TABLE_SIZE_MB, MAX_SEARCH_DEPTH, PositionBase,
    SearchOutcome, SearchRequest,
};

use std::fmt::Display;

use chess_kit_primitives::Move;

/// `Engine` is the protocol-agnostic session surface used by presentation
/// adapters
///
/// Adapters such as UCI and the interactive CLI should depend on this contract
/// rather than composing search, evaluation, and position state themselves
///
/// @trait
pub trait Engine: Display {
    /// new_game resets engine state for a fresh game and loads the start position
    ///
    /// @return: Ok on success, or the engine error
    /// @side-effects: clears game-associated search state and replaces the position
    fn new_game(&mut self) -> Result<(), EngineError>;

    /// set_position replaces the current position from a base and UCI move history
    ///
    /// @param: base - root position before applying moves
    /// @param: moves - ordered UCI move strings to apply
    /// @return: Ok on success, or the engine error
    /// @side-effects: replaces the current engine position on success
    fn set_position(&mut self, base: PositionBase, moves: &[&str]) -> Result<(), EngineError>;

    /// play_uci applies one legal move written in UCI notation
    ///
    /// @param: uci - move in UCI notation
    /// @return: Ok on success, or the engine error
    /// @side-effects: updates the current position when the move is legal
    fn play_uci(&mut self, uci: &str) -> Result<(), EngineError>;

    /// apply plays a previously validated legal move
    ///
    /// @param: mv - legal move to apply
    /// @return: Ok on success, or the engine error
    /// @side-effects: updates the current position
    fn apply(&mut self, mv: Move) -> Result<(), EngineError>;

    /// search searches the current position to the requested depth
    ///
    /// @param: depth - maximum search depth in plies
    /// @return: completed search outcome, or the engine error
    /// @side-effects: may modify engine search state
    fn search(&mut self, depth: u8) -> Result<SearchOutcome, EngineError>;

    /// has_legal_moves reports whether the side to move has any legal reply
    ///
    /// @return: true when at least one legal move exists
    fn has_legal_moves(&self) -> bool;
}
