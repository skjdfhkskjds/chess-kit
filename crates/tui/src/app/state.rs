use chess_kit_engine::PositionSnapshot;
use chess_kit_primitives::{Sides, Square};

use crate::{GameSession, SearchInfo, UciOption};

/// MAX_PROTOCOL_ENTRIES bounds memory used by the visible protocol history.
pub(super) const MAX_PROTOCOL_ENTRIES: usize = 200;

/// `ConnectionState` describes the UCI session lifecycle.
///
/// @type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    AwaitingUci,
    AwaitingReady,
    Ready,
    Searching,
    Stopping,
    Exited,
    Failed,
}

/// `GameMode` selects whether engine search is advisory or plays Black.
///
/// @type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameMode {
    /// User moves update the board while engine analysis is display-only.
    Analysis,
    /// The human plays White and the connected engine automatically plays Black.
    PlayVsEngine,
}

/// `SearchPurpose` records how the result of the active search may be used.
///
/// @type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SearchPurpose {
    Analysis,
    EngineMove,
}

/// `EngineIdentity` contains the engine's advertised identity.
///
/// @type
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EngineIdentity {
    pub name: Option<String>,
    pub author: Option<String>,
}

/// `Analysis` contains the latest displayed engine analysis.
///
/// @type
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Analysis {
    pub info: SearchInfo,
    pub best_move: Option<String>,
    pub ponder: Option<String>,
}

/// `ProtocolDirection` identifies the source of a protocol-log entry.
///
/// @type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProtocolDirection {
    Gui,
    Engine,
    StandardError,
    System,
}

/// `ProtocolEntry` is one bounded protocol-log record.
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProtocolEntry {
    pub direction: ProtocolDirection,
    pub text: String,
}

/// `App` owns interactive state independently of terminal rendering.
///
/// @type
pub struct App {
    pub(super) game: Box<dyn GameSession>,
    pub(super) mode: GameMode,
    pub(super) position: PositionSnapshot,
    pub(super) connection: ConnectionState,
    pub(super) identity: EngineIdentity,
    pub(super) options: Vec<UciOption>,
    pub(super) analysis: Analysis,
    pub(super) moves: Vec<String>,
    pub(super) cursor: Square,
    pub(super) selected: Option<Square>,
    pub(super) flipped: bool,
    pub(super) show_protocol: bool,
    pub(super) show_help: bool,
    pub(super) status: String,
    pub(super) error: Option<String>,
    pub(super) protocol: Vec<ProtocolEntry>,
    pub(super) pending_new_game: bool,
    pub(super) search_purpose: Option<SearchPurpose>,
    pub(super) should_quit: bool,
}

impl App {
    /// new creates an application over a local game session.
    ///
    /// @param: game - local session used for validation and board snapshots
    /// @return: initialized disconnected application
    pub fn new(game: Box<dyn GameSession>) -> Self {
        Self::with_mode(game, GameMode::Analysis)
    }

    /// with_mode creates an application using the requested interaction mode.
    ///
    /// @param: game - local session used for validation and board snapshots
    /// @param: mode - analysis-only or play-against-engine behavior
    /// @return: initialized disconnected application
    pub fn with_mode(game: Box<dyn GameSession>, mode: GameMode) -> Self {
        let position = game.position();
        Self {
            game,
            mode,
            position,
            connection: ConnectionState::Disconnected,
            identity: EngineIdentity::default(),
            options: Vec::new(),
            analysis: Analysis::default(),
            moves: Vec::new(),
            cursor: Square::E2,
            selected: None,
            flipped: false,
            show_protocol: false,
            show_help: false,
            status: "Disconnected".to_owned(),
            error: None,
            protocol: Vec::new(),
            pending_new_game: false,
            search_purpose: None,
            should_quit: false,
        }
    }

    /// mode returns the configured interaction mode.
    ///
    /// @return: current game mode
    pub const fn mode(&self) -> GameMode {
        self.mode
    }

    /// position returns the board snapshot displayed by the application.
    ///
    /// @return: current board snapshot
    pub const fn position(&self) -> &PositionSnapshot {
        &self.position
    }

    /// connection returns the current UCI lifecycle state.
    ///
    /// @return: current connection state
    pub const fn connection(&self) -> ConnectionState {
        self.connection
    }

    /// identity returns the advertised engine identity.
    ///
    /// @return: engine identity
    pub const fn identity(&self) -> &EngineIdentity {
        &self.identity
    }

    /// options returns the advertised engine options.
    ///
    /// @return: engine options
    pub fn options(&self) -> &[UciOption] {
        &self.options
    }

    /// analysis returns the latest engine analysis.
    ///
    /// @return: latest analysis
    pub const fn analysis(&self) -> &Analysis {
        &self.analysis
    }

    /// moves returns the UCI move history.
    ///
    /// @return: ordered UCI moves
    pub fn moves(&self) -> &[String] {
        &self.moves
    }

    /// cursor returns the currently focused board square.
    ///
    /// @return: cursor square
    pub const fn cursor(&self) -> Square {
        self.cursor
    }

    /// selected returns the selected source square, when any.
    ///
    /// @return: selected source square
    pub const fn selected(&self) -> Option<Square> {
        self.selected
    }

    /// flipped reports whether Black is displayed at the bottom.
    ///
    /// @return: true when the board is flipped
    pub const fn flipped(&self) -> bool {
        self.flipped
    }

    /// show_protocol reports whether the raw protocol pane is visible.
    ///
    /// @return: true when the protocol pane is visible
    pub const fn show_protocol(&self) -> bool {
        self.show_protocol
    }

    /// show_help reports whether the help overlay is visible.
    ///
    /// @return: true when help is visible
    pub const fn show_help(&self) -> bool {
        self.show_help
    }

    /// status returns the current human-readable status.
    ///
    /// @return: status text
    pub fn status(&self) -> &str {
        &self.status
    }

    /// error returns the latest recoverable error.
    ///
    /// @return: latest error text
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    /// protocol returns the bounded raw protocol history.
    ///
    /// @return: protocol log entries
    pub fn protocol(&self) -> &[ProtocolEntry] {
        &self.protocol
    }

    /// should_quit reports whether the application requested exit.
    ///
    /// @return: true when exit was requested
    pub const fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// side_to_move returns the side expected to make a local move.
    ///
    /// @return: side to move
    pub const fn side_to_move(&self) -> Sides {
        self.position.side_to_move()
    }

    /// push_protocol appends a bounded protocol entry.
    ///
    /// @param: direction - source of the line
    /// @param: text - line content
    /// @return: void
    /// @side-effects: appends and may evict the oldest protocol entry
    pub(super) fn push_protocol(&mut self, direction: ProtocolDirection, text: impl Into<String>) {
        if self.protocol.len() == MAX_PROTOCOL_ENTRIES {
            self.protocol.remove(0);
        }
        self.protocol.push(ProtocolEntry {
            direction,
            text: text.into(),
        });
    }
}
