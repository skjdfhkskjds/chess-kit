use chess_kit_primitives::Square;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use super::cell::Cell;
use super::pieces::PieceSet;
use crate::App;

/// render draws the chess board with cursor and selection highlighting.
///
/// @param: frame - destination terminal frame
/// @param: area - board widget area
/// @param: app - application state to render
/// @param: piece_set - assets used to draw occupied cells
/// @return: void
/// @side-effects: writes the board widget into the frame buffer
pub(super) fn render(frame: &mut Frame<'_>, area: Rect, app: &App, piece_set: PieceSet) {
    let inner_width = area.width.saturating_sub(2);
    let inner_height = area.height.saturating_sub(2);
    let (square_width, square_height) = square_dimensions(inner_width, inner_height);
    let board_width = 2 + square_width * 8;
    let board_height = square_height * 8 + 1;
    let horizontal_padding = inner_width.saturating_sub(board_width) / 2;
    let vertical_padding = inner_height.saturating_sub(board_height) / 2;
    let mut lines = Vec::with_capacity(inner_height as usize);
    lines.extend((0..vertical_padding).map(|_| Line::default()));

    for display_rank in 0..8 {
        let rank = if app.flipped() {
            display_rank
        } else {
            7 - display_rank
        };
        for square_row in 0..square_height {
            let label = if square_row == square_height / 2 {
                format!("{} ", rank + 1)
            } else {
                "  ".to_owned()
            };
            let mut spans = vec![
                Span::raw(" ".repeat(horizontal_padding as usize)),
                Span::styled(label, Style::default().fg(Color::DarkGray)),
            ];
            for display_file in 0..8 {
                let file = if app.flipped() {
                    7 - display_file
                } else {
                    display_file
                };
                let square = Square::from_idx(rank * 8 + file);
                let cell = Cell::new(square, file, rank, square_width, square_height);
                spans.push(cell.render_row(app, piece_set, square_row));
            }
            lines.push(Line::from(spans));
        }
    }
    let files = (0..8)
        .map(|display_file| {
            let file = if app.flipped() {
                7 - display_file
            } else {
                display_file
            };
            Span::styled(
                centered((b'a' + file as u8) as char, square_width),
                Style::default().fg(Color::DarkGray),
            )
        })
        .collect::<Vec<_>>();
    let mut labels = vec![
        Span::raw(" ".repeat(horizontal_padding as usize)),
        Span::raw("  "),
    ];
    labels.extend(files);
    lines.push(Line::from(labels));

    let title = app.selected().map_or_else(
        || format!(" Board · cursor {} ", app.cursor()),
        |selected| format!(" Board · cursor {} · sel {selected} ", app.cursor()),
    );
    frame.render_widget(
        Paragraph::new(lines).block(Block::default().title(title).borders(Borders::ALL)),
        area,
    );
}

/// `square_dimensions` scales board cells while accounting for terminal character aspect ratio.
///
/// @param: width - available width inside the board border
/// @param: height - available height inside the board border
/// @return: square width and height in terminal cells
fn square_dimensions(width: u16, height: u16) -> (u16, u16) {
    let maximum_width = width.saturating_sub(2) / 8;
    let maximum_height = height.saturating_sub(1) / 8;
    let square_height = maximum_height.min(maximum_width / 2).max(1);
    let square_width = (square_height * 2).max(3).min(maximum_width.max(1));
    (square_width, square_height)
}

/// `centered` places one character in the middle of a fixed-width square.
///
/// @param: symbol - character to center
/// @param: width - square width in terminal cells
/// @return: padded square contents
fn centered(symbol: char, width: u16) -> String {
    let left = width.saturating_sub(1) / 2;
    let right = width.saturating_sub(left + 1);
    format!(
        "{}{}{}",
        " ".repeat(left as usize),
        symbol,
        " ".repeat(right as usize)
    )
}
