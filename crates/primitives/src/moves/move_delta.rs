use crate::{Pieces, Sides, Square};
use core::mem::size_of;

const PIECE_SHIFT: u16 = 6;
const SIDE_SHIFT: u16 = 9;
const PIECE_DELTA_KIND_SHIFT: u16 = 10;
const PRESENT_SHIFT: u16 = 11;

const SQUARE_MASK: u16 = 0x3f;
const PIECE_MASK: u16 = 0x7;
const SIDE_MASK: u16 = 0x1;
const PIECE_DELTA_KIND_MASK: u16 = 0x1;
const PRESENT_MASK: u16 = 0x1;

/// PieceDeltaKind is the kind of change represented by a [`PieceDelta`]
///
/// PieceDeltaKind distinguishes piece additions from removals when consumers apply a move
/// delta to their own incremental state
///
/// @type
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum PieceDeltaKind {
    Added,
    Removed = 1 << PIECE_DELTA_KIND_SHIFT,
}

/// PieceDelta is a compact, typed representation of a piece being added to
/// or removed from a position
///
/// PieceDelta data is stored in a `u16` with the following schema
///
/// |       | square | piece | side | delta kind | present | reserved |
/// | ----- | ------ | ----- | ---- | ---------- | ------- | -------- |
/// | bits  |    0-5 |   6-8 |    9 |         10 |      11 |    12-15 |
/// | mask  |   0x3f |   0x7 |  0x1 |        0x1 |     0x1 |      0xf |
/// | shift |      0 |     6 |    9 |         10 |      11 |       12 |
///
/// PieceDelta's `present` bit distinguishes an initialized delta from the zero-valued
/// empty representation used by [`MoveDelta`]
///
/// PieceDelta values are encoded from and decoded into the [`Square`], [`Pieces`], and
/// [`Sides`] primitive types
///
/// PieceDelta allows downstream systems to update derived data without comparing complete
/// board representations
///
/// @type
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[repr(transparent)]
pub struct PieceDelta {
    data: u16,
}

impl PieceDelta {
    /// new creates a piece delta
    ///
    /// @param: side - side whose piece changed
    /// @param: piece - piece type that changed
    /// @param: square - square where the change occurred
    /// @param: kind - kind of piece change
    /// @return: initialized piece delta
    #[inline]
    pub fn new(side: Sides, piece: Pieces, square: Square, kind: PieceDeltaKind) -> Self {
        assert_ne!(piece, Pieces::None, "a piece delta must contain a piece");

        Self {
            data: square.idx() as u16
                | ((piece.idx() as u16) << PIECE_SHIFT)
                | ((side.idx() as u16) << SIDE_SHIFT)
                | kind as u16
                | (PRESENT_MASK << PRESENT_SHIFT),
        }
    }

    /// added creates a delta for a piece added to a square
    ///
    /// @param: side - side whose piece was added
    /// @param: piece - piece type that was added
    /// @param: square - square where the piece was added
    /// @return: initialized added-piece delta
    #[inline]
    pub fn added(side: Sides, piece: Pieces, square: Square) -> Self {
        Self::new(side, piece, square, PieceDeltaKind::Added)
    }

    /// removed creates a delta for a piece removed from a square
    ///
    /// @param: side - side whose piece was removed
    /// @param: piece - piece type that was removed
    /// @param: square - square where the piece was removed
    /// @return: initialized removed-piece delta
    #[inline]
    pub fn removed(side: Sides, piece: Pieces, square: Square) -> Self {
        Self::new(side, piece, square, PieceDeltaKind::Removed)
    }

    /// side returns the side whose piece changed
    ///
    /// @return: side whose piece changed
    #[inline]
    pub fn side(self) -> Sides {
        Sides::from_idx(((self.data >> SIDE_SHIFT) & SIDE_MASK) as usize)
    }

    /// piece returns the piece type that changed
    ///
    /// @return: piece type that changed
    #[inline]
    pub fn piece(self) -> Pieces {
        Pieces::from_idx(((self.data >> PIECE_SHIFT) & PIECE_MASK) as usize)
    }

    /// square returns the square where the piece changed
    ///
    /// @return: square where the piece changed
    #[inline]
    pub fn square(self) -> Square {
        Square::from_idx((self.data & SQUARE_MASK) as usize)
    }

    /// kind returns the kind of piece change
    ///
    /// @return: kind of piece change
    #[inline]
    pub fn kind(self) -> PieceDeltaKind {
        if ((self.data >> PIECE_DELTA_KIND_SHIFT) & PIECE_DELTA_KIND_MASK) == 0 {
            PieceDeltaKind::Added
        } else {
            PieceDeltaKind::Removed
        }
    }

    /// is_present checks whether this delta is initialized
    ///
    /// @return: true if the delta is initialized, false otherwise
    #[inline]
    pub fn is_present(self) -> bool {
        ((self.data >> PRESENT_SHIFT) & PRESENT_MASK) != 0
    }
}

const _: () = assert!(size_of::<PieceDelta>() == 2);

/// MoveDelta contains the piece changes made by one chess move
///
/// MoveDelta changes have deterministic order: captured pieces are removed first,
/// followed by moving pieces being removed, then destination pieces being
/// added, with castling removing the king and rook before adding both pieces
///
/// MoveDelta is returned by move-making operations so evaluation and other incremental
/// systems can consume every board change in a stable order
///
/// @type
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[repr(C)]
#[must_use]
pub struct MoveDelta {
    changes: [PieceDelta; MoveDelta::MAX_CHANGES],
}

impl MoveDelta {
    /// MAX_CHANGES is the maximum number of piece changes produced by one chess move
    pub const MAX_CHANGES: usize = 4;

    /// push appends a piece change to the move delta
    ///
    /// @param: delta - initialized piece change to append
    /// @return: void
    /// @side-effects: modifies the move delta
    #[inline]
    pub fn push(&mut self, delta: PieceDelta) {
        assert!(delta.is_present(), "cannot push an empty piece delta");
        let slot = self
            .changes
            .iter_mut()
            .find(|change| !change.is_present())
            .expect("a chess move cannot contain more than four piece changes");
        *slot = delta;
    }

    /// iter returns the piece changes in deterministic insertion order
    ///
    /// @return: iterator over the initialized piece changes
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = PieceDelta> + '_ {
        self.changes
            .iter()
            .copied()
            .take_while(|change| change.is_present())
    }
}

const _: () = assert!(size_of::<MoveDelta>() == 8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_piece_delta_combination_round_trips() {
        for side in [Sides::White, Sides::Black] {
            for piece in Pieces::ALL {
                for square in Square::ALL {
                    for kind in [PieceDeltaKind::Added, PieceDeltaKind::Removed] {
                        let delta = PieceDelta::new(side, piece, square, kind);
                        assert!(delta.is_present());
                        assert_eq!(delta.side(), side);
                        assert_eq!(delta.piece(), piece);
                        assert_eq!(delta.square(), square);
                        assert_eq!(delta.kind(), kind);
                    }
                }
            }
        }
    }

    #[test]
    fn default_piece_delta_is_not_present() {
        assert!(!PieceDelta::default().is_present());
    }

    #[test]
    fn move_delta_iterates_in_insertion_order() {
        let changes = [
            PieceDelta::removed(Sides::Black, Pieces::Pawn, Square::D5),
            PieceDelta::removed(Sides::White, Pieces::Pawn, Square::E4),
            PieceDelta::added(Sides::White, Pieces::Pawn, Square::D5),
        ];
        let mut delta = MoveDelta::default();
        for change in changes {
            delta.push(change);
        }

        assert_eq!(delta.iter().collect::<Vec<_>>(), changes);
    }

    #[test]
    fn delta_sizes_are_stable() {
        assert_eq!(size_of::<PieceDelta>(), 2);
        assert_eq!(size_of::<MoveDelta>(), 8);
    }
}
