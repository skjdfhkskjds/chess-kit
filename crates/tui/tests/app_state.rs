use std::str::FromStr;

use chess_kit_engine::{EngineError, PositionSnapshot};
use chess_kit_primitives::{Black, Move, Pieces, Sides, Square, White};
use chess_kit_tui::{
    Action, App, ConnectionState, Direction, Effect, EngineMessage, GameMode, GameSession,
    RunnerEvent, SearchRequest, UciCommand,
};

struct TestGame {
    position: PositionSnapshot,
    allowed_moves: Vec<Move>,
    has_legal_moves: bool,
    resets: usize,
}

impl TestGame {
    fn with_position(position: PositionSnapshot, allowed_move: Option<Move>) -> Self {
        Self {
            position,
            allowed_moves: allowed_move.into_iter().collect(),
            has_legal_moves: true,
            resets: 0,
        }
    }

    fn with_moves(position: PositionSnapshot, allowed_moves: Vec<Move>) -> Self {
        Self {
            position,
            allowed_moves,
            has_legal_moves: true,
            resets: 0,
        }
    }

    fn without_legal_moves(mut self) -> Self {
        self.has_legal_moves = false;
        self
    }
}

impl GameSession for TestGame {
    fn new_game(&mut self) -> Result<(), EngineError> {
        self.resets += 1;
        self.position =
            PositionSnapshot::empty::<White>().with_piece::<White>(Square::E2, Pieces::Pawn);
        Ok(())
    }

    fn play(&mut self, chess_move: Move) -> Result<(), EngineError> {
        if !self.allowed_moves.contains(&chess_move) {
            return Err(EngineError::new("illegal move"));
        }
        self.position = match self.position.side_to_move() {
            Sides::White => PositionSnapshot::empty::<Black>()
                .with_piece::<White>(chess_move.to(), Pieces::Pawn),
            Sides::Black => PositionSnapshot::empty::<White>()
                .with_piece::<Black>(chess_move.to(), Pieces::Pawn),
        };
        Ok(())
    }

    fn position(&self) -> PositionSnapshot {
        self.position.clone()
    }

    fn has_legal_moves(&self) -> bool {
        self.has_legal_moves
    }
}

#[test]
fn defaults_to_analysis_mode_and_accepts_an_explicit_play_mode() {
    let analysis = app_with_move(Move::new(Square::E2, Square::E4));
    assert_eq!(analysis.mode(), GameMode::Analysis);

    let game = TestGame::with_position(start_position(), Some(Move::new(Square::E2, Square::E4)));
    let play = App::with_mode(Box::new(game), GameMode::PlayVsEngine);
    assert_eq!(play.mode(), GameMode::PlayVsEngine);
}

#[test]
fn completes_the_uci_handshake_and_synchronizes_position() {
    let mut app = app_with_move(Move::new(Square::E2, Square::E4));

    assert_eq!(app.update(Action::Connect), [Effect::Send(UciCommand::Uci)]);
    assert_eq!(app.connection(), ConnectionState::AwaitingUci);

    let effects = engine_line(&mut app, "id name Fixture Engine");
    assert!(effects.is_empty());
    assert_eq!(app.identity().name.as_deref(), Some("Fixture Engine"));

    assert_eq!(
        engine_line(&mut app, "uciok"),
        [
            Effect::Send(UciCommand::UciNewGame),
            Effect::Send(UciCommand::IsReady),
        ]
    );
    assert_eq!(app.connection(), ConnectionState::AwaitingReady);

    let effects = engine_line(&mut app, "readyok");
    assert_eq!(
        effects,
        [Effect::Send(UciCommand::Position(
            chess_kit_tui::UciPosition::startpos(Vec::new())
        ))]
    );
    assert_eq!(app.connection(), ConnectionState::Ready);
}

#[test]
fn starts_stops_and_accumulates_streaming_analysis() {
    let mut app = ready_app(Move::new(Square::E2, Square::E4));

    assert_eq!(
        app.update(Action::ToggleAnalysis),
        [
            Effect::Send(UciCommand::Position(chess_kit_tui::UciPosition::startpos(
                Vec::new()
            ))),
            Effect::Send(UciCommand::Go(SearchRequest::Infinite)),
        ]
    );
    assert_eq!(app.connection(), ConnectionState::Searching);

    engine_line(&mut app, "info depth 8 score cp 21");
    engine_line(&mut app, "info nodes 420 nps 21000 pv e2e4 e7e5");
    assert_eq!(app.analysis().info.depth, Some(8));
    assert_eq!(app.analysis().info.nodes, Some(420));
    assert_eq!(app.analysis().info.principal_variation, ["e2e4", "e7e5"]);

    assert_eq!(
        app.update(Action::ToggleAnalysis),
        [Effect::Send(UciCommand::Stop)]
    );
    assert_eq!(app.connection(), ConnectionState::Stopping);
    engine_line(&mut app, "bestmove e2e4 ponder e7e5");
    assert_eq!(app.connection(), ConnectionState::Ready);
    assert_eq!(app.analysis().best_move.as_deref(), Some("e2e4"));
    assert_eq!(app.analysis().ponder.as_deref(), Some("e7e5"));

    app.update(Action::ToggleAnalysis);
    assert_eq!(app.analysis().info.depth, None);
    assert!(app.analysis().info.principal_variation.is_empty());
    assert_eq!(app.analysis().best_move, None);
}

#[test]
fn analysis_bestmove_is_display_only() {
    let mut app = ready_app(Move::new(Square::E2, Square::E4));
    app.update(Action::ToggleAnalysis);

    assert!(engine_line(&mut app, "bestmove e2e4 ponder e7e5").is_empty());
    assert_eq!(app.connection(), ConnectionState::Ready);
    assert_eq!(app.analysis().best_move.as_deref(), Some("e2e4"));
    assert_eq!(app.analysis().ponder.as_deref(), Some("e7e5"));
    assert!(app.moves().is_empty());
    assert_eq!(app.side_to_move(), Sides::White);
}

#[test]
fn queues_a_new_game_until_search_returns_best_move() {
    let mut app = ready_app(Move::new(Square::E2, Square::E4));
    app.update(Action::ToggleAnalysis);

    assert_eq!(
        app.update(Action::NewGame),
        [Effect::Send(UciCommand::Stop)]
    );
    assert_eq!(app.connection(), ConnectionState::Stopping);
    assert_eq!(
        engine_line(&mut app, "bestmove e2e4"),
        [
            Effect::Send(UciCommand::UciNewGame),
            Effect::Send(UciCommand::IsReady),
        ]
    );
    assert_eq!(app.connection(), ConnectionState::AwaitingReady);
    assert!(app.moves().is_empty());
}

#[test]
fn validates_selected_moves_and_synchronizes_history() {
    let expected = Move::new(Square::E2, Square::E4);
    let mut app = ready_app(expected);

    app.update(Action::SelectSquare);
    app.update(Action::MoveCursor(Direction::Up));
    app.update(Action::MoveCursor(Direction::Up));
    assert_eq!(app.cursor(), Square::E4);
    assert_eq!(
        app.update(Action::SelectSquare),
        [Effect::Send(UciCommand::Position(
            chess_kit_tui::UciPosition::startpos(vec!["e2e4".to_owned()])
        ))]
    );
    assert_eq!(app.moves(), ["e2e4"]);
    assert_eq!(app.side_to_move(), chess_kit_primitives::Sides::Black);
    assert_eq!(app.selected(), None);
}

#[test]
fn rejects_illegal_moves_without_losing_the_selection() {
    let mut app = ready_app(Move::new(Square::E2, Square::E4));

    app.update(Action::SelectSquare);
    app.update(Action::MoveCursor(Direction::Right));
    assert!(app.update(Action::SelectSquare).is_empty());

    assert_eq!(app.selected(), Some(Square::E2));
    assert_eq!(app.error(), Some("illegal move"));
    assert!(app.moves().is_empty());
}

#[test]
fn automatically_uses_queen_promotion() {
    let promotion = Move::new(Square::A7, Square::A8).with_promotion(Pieces::Queen);
    let game = TestGame::with_position(
        PositionSnapshot::empty::<White>().with_piece::<White>(Square::A7, Pieces::Pawn),
        Some(promotion),
    );
    let mut app = App::new(Box::new(game));
    complete_handshake(&mut app);
    for _ in 0..4 {
        app.update(Action::MoveCursor(Direction::Left));
    }
    for _ in 0..5 {
        app.update(Action::MoveCursor(Direction::Up));
    }
    assert_eq!(app.cursor(), Square::A7);

    app.update(Action::SelectSquare);
    app.update(Action::MoveCursor(Direction::Up));
    app.update(Action::SelectSquare);

    assert_eq!(app.moves(), ["a7a8q"]);
}

#[test]
fn play_mode_searches_for_black_after_a_legal_human_move() {
    let human_move = Move::new(Square::E2, Square::E4);
    let mut app = ready_play_app(vec![human_move]);

    let effects = select_e2e4(&mut app);

    assert_eq!(app.moves(), ["e2e4"]);
    assert_eq!(app.side_to_move(), Sides::Black);
    assert_eq!(app.connection(), ConnectionState::Searching);
    assert_eq!(app.status(), "Engine is thinking");
    assert_eq!(
        effects,
        [
            Effect::Send(UciCommand::Position(chess_kit_tui::UciPosition::startpos(
                vec!["e2e4".to_owned()]
            ))),
            Effect::Send(UciCommand::Go(SearchRequest::Depth(6))),
        ]
    );
}

#[test]
fn play_mode_applies_valid_engine_reply_and_resynchronizes() {
    let mut app = ready_play_app(vec![
        Move::new(Square::E2, Square::E4),
        Move::new(Square::E7, Square::E5),
    ]);
    select_e2e4(&mut app);

    assert_eq!(
        engine_line(&mut app, "bestmove e7e5 ponder g1f3"),
        [Effect::Send(UciCommand::Position(
            chess_kit_tui::UciPosition::startpos(vec!["e2e4".to_owned(), "e7e5".to_owned()])
        ))]
    );
    assert_eq!(app.moves(), ["e2e4", "e7e5"]);
    assert_eq!(app.side_to_move(), Sides::White);
    assert_eq!(app.connection(), ConnectionState::Ready);
    assert_eq!(app.analysis().best_move.as_deref(), Some("e7e5"));
    assert_eq!(app.analysis().ponder.as_deref(), Some("g1f3"));
    assert_eq!(app.status(), "Your move");
    assert_eq!(app.error(), None);
}

#[test]
fn play_mode_rejects_malformed_and_illegal_engine_replies() {
    let mut malformed = ready_play_app(vec![Move::new(Square::E2, Square::E4)]);
    select_e2e4(&mut malformed);
    assert!(engine_line(&mut malformed, "bestmove nonsense").is_empty());
    assert_eq!(malformed.moves(), ["e2e4"]);
    assert_eq!(malformed.connection(), ConnectionState::Ready);
    assert!(
        malformed
            .error()
            .unwrap()
            .contains("malformed move nonsense")
    );

    let mut illegal = ready_play_app(vec![
        Move::new(Square::E2, Square::E4),
        Move::new(Square::E7, Square::E5),
    ]);
    select_e2e4(&mut illegal);
    assert!(engine_line(&mut illegal, "bestmove e7e6").is_empty());
    assert_eq!(illegal.moves(), ["e2e4"]);
    assert!(illegal.error().unwrap().contains("illegal move e7e6"));
}

#[test]
fn play_mode_accepts_null_bestmove_only_at_game_over() {
    let mut invalid = ready_play_app(vec![Move::new(Square::E2, Square::E4)]);
    select_e2e4(&mut invalid);
    engine_line(&mut invalid, "bestmove 0000");
    assert_eq!(
        invalid.error(),
        Some("Engine returned no move in a position with legal moves")
    );

    let game = TestGame::with_moves(start_position(), vec![Move::new(Square::E2, Square::E4)])
        .without_legal_moves();
    let mut terminal = App::with_mode(Box::new(game), GameMode::PlayVsEngine);
    complete_handshake(&mut terminal);
    select_e2e4(&mut terminal);
    engine_line(&mut terminal, "bestmove (none)");
    assert_eq!(terminal.error(), None);
    assert_eq!(terminal.status(), "Game over");
    assert_eq!(terminal.moves(), ["e2e4"]);
}

#[test]
fn play_mode_discards_engine_reply_when_new_game_stops_search() {
    let mut app = ready_play_app(vec![
        Move::new(Square::E2, Square::E4),
        Move::new(Square::E7, Square::E5),
    ]);
    select_e2e4(&mut app);

    assert_eq!(
        app.update(Action::NewGame),
        [Effect::Send(UciCommand::Stop)]
    );
    assert_eq!(
        engine_line(&mut app, "bestmove e7e5"),
        [
            Effect::Send(UciCommand::UciNewGame),
            Effect::Send(UciCommand::IsReady),
        ]
    );
    assert!(app.moves().is_empty());
    assert_eq!(app.analysis().best_move, None);
}

#[test]
fn play_mode_gates_board_selection_until_it_is_whites_turn() {
    let mut app = ready_play_app(vec![Move::new(Square::E2, Square::E4)]);
    select_e2e4(&mut app);
    engine_line(&mut app, "bestmove malformed");
    assert_eq!(app.connection(), ConnectionState::Ready);
    assert_eq!(app.side_to_move(), Sides::Black);

    app.update(Action::SelectSquare);
    assert_eq!(app.selected(), None);
    assert_eq!(app.moves(), ["e2e4"]);
}

fn app_with_move(allowed_move: Move) -> App {
    let position = start_position();
    App::new(Box::new(TestGame::with_position(
        position,
        Some(allowed_move),
    )))
}

fn ready_play_app(allowed_moves: Vec<Move>) -> App {
    let mut app = App::with_mode(
        Box::new(TestGame::with_moves(start_position(), allowed_moves)),
        GameMode::PlayVsEngine,
    );
    complete_handshake(&mut app);
    app
}

fn start_position() -> PositionSnapshot {
    PositionSnapshot::empty::<White>().with_piece::<White>(Square::E2, Pieces::Pawn)
}

fn select_e2e4(app: &mut App) -> Vec<Effect> {
    app.update(Action::SelectSquare);
    app.update(Action::MoveCursor(Direction::Up));
    app.update(Action::MoveCursor(Direction::Up));
    app.update(Action::SelectSquare)
}

fn ready_app(allowed_move: Move) -> App {
    let mut app = app_with_move(allowed_move);
    complete_handshake(&mut app);
    app
}

fn complete_handshake(app: &mut App) {
    app.update(Action::Connect);
    engine_line(app, "uciok");
    engine_line(app, "readyok");
}

fn engine_line(app: &mut App, line: &str) -> Vec<Effect> {
    let message = EngineMessage::from_str(line).unwrap();
    app.update(Action::Runner(RunnerEvent::Message(message)))
}
