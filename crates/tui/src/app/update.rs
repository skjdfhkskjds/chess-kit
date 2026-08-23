use std::str::FromStr;

use chess_kit_comm::uci::UciMove;
use chess_kit_primitives::{File, Move, Pieces, Rank, Sides, Square};

use super::state::SearchPurpose;
use super::{Action, App, ConnectionState, Direction, Effect, GameMode, ProtocolDirection};
use crate::{
    EngineMessageKind, IdentityField, RunnerEvent, SearchInfo, SearchRequest, UciCommand,
    UciPosition,
};

/// ENGINE_PLAY_DEPTH is the fixed search depth for automatic Black replies.
const ENGINE_PLAY_DEPTH: u16 = 6;

impl App {
    /// update consumes one action and returns its requested runtime effects.
    ///
    /// @param: action - user or engine action to consume
    /// @return: ordered runtime effects
    /// @side-effects: updates application and local game state
    pub fn update(&mut self, action: Action) -> Vec<Effect> {
        match action {
            Action::Connect => self.connect(),
            Action::MoveCursor(direction) => {
                self.move_cursor(direction);
                Vec::new()
            }
            Action::SelectSquare => self.select_square(),
            Action::CancelSelection => {
                self.selected = None;
                self.error = None;
                Vec::new()
            }
            Action::ToggleAnalysis => self.toggle_analysis(),
            Action::FlipBoard => {
                self.flipped = !self.flipped;
                Vec::new()
            }
            Action::NewGame => self.new_game(),
            Action::ToggleProtocol => {
                self.show_protocol = !self.show_protocol;
                Vec::new()
            }
            Action::ToggleHelp => {
                self.show_help = !self.show_help;
                Vec::new()
            }
            Action::Runner(event) => self.handle_runner_event(event),
            Action::TransportFailed(error) => {
                self.fail(error);
                Vec::new()
            }
            Action::Quit => {
                self.should_quit = true;
                vec![Effect::Quit]
            }
        }
    }

    /// connect starts the UCI initialization handshake.
    ///
    /// @return: UCI initialization effect when disconnected
    /// @side-effects: updates connection state and protocol history
    fn connect(&mut self) -> Vec<Effect> {
        if self.connection != ConnectionState::Disconnected {
            return Vec::new();
        }
        self.connection = ConnectionState::AwaitingUci;
        self.status = "Waiting for UCI handshake".to_owned();
        vec![self.send(UciCommand::Uci)]
    }

    /// handle_runner_event consumes one child-process event.
    ///
    /// @param: event - runner event to consume
    /// @return: effects produced by the event
    /// @side-effects: updates connection, analysis, and protocol state
    fn handle_runner_event(&mut self, event: RunnerEvent) -> Vec<Effect> {
        match event {
            RunnerEvent::Message(message) => {
                self.push_protocol(ProtocolDirection::Engine, message.raw());
                self.handle_engine_message(message.kind().clone())
            }
            RunnerEvent::StandardError(line) => {
                self.push_protocol(ProtocolDirection::StandardError, line);
                Vec::new()
            }
            RunnerEvent::Error(error) => {
                self.fail(error);
                Vec::new()
            }
            RunnerEvent::Exited(code) => {
                self.connection = ConnectionState::Exited;
                self.search_purpose = None;
                self.status = match code {
                    Some(code) => format!("Engine exited with status {code}"),
                    None => "Engine exited".to_owned(),
                };
                Vec::new()
            }
        }
    }

    /// handle_engine_message applies one parsed engine message.
    ///
    /// @param: kind - parsed message meaning
    /// @return: effects produced by the message
    /// @side-effects: updates engine metadata, lifecycle, and analysis
    fn handle_engine_message(&mut self, kind: EngineMessageKind) -> Vec<Effect> {
        match kind {
            EngineMessageKind::Id { field, value } => {
                match field {
                    IdentityField::Name => self.identity.name = Some(value),
                    IdentityField::Author => self.identity.author = Some(value),
                }
                Vec::new()
            }
            EngineMessageKind::Option(option) => {
                self.options.push(option);
                Vec::new()
            }
            EngineMessageKind::UciOk if self.connection == ConnectionState::AwaitingUci => {
                self.connection = ConnectionState::AwaitingReady;
                self.status = "Waiting for engine readiness".to_owned();
                vec![
                    self.send(UciCommand::UciNewGame),
                    self.send(UciCommand::IsReady),
                ]
            }
            EngineMessageKind::ReadyOk if self.connection == ConnectionState::AwaitingReady => {
                self.connection = ConnectionState::Ready;
                self.status = match self.mode {
                    GameMode::Analysis => "Ready".to_owned(),
                    GameMode::PlayVsEngine => "Your move".to_owned(),
                };
                self.pending_new_game = false;
                self.search_purpose = None;
                vec![self.sync_position()]
            }
            EngineMessageKind::Info(info) => {
                merge_info(&mut self.analysis.info, info);
                Vec::new()
            }
            EngineMessageKind::BestMove { best_move, ponder } => {
                let purpose = self.search_purpose.take();
                if matches!(
                    self.connection,
                    ConnectionState::Searching | ConnectionState::Stopping
                ) {
                    self.connection = ConnectionState::Ready;
                    self.status = "Ready".to_owned();
                }
                if self.pending_new_game {
                    self.reset_game()
                } else {
                    match purpose {
                        Some(SearchPurpose::Analysis) => {
                            self.analysis.best_move = best_move;
                            self.analysis.ponder = ponder;
                            Vec::new()
                        }
                        Some(SearchPurpose::EngineMove) => {
                            self.apply_engine_move(best_move, ponder)
                        }
                        None => Vec::new(),
                    }
                }
            }
            EngineMessageKind::UciOk | EngineMessageKind::ReadyOk | EngineMessageKind::Unknown => {
                Vec::new()
            }
        }
    }

    /// toggle_analysis starts an infinite search or requests its completion.
    ///
    /// @return: search effects
    /// @side-effects: updates connection and analysis state
    fn toggle_analysis(&mut self) -> Vec<Effect> {
        if self.mode != GameMode::Analysis {
            return Vec::new();
        }
        match self.connection {
            ConnectionState::Ready => {
                self.connection = ConnectionState::Searching;
                self.search_purpose = Some(SearchPurpose::Analysis);
                self.status = "Searching".to_owned();
                self.analysis = Default::default();
                vec![
                    self.sync_position(),
                    self.send(UciCommand::Go(SearchRequest::Infinite)),
                ]
            }
            ConnectionState::Searching => {
                self.connection = ConnectionState::Stopping;
                self.status = "Stopping search".to_owned();
                vec![self.send(UciCommand::Stop)]
            }
            _ => Vec::new(),
        }
    }

    /// new_game resets immediately or queues reset after an active search.
    ///
    /// @return: stop or reset effects
    /// @side-effects: may update local game and connection state
    fn new_game(&mut self) -> Vec<Effect> {
        match self.connection {
            ConnectionState::Searching => {
                self.pending_new_game = true;
                self.connection = ConnectionState::Stopping;
                self.status = "Stopping search for new game".to_owned();
                vec![self.send(UciCommand::Stop)]
            }
            ConnectionState::Stopping => {
                self.pending_new_game = true;
                Vec::new()
            }
            ConnectionState::Ready => self.reset_game(),
            _ => Vec::new(),
        }
    }

    /// reset_game clears local state and starts a synchronized UCI game.
    ///
    /// @return: new-game and readiness effects
    /// @side-effects: resets local game and application state
    fn reset_game(&mut self) -> Vec<Effect> {
        if let Err(error) = self.game.new_game() {
            self.error = Some(error.to_string());
            return Vec::new();
        }
        self.position = self.game.position();
        self.moves.clear();
        self.selected = None;
        self.analysis = Default::default();
        self.pending_new_game = false;
        self.search_purpose = None;
        self.connection = ConnectionState::AwaitingReady;
        self.status = "Starting new game".to_owned();
        vec![
            self.send(UciCommand::UciNewGame),
            self.send(UciCommand::IsReady),
        ]
    }

    /// select_square selects a source or attempts a destination move.
    ///
    /// @return: position synchronization effect after a legal move
    /// @side-effects: may update local game, board, history, and selection
    fn select_square(&mut self) -> Vec<Effect> {
        if self.connection != ConnectionState::Ready
            || (self.mode == GameMode::PlayVsEngine && self.position.side_to_move() != Sides::White)
        {
            return Vec::new();
        }
        let Some(from) = self.selected else {
            if self
                .position
                .piece_at(self.cursor)
                .is_some_and(|(side, _)| side == self.position.side_to_move())
            {
                self.selected = Some(self.cursor);
                self.error = None;
            } else {
                self.error = Some("Select a piece belonging to the side to move".to_owned());
            }
            return Vec::new();
        };

        if from == self.cursor {
            self.selected = None;
            return Vec::new();
        }

        let mut chess_move = Move::new(from, self.cursor);
        if self
            .position
            .piece_at(from)
            .is_some_and(|(_, piece)| piece == Pieces::Pawn)
            && matches!(self.cursor.rank(), Rank::R1 | Rank::R8)
        {
            chess_move = chess_move.with_promotion(Pieces::Queen);
        }
        match self.game.play(chess_move) {
            Ok(()) => {
                self.moves.push(UciMove::from(chess_move).to_string());
                self.position = self.game.position();
                self.analysis = Default::default();
                self.selected = None;
                self.error = None;
                if self.mode == GameMode::PlayVsEngine {
                    self.connection = ConnectionState::Searching;
                    self.search_purpose = Some(SearchPurpose::EngineMove);
                    self.status = "Engine is thinking".to_owned();
                    vec![
                        self.sync_position(),
                        self.send(UciCommand::Go(SearchRequest::Depth(ENGINE_PLAY_DEPTH))),
                    ]
                } else {
                    self.status = "Position updated".to_owned();
                    vec![self.sync_position()]
                }
            }
            Err(error) => {
                self.error = Some(error.to_string());
                Vec::new()
            }
        }
    }

    /// apply_engine_move validates and applies the Black reply from a play search.
    ///
    /// @param: best_move - parsed protocol text, or None for a null move
    /// @param: ponder - optional engine ponder move for display
    /// @return: position synchronization after a valid reply
    /// @side-effects: may update local game, board, history, analysis, and error state
    fn apply_engine_move(
        &mut self,
        best_move: Option<String>,
        ponder: Option<String>,
    ) -> Vec<Effect> {
        self.analysis.best_move = best_move.clone();
        self.analysis.ponder = ponder;

        let Some(best_move) = best_move else {
            if self.game.has_legal_moves() {
                self.error =
                    Some("Engine returned no move in a position with legal moves".to_owned());
                self.status = "Invalid engine response".to_owned();
            } else {
                self.error = None;
                self.status = "Game over".to_owned();
            }
            return Vec::new();
        };

        let uci_move = match UciMove::from_str(&best_move) {
            Ok(uci_move) => uci_move,
            Err(error) => {
                self.error = Some(format!(
                    "Engine returned malformed move {best_move}: {error}"
                ));
                self.status = "Invalid engine response".to_owned();
                return Vec::new();
            }
        };
        let chess_move = match Move::try_from(&uci_move) {
            Ok(chess_move) => chess_move,
            Err(_) => {
                if self.game.has_legal_moves() {
                    self.error =
                        Some("Engine returned no move in a position with legal moves".to_owned());
                    self.status = "Invalid engine response".to_owned();
                } else {
                    self.error = None;
                    self.status = "Game over".to_owned();
                }
                return Vec::new();
            }
        };

        if let Err(error) = self.game.play(chess_move) {
            self.error = Some(format!("Engine returned illegal move {best_move}: {error}"));
            self.status = "Invalid engine response".to_owned();
            return Vec::new();
        }

        self.moves.push(UciMove::from(chess_move).to_string());
        self.position = self.game.position();
        self.selected = None;
        self.error = None;
        self.status = if self.game.has_legal_moves() {
            "Your move".to_owned()
        } else {
            "Game over".to_owned()
        };
        vec![self.sync_position()]
    }

    /// move_cursor moves the logical cursor using screen-relative direction.
    ///
    /// @param: direction - requested screen direction
    /// @return: void
    /// @side-effects: updates cursor within board bounds
    fn move_cursor(&mut self, direction: Direction) {
        let (mut file_delta, mut rank_delta) = match direction {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        if self.flipped {
            file_delta *= -1;
            rank_delta *= -1;
        }
        let file = (self.cursor.file().idx() as i8 + file_delta).clamp(0, 7) as usize;
        let rank = (self.cursor.rank().idx() as i8 + rank_delta).clamp(0, 7) as usize;
        self.cursor = Square::new(File::from_idx(file), Rank::from_idx(rank));
    }

    /// sync_position creates the current full-history position command.
    ///
    /// @return: outbound position effect
    /// @side-effects: appends the outbound command to protocol history
    fn sync_position(&mut self) -> Effect {
        self.send(UciCommand::Position(UciPosition::startpos(
            self.moves.clone(),
        )))
    }

    /// send records and wraps an outbound command.
    ///
    /// @param: command - UCI command to request
    /// @return: send effect
    /// @side-effects: appends the command to protocol history
    fn send(&mut self, command: UciCommand) -> Effect {
        self.push_protocol(ProtocolDirection::Gui, command.to_string());
        Effect::Send(command)
    }

    /// fail records a terminal transport failure.
    ///
    /// @param: error - failure text
    /// @return: void
    /// @side-effects: marks the connection failed
    fn fail(&mut self, error: String) {
        self.push_protocol(ProtocolDirection::System, error.clone());
        self.connection = ConnectionState::Failed;
        self.search_purpose = None;
        self.status = "Engine connection failed".to_owned();
        self.error = Some(error);
    }
}

/// merge_info overlays fields reported by one partial UCI info message.
///
/// @param: current - accumulated search information
/// @param: update - newly reported fields
/// @return: void
/// @side-effects: replaces fields present in update
fn merge_info(current: &mut SearchInfo, update: SearchInfo) {
    current.depth = update.depth.or(current.depth);
    current.selective_depth = update.selective_depth.or(current.selective_depth);
    current.elapsed = update.elapsed.or(current.elapsed);
    current.nodes = update.nodes.or(current.nodes);
    current.nodes_per_second = update.nodes_per_second.or(current.nodes_per_second);
    current.hash_full = update.hash_full.or(current.hash_full);
    current.multi_pv = update.multi_pv.or(current.multi_pv);
    current.score = update.score.or(current.score);
    current.current_move = update.current_move.or_else(|| current.current_move.take());
    if !update.principal_variation.is_empty() {
        current.principal_variation = update.principal_variation;
    }
    current.text = update.text.or_else(|| current.text.take());
}
