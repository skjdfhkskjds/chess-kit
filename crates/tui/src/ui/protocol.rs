use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use super::text::sanitize;
use crate::{App, ProtocolDirection};

/// render draws the tail of the raw UCI protocol transcript.
///
/// @param: frame - destination terminal frame
/// @param: area - protocol widget area
/// @param: app - application state to render
/// @return: void
/// @side-effects: writes the protocol widget into the frame buffer
pub(super) fn render(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let visible = usize::from(area.height.saturating_sub(2));
    let start = app.protocol().len().saturating_sub(visible);
    let lines = app.protocol()[start..]
        .iter()
        .map(|entry| {
            let (prefix, color) = match entry.direction {
                ProtocolDirection::Gui => (">", Color::Cyan),
                ProtocolDirection::Engine => ("<", Color::Green),
                ProtocolDirection::StandardError => ("!", Color::Yellow),
                ProtocolDirection::System => ("×", Color::Red),
            };
            Line::styled(
                format!("{prefix} {}", sanitize(&entry.text)),
                Style::default().fg(color),
            )
        })
        .collect::<Vec<_>>();
    frame.render_widget(
        Paragraph::new(lines).wrap(Wrap { trim: true }).block(
            Block::default()
                .title(" UCI protocol ")
                .borders(Borders::ALL),
        ),
        area,
    );
}
