//! Terminal configuration, input, and runtime lifecycle.

mod config;
mod event;
mod runtime;

pub use config::{ConfigError, TerminalConfig};
pub use runtime::run_terminal;
