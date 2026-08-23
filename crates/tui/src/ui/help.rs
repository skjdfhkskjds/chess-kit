use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use crate::{App, GameMode};

/// render draws a centered keyboard-help overlay.
///
/// @param: frame - destination terminal frame
/// @param: app - application state whose mode selects the visible controls
/// @return: void
/// @side-effects: clears and replaces the centered frame region
pub(super) fn render(frame: &mut Frame<'_>, app: &App) {
    let area = centered_rect(frame.area(), 58, 17);
    let mut controls = vec![
        ("Arrow keys / h j k l", "Move board cursor"),
        ("Enter", "Select or play a move"),
        ("Escape", "Cancel selection"),
    ];
    let (mode, title) = match app.mode() {
        GameMode::PlayVsEngine => (
            "Engine replies automatically at depth 6",
            " Keyboard help: Play vs engine ",
        ),
        GameMode::Analysis => {
            controls.push(("Space", "Start or stop analysis"));
            (
                "Move either side and analyze positions",
                " Keyboard help: Analysis ",
            )
        }
    };
    controls.extend([
        ("n", "Start a new game"),
        ("f", "Flip board"),
        ("p", "Toggle raw protocol"),
        ("?", "Close keyboard help"),
        ("q", "Quit"),
    ]);
    let lines = std::iter::once(Line::from(format!(" {mode}")))
        .chain(std::iter::once(Line::from("")))
        .chain(
            controls
                .into_iter()
                .map(|(key, action)| Line::from(format!(" {key:<23} {action}"))),
        )
        .collect::<Vec<_>>();
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(lines).block(
            Block::default()
                .title(title)
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
