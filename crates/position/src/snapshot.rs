use chess_kit_primitives::{Black, Pieces, Side, Sides, Square, White, call_as};

use crate::PositionView;

/// `PositionSnapshot` is an owned, read-only projection of a chess position
///
/// It contains the presentation-neutral state needed to inspect a board
/// without exposing a position's search and move-generation representation.
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PositionSnapshot {
    squares: [Option<(Sides, Pieces)>; Square::TOTAL],
    side_to_move: Sides,
}

impl PositionSnapshot {
    /// empty creates an empty position snapshot for the given side to move
    ///
    /// This constructor is useful for custom position providers and test
    /// doubles that do not own a complete [`PositionView`].
    ///
    /// @marker: SideT - side whose turn it is
    /// @return: empty position snapshot
    pub const fn empty<SideT: Side>() -> Self {
        Self {
            squares: [None; Square::TOTAL],
            side_to_move: SideT::SIDE,
        }
    }

    /// piece_at returns the side and piece occupying a square
    ///
    /// @param: square - square to inspect
    /// @return: occupying side and piece, or None when the square is empty
    pub const fn piece_at(&self, square: Square) -> Option<(Sides, Pieces)> {
        self.squares[square.idx()]
    }

    /// side_to_move returns the side whose turn it is
    ///
    /// @return: side to move
    pub const fn side_to_move(&self) -> Sides {
        self.side_to_move
    }

    /// with_piece adds a piece and returns the updated snapshot
    ///
    /// This builder supports custom position providers while keeping the
    /// snapshot's storage representation private.
    ///
    /// @marker: SideT - side that owns the piece
    /// @param: square - square occupied by the piece
    /// @param: piece - piece occupying the square
    /// @return: updated position snapshot
    pub fn with_piece<SideT: Side>(mut self, square: Square, piece: Pieces) -> Self {
        self.squares[square.idx()] = Some((SideT::SIDE, piece));
        self
    }

    /// copy_side projects one side of a position view into this snapshot
    ///
    /// @marker: SideT - side whose pieces are projected
    /// @marker: PositionT - position view being projected
    /// @param: position - position view to project
    /// @return: void
    /// @side-effects: adds SideT's pieces to this snapshot
    fn copy_side<SideT, PositionT>(&mut self, position: &PositionT)
    where
        SideT: Side,
        PositionT: PositionView,
    {
        for square in position.occupancy::<SideT>() {
            let piece = position.piece_at(square);
            debug_assert_ne!(piece, Pieces::None);
            self.squares[square.idx()] = Some((SideT::SIDE, piece));
        }
    }
}

impl<PositionT> From<&PositionT> for PositionSnapshot
where
    PositionT: PositionView,
{
    /// from projects a position view into an owned position snapshot
    ///
    /// @param: position - position view to snapshot
    /// @return: owned position snapshot
    fn from(position: &PositionT) -> Self {
        let mut snapshot = call_as!(position.turn(), |SideT| Self::empty::<SideT>());
        snapshot.copy_side::<White, _>(position);
        snapshot.copy_side::<Black, _>(position);
        snapshot
    }
}

#[cfg(test)]
mod tests {
    use chess_kit_attack_table::DefaultAttackTable;

    use crate::{DefaultPosition, Fen, Setup};

    use super::*;

    #[test]
    fn converts_a_position_view_into_an_owned_snapshot() {
        let position = DefaultPosition::<DefaultAttackTable>::default();

        let snapshot = PositionSnapshot::from(&position);

        assert_eq!(snapshot.side_to_move(), Sides::White);
        assert_eq!(
            snapshot.piece_at(Square::E1),
            Some((Sides::White, Pieces::King))
        );
        assert_eq!(
            snapshot.piece_at(Square::E8),
            Some((Sides::Black, Pieces::King))
        );
        assert_eq!(snapshot.piece_at(Square::E4), None);
    }

    #[test]
    fn builds_side_aware_pieces_from_marker_types() {
        let snapshot = PositionSnapshot::empty::<White>()
            .with_piece::<White>(Square::E1, Pieces::King)
            .with_piece::<Black>(Square::E8, Pieces::King);

        assert_eq!(
            snapshot.piece_at(Square::E1),
            Some((Sides::White, Pieces::King))
        );
        assert_eq!(
            snapshot.piece_at(Square::E8),
            Some((Sides::Black, Pieces::King))
        );
    }

    #[test]
    fn derives_the_side_to_move_from_a_marker_type() {
        let snapshot = PositionSnapshot::empty::<Black>();

        assert_eq!(snapshot.side_to_move(), Sides::Black);
    }

    #[test]
    fn dispatches_a_runtime_turn_to_its_marker_type() {
        let fen = Fen::try_from("4k3/8/8/8/8/8/8/4K3 b - - 0 1").unwrap();
        let position = DefaultPosition::<DefaultAttackTable>::from(Setup::from(fen));

        let snapshot = PositionSnapshot::from(&position);

        assert_eq!(snapshot.side_to_move(), Sides::Black);
    }
}
