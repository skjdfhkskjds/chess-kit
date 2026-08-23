use chess_kit_primitives::{Pieces, Sides, call_as};

const PATTERN_WIDTH: usize = 8;
const PATTERN_HEIGHT: usize = 10;

#[rustfmt::skip]
const PAWN: [&str; PATTERN_HEIGHT] = [
    "........",
    "...##...",
    "..####..",
    "..####..",
    "...##...",
    "...##...",
    "..####..",
    ".######.",
    "########",
    "........",
];

#[rustfmt::skip]
const KNIGHT: [&str; PATTERN_HEIGHT] = [
    "..###...",
    ".#####..",
    "######..",
    "###.###.",
    "######..",
    ".#####..",
    "..####..",
    "..####..",
    ".######.",
    "########",
];

#[rustfmt::skip]
const BISHOP: [&str; PATTERN_HEIGHT] = [
    "...##...",
    "..####..",
    ".##.###.",
    "..####..",
    "...##...",
    "..####..",
    "..####..",
    ".######.",
    "########",
    "........",
];

#[rustfmt::skip]
const ROOK: [&str; PATTERN_HEIGHT] = [
    "##.##.##",
    "########",
    ".######.",
    "..####..",
    "..####..",
    "..####..",
    "..####..",
    ".######.",
    "########",
    "........",
];

#[rustfmt::skip]
const QUEEN: [&str; PATTERN_HEIGHT] = [
    "#..##..#",
    "##.##.##",
    ".######.",
    "..####..",
    "...##...",
    "..####..",
    "..####..",
    ".######.",
    "########",
    "........",
];

#[rustfmt::skip]
const KING: [&str; PATTERN_HEIGHT] = [
    "...##...",
    "..####..",
    "...##...",
    ".######.",
    "..####..",
    "...##...",
    "..####..",
    ".######.",
    "########",
    "........",
];

/// render_row draws the existing block-and-glyph piece style.
///
/// @param: piece - piece type to draw
/// @param: side - side that owns the piece
/// @param: width - cell width in terminal columns
/// @param: height - cell height in terminal rows
/// @param: row - zero-based row within the cell
/// @return: piece row padded to the cell width
pub(super) fn render_row(piece: Pieces, side: Sides, width: u16, height: u16, row: u16) -> String {
    if height >= 3 {
        return large_piece_row(piece, width, height, row);
    }
    if row == height / 2 {
        return centered(piece_symbol(side, piece), width);
    }
    " ".repeat(width as usize)
}

/// `large_piece_row` rasterizes one row of a piece at 75% of its cell width.
///
/// @param: piece - piece shape to rasterize
/// @param: width - cell width in terminal columns
/// @param: height - cell height in terminal rows
/// @param: row - row within the cell to render
/// @return: one terminal row of half-block graphics
fn large_piece_row(piece: Pieces, width: u16, height: u16, row: u16) -> String {
    let piece_width = (u32::from(width) * 3).div_ceil(4) as u16;
    let canvas_height = height * 2;
    let piece_height = (u32::from(canvas_height) * 4).div_ceil(5) as u16;
    let left = width.saturating_sub(piece_width) / 2;
    let top = canvas_height.saturating_sub(piece_height) / 2;
    let top_pixel = row * 2;
    let bottom_pixel = top_pixel + 1;
    let pattern = pattern(piece);

    (0..width)
        .map(|column| {
            let bounds = (left, top, piece_width, piece_height);
            let upper = piece_pixel(pattern, column, top_pixel, bounds);
            let lower = piece_pixel(pattern, column, bottom_pixel, bounds);
            match (upper, lower) {
                (false, false) => ' ',
                (true, false) => '▀',
                (false, true) => '▄',
                (true, true) => '█',
            }
        })
        .collect()
}

/// `piece_pixel` samples a canonical silhouette into a scaled pixel canvas.
///
/// @param: pattern - canonical piece silhouette
/// @param: x - horizontal canvas coordinate
/// @param: y - vertical canvas coordinate
/// @param: bounds - silhouette origin and scaled dimensions
/// @return: whether the sampled pixel belongs to the piece
fn piece_pixel(
    pattern: &[&str; PATTERN_HEIGHT],
    x: u16,
    y: u16,
    bounds: (u16, u16, u16, u16),
) -> bool {
    let (left, top, width, height) = bounds;
    if x < left || x >= left + width || y < top || y >= top + height {
        return false;
    }

    let pattern_x = usize::from(x - left) * PATTERN_WIDTH / usize::from(width);
    let pattern_y = usize::from(y - top) * PATTERN_HEIGHT / usize::from(height);
    pattern[pattern_y].as_bytes()[pattern_x] == b'#'
}

/// pattern returns the canonical silhouette for a piece type.
///
/// @param: piece - piece type to look up
/// @return: fixed-size bitmap silhouette
fn pattern(piece: Pieces) -> &'static [&'static str; PATTERN_HEIGHT] {
    match piece {
        Pieces::Pawn => &PAWN,
        Pieces::Knight => &KNIGHT,
        Pieces::Bishop => &BISHOP,
        Pieces::Rook => &ROOK,
        Pieces::Queen => &QUEEN,
        Pieces::King => &KING,
        Pieces::None => unreachable!("empty squares do not have piece silhouettes"),
    }
}

/// `piece_symbol` returns the side-aware Unicode fallback symbol.
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

/// `centered` places one character in the middle of a fixed-width cell.
///
/// @param: symbol - character to center
/// @param: width - cell width in terminal columns
/// @return: padded cell contents
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
