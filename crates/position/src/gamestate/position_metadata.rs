use super::DrawState;
use chess_kit_primitives::{Castling, Clock, Sides, Square, ZobristKey};
use core::mem::size_of;

/// PositionMetadata contains non-board information describing a position and its identity
///
/// PositionMetadata is the portion of a history entry copied forward before a move updates
/// the side to move, clocks, rights, incremental key, and draw state
///
/// @type
#[derive(Clone, Copy)]
pub(crate) struct PositionMetadata {
    pub(crate) turn: Sides,
    pub(crate) castling: Castling,
    pub(crate) en_passant: Option<Square>,
    pub(crate) halfmoves: Clock,
    pub(crate) fullmoves: Clock,
    pub(crate) key: ZobristKey,
    pub(crate) draw_state: DrawState,
}

impl Default for PositionMetadata {
    fn default() -> Self {
        Self {
            turn: Sides::White,
            castling: Castling::all(),
            en_passant: None,
            halfmoves: 0,
            fullmoves: 0,
            key: ZobristKey::default(),
            draw_state: DrawState::default(),
        }
    }
}

const _: () = assert!(size_of::<PositionMetadata>() == 24);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_size_is_stable() {
        assert_eq!(size_of::<PositionMetadata>(), 24);
    }
}
