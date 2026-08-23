use chess_kit_primitives::{Pieces, Sides};

use super::{silhouette, symbol};

const BRAILLE_BASE: u32 = 0x2800;

const DOTS: [(u16, u16, u32); 8] = [
    (0, 0, 1),
    (0, 1, 2),
    (0, 2, 4),
    (1, 0, 8),
    (1, 1, 16),
    (1, 2, 32),
    (0, 3, 64),
    (1, 3, 128),
];

/// render_row draws a detailed piece silhouette with Unicode Braille cells.
///
/// @param: piece - piece type to draw
/// @param: side - side that owns the piece
/// @param: width - cell width in terminal columns
/// @param: height - cell height in terminal rows
/// @param: row - zero-based row within the cell
/// @return: piece row padded to the cell width
pub(super) fn render_row(piece: Pieces, side: Sides, width: u16, height: u16, row: u16) -> String {
    if height < 3 {
        return symbol::render_row(piece, side, width, height, row);
    }

    let piece_columns = (u32::from(width) * 3).div_ceil(4) as u16;
    let piece_dot_width = piece_columns * 2;
    let canvas_dot_width = width * 2;
    let canvas_dot_height = height * 4;
    let piece_dot_height = (u32::from(canvas_dot_height) * 4).div_ceil(5) as u16;
    let left = canvas_dot_width.saturating_sub(piece_dot_width) / 2;
    let top = canvas_dot_height.saturating_sub(piece_dot_height) / 2;
    let bounds = (left, top, piece_dot_width, piece_dot_height);
    let mask = silhouette::piece(piece);

    (0..width)
        .map(|column| braille_cell(mask, column, row, bounds))
        .collect()
}

/// `braille_cell` samples the eight dots represented by one terminal character.
///
/// @param: mask - normalized piece silhouette
/// @param: column - terminal column within the board cell
/// @param: row - terminal row within the board cell
/// @param: bounds - silhouette origin and dimensions in Braille dots
/// @return: Braille glyph or a regular background space
fn braille_cell(mask: &str, column: u16, row: u16, bounds: (u16, u16, u16, u16)) -> char {
    let dots = DOTS.iter().fold(0, |dots, &(offset_x, offset_y, bit)| {
        if silhouette::sample(mask, column * 2 + offset_x, row * 4 + offset_y, bounds) {
            dots | bit
        } else {
            dots
        }
    });
    if dots == 0 {
        ' '
    } else {
        char::from_u32(BRAILLE_BASE + dots).expect("Braille dot masks are valid Unicode")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn each_piece_has_a_distinct_fixed_width_braille_rendering() {
        let renderings = Pieces::ALL
            .map(|piece| {
                (0..5)
                    .map(|row| render_row(piece, Sides::White, 10, 5, row))
                    .inspect(|row| assert_eq!(row.chars().count(), 10))
                    .collect::<String>()
            })
            .into_iter()
            .collect::<HashSet<_>>();

        assert_eq!(renderings.len(), Pieces::ALL.len());
        assert!(renderings.iter().all(|rendering| {
            rendering
                .chars()
                .any(|character| ('\u{2801}'..='\u{28ff}').contains(&character))
        }));
    }
}
