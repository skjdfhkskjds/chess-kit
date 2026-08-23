//! Immediate-mode terminal rendering.

mod analysis;
mod board;
mod cell;
mod help;
mod history;
mod pieces;
mod protocol;
mod status;
mod text;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::App;

pub use pieces::PieceSet;

/// WIDE_LAYOUT_MINIMUM is the width at which the sidebar fits beside the board.
const WIDE_LAYOUT_MINIMUM: u16 = 76;

/// SIDEBAR_WIDTH_PERCENT keeps the board as the dominant wide-layout pane.
const SIDEBAR_WIDTH_PERCENT: u16 = 33;

/// SIDEBAR_MINIMUM_WIDTH preserves enough room for analysis labels and values.
const SIDEBAR_MINIMUM_WIDTH: u16 = 30;

/// MINIMUM_WIDTH is the smallest supported full interface width.
const MINIMUM_WIDTH: u16 = 40;

/// MINIMUM_HEIGHT is the smallest supported full interface height.
const MINIMUM_HEIGHT: u16 = 18;

/// render draws one complete application frame.
///
/// @param: frame - destination terminal frame
/// @param: app - application state to render
/// @return: void
/// @side-effects: writes widgets into the frame buffer
pub fn render(frame: &mut Frame<'_>, app: &App) {
    render_with_piece_set(frame, app, PieceSet::default());
}

/// render_with_piece_set draws one frame with explicitly selected piece assets.
///
/// @param: frame - destination terminal frame
/// @param: app - application state to render
/// @param: piece_set - assets used to draw pieces
/// @return: void
/// @side-effects: writes widgets into the frame buffer
pub fn render_with_piece_set(frame: &mut Frame<'_>, app: &App, piece_set: PieceSet) {
    if frame.area().width < MINIMUM_WIDTH || frame.area().height < MINIMUM_HEIGHT {
        frame.render_widget(
            Paragraph::new(format!(
                "Terminal too small. Resize to at least {MINIMUM_WIDTH}x{MINIMUM_HEIGHT}."
            ))
            .wrap(Wrap { trim: true })
            .block(Block::default().title(" chess-kit ").borders(Borders::ALL)),
            frame.area(),
        );
        return;
    }

    let [header, content, footer] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(10),
        Constraint::Length(3),
    ])
    .areas(frame.area());
    status::render_header(frame, header, app);
    status::render_footer(frame, footer, app);

    if content.width >= WIDE_LAYOUT_MINIMUM {
        let sidebar_width =
            (u32::from(content.width) * u32::from(SIDEBAR_WIDTH_PERCENT)).div_ceil(100) as u16;
        let sidebar_width = sidebar_width.max(SIDEBAR_MINIMUM_WIDTH);
        let [board, sidebar] = Layout::horizontal([
            Constraint::Min(content.width.saturating_sub(sidebar_width)),
            Constraint::Length(sidebar_width),
        ])
        .areas(content);
        self::board::render(frame, board, app, piece_set);
        render_sidebar(frame, sidebar, app);
    } else {
        let [board, sidebar] =
            Layout::vertical([Constraint::Length(12), Constraint::Min(6)]).areas(content);
        self::board::render(frame, board, app, piece_set);
        render_sidebar(frame, sidebar, app);
    }

    if app.show_help() {
        help::render(frame, app);
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
