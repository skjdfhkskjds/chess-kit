use chess_kit_primitives::{Pieces, Sides, call_as};

/// render_row draws a centered Unicode chess symbol for compact cells.
///
/// @param: piece - piece type to draw
/// @param: side - side that owns the piece
/// @param: width - cell width in terminal columns
/// @param: height - cell height in terminal rows
/// @param: row - zero-based row within the cell
/// @return: symbol row padded to the cell width
pub(super) fn render_row(piece: Pieces, side: Sides, width: u16, height: u16, row: u16) -> String {
    if row != height / 2 {
        return " ".repeat(width as usize);
    }

    let symbol = piece_symbol(side, piece);
    let left = width.saturating_sub(1) / 2;
    let right = width.saturating_sub(left + 1);
    format!(
        "{}{}{}",
        " ".repeat(left as usize),
        symbol,
        " ".repeat(right as usize)
    )
}

/// `piece_symbol` returns the side-aware Unicode chess symbol.
///
/// @param: side - side that owns the piece
/// @param: piece - piece type to draw
/// @return: Unicode chess symbol
fn piece_symbol(side: Sides, piece: Pieces) -> char {
    let display = call_as!(side, |SideT| piece.display::<SideT>().to_string());
    display
        .chars()
        .next()
        .expect("primitive piece displays are never empty")
}
