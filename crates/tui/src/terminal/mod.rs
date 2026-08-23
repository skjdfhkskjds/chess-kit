//! Terminal configuration, input, and runtime lifecycle.

mod config;
mod event;
mod runtime;

pub use config::{ConfigError, TerminalConfig};
pub use runtime::{run_terminal, run_terminal_with_piece_set};
