//! Configurable terminal piece assets.

mod ascii;
mod braille;
mod quadrant;
mod silhouette;
mod symbol;

use std::fmt::{self, Display};
use std::str::FromStr;

use chess_kit_primitives::{Pieces, Sides};
use ratatui::style::Modifier;

/// `PieceSet` selects the assets used to render chess pieces.
///
/// @type
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PieceSet {
    #[default]
    Ascii,
    Braille,
    Quadrant,
}

impl PieceSet {
    /// NAMES lists accepted command-line piece-set names.
    pub const NAMES: &str = "ascii, braille, quadrant";

    /// render_row draws one row of a piece within a board cell.
    ///
    /// @param: piece - piece type to draw
    /// @param: side - side that owns the piece
    /// @param: width - cell width in terminal columns
    /// @param: height - cell height in terminal rows
    /// @param: row - zero-based row within the cell
    /// @return: piece row padded to the cell width
    pub(super) fn render_row(
        self,
        piece: Pieces,
        side: Sides,
        width: u16,
        height: u16,
        row: u16,
    ) -> String {
        match self {
            Self::Ascii => ascii::render_row(piece, side, width, height, row),
            Self::Braille => braille::render_row(piece, side, width, height, row),
            Self::Quadrant => quadrant::render_row(piece, side, width, height, row),
        }
    }

    /// style_modifier returns the terminal emphasis for this piece set.
    ///
    /// @return: style modifier applied to occupied cells
    pub(super) const fn style_modifier(self) -> Modifier {
        match self {
            Self::Ascii => Modifier::empty(),
            Self::Braille => Modifier::BOLD,
            Self::Quadrant => Modifier::empty(),
        }
    }
}

impl Display for PieceSet {
    /// fmt returns the command-line name of this piece set.
    ///
    /// @impl: Display::fmt
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ascii => formatter.write_str("ascii"),
            Self::Braille => formatter.write_str("braille"),
            Self::Quadrant => formatter.write_str("quadrant"),
        }
    }
}

impl FromStr for PieceSet {
    type Err = String;

    /// from_str parses a command-line piece-set name.
    ///
    /// @impl: FromStr::from_str
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "ascii" => Ok(Self::Ascii),
            "braille" => Ok(Self::Braille),
            "quadrant" => Ok(Self::Quadrant),
            _ => Err(value.to_owned()),
        }
    }
}
