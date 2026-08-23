//! Terminal application state and transitions.

mod action;
mod state;
mod update;

pub use action::{Action, Direction, Effect};
pub use state::{
    Analysis, App, ConnectionState, EngineIdentity, GameMode, ProtocolDirection, ProtocolEntry,
};
