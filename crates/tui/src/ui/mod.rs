//! Immediate-mode terminal rendering.

mod analysis;
mod board;
mod help;
mod history;
mod protocol;
mod status;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};

use crate::App;

/// WIDE_LAYOUT_MINIMUM is the width at which the sidebar fits beside the board.
const WIDE_LAYOUT_MINIMUM: u16 = 76;

/// render draws one complete application frame.
///
/// @param: frame - destination terminal frame
/// @param: app - application state to render
/// @return: void
/// @side-effects: writes widgets into the frame buffer
pub fn render(frame: &mut Frame<'_>, app: &App) {
    let [header, content, footer] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(10),
        Constraint::Length(3),
    ])
    .areas(frame.area());
    status::render_header(frame, header, app);
    status::render_footer(frame, footer, app);

    if content.width >= WIDE_LAYOUT_MINIMUM {
        let [board, sidebar] =
            Layout::horizontal([Constraint::Length(29), Constraint::Min(30)]).areas(content);
        self::board::render(frame, board, app);
        render_sidebar(frame, sidebar, app);
    } else {
        let [board, sidebar] =
            Layout::vertical([Constraint::Length(12), Constraint::Min(6)]).areas(content);
        self::board::render(frame, board, app);
        render_sidebar(frame, sidebar, app);
    }

    if app.show_help() {
        help::render(frame);
    }
}

/// render_sidebar draws analysis or protocol output above move history.
///
/// @param: frame - destination terminal frame
/// @param: area - sidebar area
/// @param: app - application state to render
/// @return: void
/// @side-effects: writes sidebar widgets into the frame buffer
fn render_sidebar(frame: &mut Frame<'_>, area: ratatui::layout::Rect, app: &App) {
    let [primary, moves] =
        Layout::vertical([Constraint::Percentage(65), Constraint::Percentage(35)]).areas(area);
    if app.show_protocol() {
        protocol::render(frame, primary, app);
    } else {
        analysis::render(frame, primary, app);
    }
    history::render(frame, moves, app);
}
