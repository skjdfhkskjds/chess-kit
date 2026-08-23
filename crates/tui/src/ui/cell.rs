use chess_kit_primitives::{Sides, Square};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;

use super::pieces::PieceSet;
use crate::App;

/// `Cell` describes one logical board square and its terminal dimensions.
///
/// @type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Cell {
    square: Square,
    file: usize,
    rank: usize,
    width: u16,
    height: u16,
}

impl Cell {
    /// new creates a renderable board cell.
    ///
    /// @param: square - logical board square
    /// @param: file - zero-based logical file
    /// @param: rank - zero-based logical rank
    /// @param: width - cell width in terminal columns
    /// @param: height - cell height in terminal rows
    /// @return: cell description
    pub(super) const fn new(
        square: Square,
        file: usize,
        rank: usize,
        width: u16,
        height: u16,
    ) -> Self {
        Self {
            square,
            file,
            rank,
            width,
            height,
        }
    }

    /// render_row draws one terminal row of this cell using the selected piece assets.
    ///
    /// @param: app - application state containing position and highlights
    /// @param: piece_set - assets used for an occupying piece
    /// @param: row - zero-based row within the cell
    /// @return: styled cell row
    pub(super) fn render_row(self, app: &App, piece_set: PieceSet, row: u16) -> Span<'static> {
        let occupant = app.position().piece_at(self.square);
        let contents = occupant.map_or_else(
            || " ".repeat(self.width as usize),
            |(side, piece)| piece_set.render_row(piece, side, self.width, self.height, row),
        );
        let style = occupant.map_or_else(
            || self.square_style(app),
            |(side, _)| self.piece_style(app, side, piece_set),
        );
        Span::styled(contents, style)
    }

    /// square_style returns the board and interaction colors for this cell.
    ///
    /// @param: app - application state containing cursor and selection
    /// @return: cell background and interaction style
    fn square_style(self, app: &App) -> Style {
        let background = if (self.file + self.rank).is_multiple_of(2) {
            Color::Rgb(92, 64, 51)
        } else {
            Color::Rgb(181, 136, 99)
        };
        let mut style = Style::default().fg(Color::White).bg(background);
        if app.selected() == Some(self.square) {
            style = style.bg(Color::Blue).add_modifier(Modifier::BOLD);
        }
        if app.cursor() == self.square {
            style = style
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED);
        }
        style
    }

    /// piece_style adds side contrast without changing cell highlights.
    ///
    /// @param: app - application state containing cursor and selection
    /// @param: side - side that owns the occupying piece
    /// @param: piece_set - assets controlling piece-specific emphasis
    /// @return: cell style with a side-specific foreground
    fn piece_style(self, app: &App, side: Sides, piece_set: PieceSet) -> Style {
        let foreground = match side {
            Sides::White => Color::Rgb(238, 238, 210),
            Sides::Black => Color::Rgb(30, 30, 30),
        };
        self.square_style(app)
            .fg(foreground)
            .add_modifier(piece_set.style_modifier())
    }
}
