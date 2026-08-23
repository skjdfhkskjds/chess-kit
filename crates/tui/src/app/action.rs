use crate::{RunnerEvent, UciCommand};

/// `Direction` describes screen-relative board cursor movement.
///
/// @type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// `Action` describes one user or engine input consumed by the application.
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Connect,
    MoveCursor(Direction),
    SelectSquare,
    CancelSelection,
    ToggleAnalysis,
    FlipBoard,
    NewGame,
    ToggleProtocol,
    ToggleHelp,
    Runner(RunnerEvent),
    TransportFailed(String),
    Quit,
}

/// `Effect` describes work requested by an application transition.
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Effect {
    Send(UciCommand),
    Quit,
}
