use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::App;

/// render draws move history in numbered full-move rows.
///
/// @param: frame - destination terminal frame
/// @param: area - history widget area
/// @param: app - application state to render
/// @return: void
/// @side-effects: writes the history widget into the frame buffer
pub(super) fn render(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let rows = app
        .moves()
        .chunks(2)
        .enumerate()
        .map(|(index, moves)| {
            let white = moves.first().map_or("", String::as_str);
            let black = moves.get(1).map_or("", String::as_str);
            Line::from(format!("{:>3}. {:<7} {black}", index + 1, white))
        })
        .collect::<Vec<_>>();
    let rows = if rows.is_empty() {
        vec![Line::from("No moves yet")]
    } else {
        rows
    };
    frame.render_widget(
        Paragraph::new(rows).block(Block::default().title(" Moves ").borders(Borders::ALL)),
        area,
    );
}
