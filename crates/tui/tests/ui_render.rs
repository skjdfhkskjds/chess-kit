use std::str::FromStr;

use chess_kit_engine::{EngineError, PositionSnapshot};
use chess_kit_primitives::{Black, Move, Pieces, Square, White};
use chess_kit_tui::{Action, App, EngineMessage, GameSession, RunnerEvent, render};
use ratatui::Terminal;
use ratatui::backend::TestBackend;

struct RenderGame {
    position: PositionSnapshot,
}

impl GameSession for RenderGame {
    fn new_game(&mut self) -> Result<(), EngineError> {
        Ok(())
    }

    fn play(&mut self, _: Move) -> Result<(), EngineError> {
        Ok(())
    }

    fn position(&self) -> PositionSnapshot {
        self.position.clone()
    }
}

#[test]
fn renders_board_identity_analysis_and_controls() {
    let mut app = populated_app();
    app.update(Action::Runner(RunnerEvent::Message(
        EngineMessage::from_str(
            "info depth 12 score cp 34 nodes 12000 nps 300000 time 40 pv e2e4 e7e5",
        )
        .unwrap(),
    )));
    let screen = render_to_string(&app, 100, 30);

    assert!(screen.contains("Fixture Engine"));
    assert!(screen.contains("Board"));
    assert!(screen.contains("Analysis"));
    assert!(screen.contains("Eval +0.34"));
    assert!(screen.contains("♙"));
    assert!(screen.contains("space analyze"));
}

#[test]
fn renders_a_narrow_fallback_without_panicking() {
    let app = populated_app();
    let screen = render_to_string(&app, 42, 28);

    assert!(screen.contains("Board"));
    assert!(screen.contains("Moves"));
}

#[test]
fn renders_help_and_protocol_overlays() {
    let mut app = populated_app();
    app.update(Action::ToggleProtocol);
    app.update(Action::ToggleHelp);
    let screen = render_to_string(&app, 100, 30);

    assert!(screen.contains("UCI protocol"));
    assert!(screen.contains("Keyboard help"));
    assert!(screen.contains("Toggle raw protocol"));
}

fn populated_app() -> App {
    let position = PositionSnapshot::empty::<White>()
        .with_piece::<White>(Square::E1, Pieces::King)
        .with_piece::<White>(Square::E2, Pieces::Pawn)
        .with_piece::<Black>(Square::E8, Pieces::King);
    let mut app = App::new(Box::new(RenderGame { position }));
    app.update(Action::Connect);
    app.update(Action::Runner(RunnerEvent::Message(
        EngineMessage::from_str("id name Fixture Engine").unwrap(),
    )));
    app.update(Action::Runner(RunnerEvent::Message(
        EngineMessage::from_str("uciok").unwrap(),
    )));
    app.update(Action::Runner(RunnerEvent::Message(
        EngineMessage::from_str("readyok").unwrap(),
    )));
    app
}

fn render_to_string(app: &App, width: u16, height: u16) -> String {
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| render(frame, app)).unwrap();
    terminal
        .backend()
        .buffer()
        .content()
        .iter()
        .map(|cell| cell.symbol())
        .collect()
}
