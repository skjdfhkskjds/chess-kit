//! Protocol-agnostic chess engine session built from the toolkit crates.
//!
//! Presentation layers should be thin adapters over [`Engine`]. They own I/O,
//! notation, and protocol shaping; this crate owns position setup, move
//! application, and search.

mod engine;
mod error;
mod types;

pub use chess_kit_position::PositionSnapshot;
pub use engine::DefaultEngine;
pub use error::EngineError;
pub use types::{EngineConfig, PositionBase, SearchOutcome};

use chess_kit_primitives::{Depth, Move};

/// `Engine` is the protocol-agnostic session surface used by presentation
/// adapters
///
/// Presentation adapters should depend on this contract rather than composing
/// search, evaluation, and position state themselves.
///
/// @trait
pub trait Engine {
    /// name returns the engine's display name
    ///
    /// @return: engine display name
    fn name(&self) -> &str;

    /// author returns the engine author's display name
    ///
    /// @return: engine author display name
    fn author(&self) -> &str;

    /// new_game resets engine state for a fresh game and loads the start position
    ///
    /// @return: Ok on success, or the engine error
    /// @side-effects: clears game-associated search state and replaces the position
    fn new_game(&mut self) -> Result<(), EngineError>;

    /// set_position replaces the current position from a base and move history
    ///
    /// @param: base - root position before applying moves
    /// @param: moves - ordered engine moves to apply
    /// @return: Ok on success, or the engine error
    /// @side-effects: replaces the current engine position on success
    fn set_position(&mut self, base: PositionBase, moves: &[Move]) -> Result<(), EngineError>;

    /// play applies a move after resolving it against the current legal moves
    ///
    /// @param: mv - move requested by a caller
    /// @return: Ok on success, or the engine error
    /// @side-effects: updates the current position when the move is legal
    fn play(&mut self, mv: Move) -> Result<(), EngineError>;

    /// search searches the current position to the requested depth
    ///
    /// @param: depth - maximum search depth in plies
    /// @return: completed search outcome, or the engine error
    /// @side-effects: may modify engine search state
    fn search(&mut self, depth: Depth) -> Result<SearchOutcome, EngineError>;

    /// has_legal_moves reports whether the side to move has any legal reply
    ///
    /// @return: true when at least one legal move exists
    fn has_legal_moves(&self) -> bool;
}

/// `PositionProvider` exposes an owned view of an engine's current position
///
/// Presentation adapters that render a board can require this capability
/// without making position inspection part of every engine integration.
///
/// @trait
pub trait PositionProvider {
    /// position returns an owned snapshot of the current position
    ///
    /// @return: current position snapshot
    fn position(&self) -> PositionSnapshot;
}
