use chess_kit_primitives::{Bitboard, Pieces, Sides};

/// CheckInfo contains cached tactical information for the current position
///
/// CheckInfo stores derived attack data that is recomputed after a move instead of copied
/// forward with the persistent position metadata
///
/// @type
#[derive(Clone, Copy, Default)]
pub(crate) struct CheckInfo {
    pub(crate) checkers: Bitboard,
    pub(crate) king_blockers: [Bitboard; Sides::TOTAL],
    pub(crate) pinners: [Bitboard; Sides::TOTAL],
    pub(crate) check_squares: [Bitboard; Pieces::TOTAL],
}
