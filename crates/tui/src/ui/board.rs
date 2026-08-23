use chess_kit_primitives::{Pieces, Sides, Square, call_as};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::App;

/// render draws the chess board with cursor and selection highlighting.
///
/// @param: frame - destination terminal frame
/// @param: area - board widget area
/// @param: app - application state to render
/// @return: void
/// @side-effects: writes the board widget into the frame buffer
pub(super) fn render(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let mut lines = Vec::with_capacity(9);
    for display_rank in 0..8 {
        let rank = if app.flipped() {
            display_rank
        } else {
            7 - display_rank
        };
        let mut spans = vec![Span::styled(
            format!("{} ", rank + 1),
            Style::default().fg(Color::DarkGray),
        )];
        for display_file in 0..8 {
            let file = if app.flipped() {
                7 - display_file
            } else {
                display_file
            };
            let square = Square::from_idx(rank * 8 + file);
            let symbol = app
                .position()
                .piece_at(square)
                .map_or(' ', |(side, piece)| piece_symbol(side, piece));
            spans.push(Span::styled(
                format!(" {symbol} "),
                square_style(square, app, file, rank),
            ));
        }
        lines.push(Line::from(spans));
    }
    let files = (0..8)
        .map(|display_file| {
            let file = if app.flipped() {
                7 - display_file
            } else {
                display_file
            };
            Span::styled(
                format!(" {} ", (b'a' + file as u8) as char),
                Style::default().fg(Color::DarkGray),
            )
        })
        .collect::<Vec<_>>();
    let mut labels = vec![Span::raw("  ")];
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

/// square_style returns the board and interaction colors for a square.
///
/// @param: square - logical board square
/// @param: app - application state
/// @param: file - zero-based logical file
/// @param: rank - zero-based logical rank
/// @return: square style
fn square_style(square: Square, app: &App, file: usize, rank: usize) -> Style {
    let background = if (file + rank).is_multiple_of(2) {
        Color::Rgb(92, 64, 51)
    } else {
        Color::Rgb(181, 136, 99)
    };
    let mut style = Style::default().fg(Color::White).bg(background);
    if app.selected() == Some(square) {
        style = style.bg(Color::Blue).add_modifier(Modifier::BOLD);
    }
    if app.cursor() == square {
        style = style
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED);
    }
    style
}

/// piece_symbol returns the side-aware Unicode chess symbol.
///
/// @param: side - side that owns the piece
/// @param: piece - piece to render
/// @return: Unicode chess symbol
fn piece_symbol(side: Sides, piece: Pieces) -> char {
    let display = call_as!(side, |SideT| piece.display::<SideT>().to_string());
    display
        .chars()
        .next()
        .expect("primitive piece displays are never empty")
}
