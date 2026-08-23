use chess_kit_primitives::{Pieces, Sides};

use super::{silhouette, symbol};

const PAWN: &str = include_str!("../../../assets/pieces/quadrant/pawn.mask");
const KNIGHT: &str = include_str!("../../../assets/pieces/quadrant/knight.mask");
const BISHOP: &str = include_str!("../../../assets/pieces/quadrant/bishop.mask");
const ROOK: &str = include_str!("../../../assets/pieces/quadrant/rook.mask");
const QUEEN: &str = include_str!("../../../assets/pieces/quadrant/queen.mask");
const KING: &str = include_str!("../../../assets/pieces/quadrant/king.mask");

const GLYPHS: [char; 16] = [
    ' ', '▘', '▝', '▀', '▖', '▌', '▞', '▛', '▗', '▚', '▐', '▜', '▄', '▙', '▟', '█',
];

/// render_row draws a piece silhouette with opaque two-by-two quadrant cells.
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
    let piece_pixel_width = piece_columns * 2;
    let canvas_pixel_width = width * 2;
    let canvas_pixel_height = height * 2;
    let piece_pixel_height = (u32::from(canvas_pixel_height) * 4).div_ceil(5) as u16;
    let left = canvas_pixel_width.saturating_sub(piece_pixel_width) / 2;
    let top = canvas_pixel_height.saturating_sub(piece_pixel_height) / 2;
    let bounds = (left, top, piece_pixel_width, piece_pixel_height);
    let mask = piece_mask(piece);

    (0..width)
        .map(|column| quadrant_cell(mask, column, row, bounds))
        .collect()
}

/// Returns the standalone silhouette asset for a quadrant piece.
fn piece_mask(piece: Pieces) -> &'static str {
    match piece {
        Pieces::Pawn => PAWN,
        Pieces::Knight => KNIGHT,
        Pieces::Bishop => BISHOP,
        Pieces::Rook => ROOK,
        Pieces::Queen => QUEEN,
        Pieces::King => KING,
        Pieces::None => unreachable!("empty squares do not have piece silhouettes"),
    }
}

/// `quadrant_cell` samples the four pixels represented by one terminal character.
///
/// @param: mask - normalized piece silhouette
/// @param: column - terminal column within the board cell
/// @param: row - terminal row within the board cell
/// @param: bounds - silhouette origin and dimensions in quadrant pixels
/// @return: block-element glyph representing the sampled quadrants
fn quadrant_cell(mask: &str, column: u16, row: u16, bounds: (u16, u16, u16, u16)) -> char {
    let top_left = silhouette::sample(mask, column * 2, row * 2, bounds) as usize;
    let top_right = silhouette::sample(mask, column * 2 + 1, row * 2, bounds) as usize;
    let bottom_left = silhouette::sample(mask, column * 2, row * 2 + 1, bounds) as usize;
    let bottom_right = silhouette::sample(mask, column * 2 + 1, row * 2 + 1, bounds) as usize;
    let quadrants = top_left | (top_right << 1) | (bottom_left << 2) | (bottom_right << 3);
    GLYPHS[quadrants]
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn filled_runs(row: &str) -> usize {
        row.chars()
            .fold((0, false), |(runs, was_filled), pixel| {
                let is_filled = pixel == '#';
                (runs + usize::from(is_filled && !was_filled), is_filled)
            })
            .0
    }

    #[test]
    fn king_crown_keeps_its_defining_cross() {
        assert!(KING.lines().take(2).all(|row| !row.contains('#')));
        let king_stem = KING.lines().nth(2).unwrap();
        let king_crossbar = KING.lines().nth(11).unwrap();
        assert_eq!(filled_runs(king_stem), 1);
        assert_eq!(filled_runs(king_crossbar), 1);
        assert!(king_crossbar.matches('#').count() > king_stem.matches('#').count());
    }

    #[test]
    fn queen_preserves_half_of_the_original_detailed_silhouette() {
        let original = silhouette::piece(Pieces::Queen);
        assert!(
            QUEEN
                .lines()
                .zip(original.lines())
                .all(|(quadrant, original)| quadrant[20..] == original[20..])
        );
    }

    #[test]
    fn bishop_has_a_long_flat_base() {
        let body = BISHOP.lines().nth(30).unwrap();
        let base = BISHOP.lines().nth(35).unwrap();
        assert_eq!(filled_runs(base), 1);
        assert!(base.matches('#').count() > body.matches('#').count());
    }

    #[test]
    fn quadrant_assets_share_dimensions_and_vertical_bounds() {
        let occupied_rows = |mask: &str| {
            let first = mask.lines().position(|row| row.contains('#')).unwrap();
            let last = mask
                .lines()
                .enumerate()
                .filter(|(_, row)| row.contains('#'))
                .map(|(index, _)| index)
                .last()
                .unwrap();
            (first, last)
        };

        for piece in Pieces::ALL {
            let mask = piece_mask(piece);
            let rows = mask.lines().collect::<Vec<_>>();
            assert_eq!(rows.len(), 40);
            assert!(rows.iter().all(|row| row.len() == 40));
            assert_eq!(occupied_rows(mask), (2, 39));
        }
    }

    #[test]
    fn assets_that_should_be_symmetric_are_exactly_mirrored() {
        for mask in [PAWN, ROOK, QUEEN, KING] {
            assert!(mask.lines().all(|row| row.bytes().eq(row.bytes().rev())));
        }
    }

    #[test]
    fn each_piece_has_a_distinct_fixed_width_quadrant_rendering() {
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
                .any(|character| GLYPHS[1..].contains(&character))
        }));
    }
}
