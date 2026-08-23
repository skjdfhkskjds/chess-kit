use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

/// render draws a centered keyboard-help overlay.
///
/// @param: frame - destination terminal frame
/// @return: void
/// @side-effects: clears and replaces the centered frame region
pub(super) fn render(frame: &mut Frame<'_>) {
    let area = centered_rect(frame.area(), 58, 17);
    let lines = [
        ("Arrow keys / h j k l", "Move board cursor"),
        ("Enter", "Select or play a move"),
        ("Escape", "Cancel selection"),
        ("Space", "Start or stop analysis"),
        ("n", "Start a new game"),
        ("f", "Flip board"),
        ("p", "Toggle raw protocol"),
        ("?", "Close keyboard help"),
        ("q", "Quit"),
    ]
    .into_iter()
    .map(|(key, action)| Line::from(format!(" {key:<23} {action}")))
    .collect::<Vec<_>>();
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(lines).block(
            Block::default()
                .title(" Keyboard help ")
                .title_style(Style::default().fg(Color::Cyan))
                .borders(Borders::ALL),
        ),
        area,
    );
}

/// centered_rect creates a fixed-size rectangle clamped to its container.
///
/// @param: area - containing rectangle
/// @param: width - preferred width
/// @param: height - preferred height
/// @return: centered rectangle
fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let width = width.min(area.width);
    let height = height.min(area.height);
    let [vertical] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    let [centered] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(vertical);
    centered
}
