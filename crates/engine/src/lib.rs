//! Protocol-agnostic chess engine session built from the toolkit crates.
//!
//! Presentation entry points such as UCI and the interactive CLI should be thin
//! adapters over [`Engine`] / [`EngineApi`]. They own I/O and protocol shaping;
//! this crate owns position setup, move application, and search.

mod api;
mod error;
mod moves;
mod session;
mod types;

pub use api::EngineApi;
pub use error::EngineError;
pub use moves::format_uci_move;
pub use session::Engine;
pub use types::{
    DEFAULT_SEARCH_DEPTH, DEFAULT_TRANSPOSITION_TABLE_SIZE_MB, MAX_SEARCH_DEPTH, PositionBase,
    SearchOutcome, SearchRequest,
};
