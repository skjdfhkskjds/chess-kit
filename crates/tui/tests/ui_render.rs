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
    let narrow = render_to_string(&app, 42, 28);
    let below_breakpoint = render_to_string(&app, 75, 28);
    let at_breakpoint = render_to_string(&app, 76, 28);

    assert!(narrow.contains("Board"));
    assert!(narrow.contains("Moves"));
    assert!(below_breakpoint.contains("Board"));
    assert!(at_breakpoint.contains("Board"));
}

#[test]
fn gives_the_board_most_of_a_wide_layout() {
    let app = populated_app();
    let screen = render_to_lines(&app, 120, 30);
    let content_border = &screen[3];
    let divider = content_border
        .find(" Analysis ")
        .expect("wide layout has an analysis pane");

    assert!(divider >= 79, "board pane ended at column {divider}");
}

#[test]
fn scales_pieces_when_board_cells_are_large_enough() {
    let app = populated_app();
    let screen = render_to_string(&app, 240, 54);

    assert!(screen.contains(['▀', '▄', '█']));
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

#[test]
fn escapes_engine_control_characters_before_rendering() {
    let mut app = populated_app();
    app.update(Action::Runner(RunnerEvent::Message(
        EngineMessage::from_str("id name unsafe\u{1b}[2J\u{7}").unwrap(),
    )));
    app.update(Action::ToggleProtocol);

    let screen = render_to_string(&app, 100, 30);

    assert!(!screen.contains('\u{1b}'));
    assert!(!screen.contains('\u{7}'));
    assert!(screen.contains("\\x1b"));
}

#[test]
fn renders_an_explicit_too_small_message() {
    let app = populated_app();
    let screen = render_to_string(&app, 30, 8);

    assert!(screen.contains("Terminal too small"));
    assert!(!screen.contains("Board"));
}

#[test]
fn exposes_selection_without_relying_only_on_color() {
    let mut app = populated_app();
    app.update(Action::SelectSquare);
    let screen = render_to_string(&app, 100, 30);

    assert!(screen.contains("sel e2"));
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
    render_to_lines(app, width, height).concat()
}

fn render_to_lines(app: &App, width: u16, height: u16) -> Vec<String> {
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| render(frame, app)).unwrap();
    terminal
        .backend()
        .buffer()
        .content()
        .chunks(width as usize)
        .map(|row| row.iter().map(|cell| cell.symbol()).collect())
        .collect()
}
