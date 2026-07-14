mod capture_info;
mod check_info;
mod draw;
mod position_metadata;
mod state;

pub use draw::DrawState;
pub(crate) use state::PositionState;

use chess_kit_collections::Stack;

/// History stores the private position state for each reversible ply
///
/// History provides the state snapshots used to restore incremental metadata and tactical
/// information when a move is undone
///
/// @type
pub(crate) type History = Stack<PositionState>;
