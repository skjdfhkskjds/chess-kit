//! Universal Chess Interface (UCI) protocol support.
//!
//! This module owns the text protocol and deliberately knows nothing about a
//! particular chess implementation. Engines implement [`UciEngine`], then pass
//! that implementation to [`run`] or [`run_with_io`].

mod command;
mod engine;
mod handler;
mod protocol;
mod types;

pub use command::*;
pub use engine::*;
pub use protocol::{run, run_with_io};
pub use types::*;
