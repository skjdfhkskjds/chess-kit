use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::App;

/// render_header draws engine identity and lifecycle status.
///
/// @param: frame - destination terminal frame
/// @param: area - header widget area
/// @param: app - application state to render
/// @return: void
/// @side-effects: writes the header widget into the frame buffer
pub(super) fn render_header(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let engine = app.identity().name.as_deref().unwrap_or("UCI engine");
    let author = app
        .identity()
        .author
        .as_deref()
        .map_or_else(String::new, |author| format!(" by {author}"));
    let line = Line::from(vec![
        Span::styled(
            " chess-kit ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!("  {engine}{author}  │  {}", app.status())),
    ]);
    frame.render_widget(
        Paragraph::new(line).block(Block::default().borders(Borders::ALL)),
        area,
    );
}

/// render_footer draws controls or the latest application error.
///
/// @param: frame - destination terminal frame
/// @param: area - footer widget area
/// @param: app - application state to render
/// @return: void
/// @side-effects: writes the footer widget into the frame buffer
pub(super) fn render_footer(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let line = app.error().map_or_else(
        || {
            Line::from(
                " arrows/hjkl move  enter select  space analyze  n new  f flip  p protocol  ? help  q quit ",
            )
        },
        |error| Line::styled(format!(" {error} "), Style::default().fg(Color::Red)),
    );
    frame.render_widget(
        Paragraph::new(line).block(Block::default().borders(Borders::ALL)),
        area,
    );
}
