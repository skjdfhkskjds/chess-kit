use std::str::FromStr;

use chess_kit_engine::{EngineError, PositionSnapshot};
use chess_kit_primitives::{Black, Move, Pieces, Square, White};
use chess_kit_tui::{
    Action, App, ConnectionState, Direction, Effect, EngineMessage, GameSession, RunnerEvent,
    SearchRequest, UciCommand,
};

struct TestGame {
    position: PositionSnapshot,
    allowed_move: Option<Move>,
    resets: usize,
}

impl TestGame {
    fn with_position(position: PositionSnapshot, allowed_move: Option<Move>) -> Self {
        Self {
            position,
            allowed_move,
            resets: 0,
        }
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
        if self.allowed_move != Some(chess_move) {
            return Err(EngineError::new("illegal move"));
        }
        self.position =
            PositionSnapshot::empty::<Black>().with_piece::<White>(chess_move.to(), Pieces::Pawn);
        Ok(())
    }

    fn position(&self) -> PositionSnapshot {
        self.position.clone()
    }
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

fn app_with_move(allowed_move: Move) -> App {
    let position = PositionSnapshot::empty::<White>().with_piece::<White>(Square::E2, Pieces::Pawn);
    App::new(Box::new(TestGame::with_position(
        position,
        Some(allowed_move),
    )))
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
