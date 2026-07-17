//! Universal Chess Interface (UCI) protocol support.
//!
//! This module owns the text protocol and the adapter to the protocol-neutral
//! engine API. Wrap an engine in [`UciAdapter`], then pass it to [`run`] or
//! [`run_with_io`].

mod adapter;
mod command;
mod engine;
mod handler;
mod protocol;
mod types;

pub use adapter::UciAdapter;
pub use command::*;
pub use engine::*;
pub use protocol::{run, run_with_io};
pub use types::*;
