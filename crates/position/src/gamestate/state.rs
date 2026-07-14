use super::{
    DrawState, capture_info::CaptureInfo, check_info::CheckInfo,
    position_metadata::PositionMetadata,
};
use chess_kit_collections::Copyable;
use chess_kit_primitives::{Bitboard, Castling, Clock, Pieces, Side, Sides, Square, ZobristKey};

/// PositionState is the complete private state for one history ply
///
/// PositionState composes persistent metadata with per-move capture information and derived
/// tactical caches so undo can restore the previous ply directly from history
///
/// @type
#[derive(Clone, Copy, Default)]
pub(crate) struct PositionState {
    pub(crate) metadata: PositionMetadata,
    pub(crate) capture_info: CaptureInfo,
    pub(crate) check_info: CheckInfo,
}

impl PositionState {
    /// initialize_from initializes a spare history entry without copying its tactical arrays
    ///
    /// @param: previous - state whose metadata should be copied
    /// @return: void
    /// @side-effects: replaces the current metadata
    #[inline(always)]
    fn initialize_from(&mut self, previous: &Self) {
        self.metadata = previous.metadata;
    }

    /// draw_state returns the incrementally maintained draw information
    ///
    /// @return: draw information for the current position
    #[inline]
    pub(crate) fn draw_state(&self) -> DrawState {
        self.metadata.draw_state
    }

    /// turn returns the side to move
    ///
    /// @return: side to move
    #[inline]
    pub(crate) fn turn(&self) -> Sides {
        self.metadata.turn
    }

    /// castling returns the current castling rights
    ///
    /// @return: current castling rights
    #[inline]
    pub(crate) fn castling(&self) -> Castling {
        self.metadata.castling
    }

    /// en_passant returns the current en passant square, if any
    ///
    /// @return: current en passant square, if any
    #[inline]
    pub(crate) fn en_passant(&self) -> Option<Square> {
        self.metadata.en_passant
    }

    /// captured_piece returns the piece captured to enter the current state
    ///
    /// @return: captured piece, or `Pieces::None` if no capture occurred
    #[inline]
    pub(crate) fn captured_piece(&self) -> Pieces {
        self.capture_info.captured_piece
    }

    /// halfmoves returns the current halfmove clock
    ///
    /// @return: current halfmove clock
    #[inline]
    pub(crate) fn halfmoves(&self) -> Clock {
        self.metadata.halfmoves
    }

    /// fullmoves returns the current fullmove clock
    ///
    /// @return: current fullmove clock
    #[inline]
    pub(crate) fn fullmoves(&self) -> Clock {
        self.metadata.fullmoves
    }

    /// key returns the incremental position key
    ///
    /// @return: unique key for the current position
    #[inline]
    pub(crate) fn key(&self) -> ZobristKey {
        self.metadata.key
    }

    /// checkers returns the pieces checking the side-to-move's king
    ///
    /// @return: bitboard of checking pieces
    #[inline]
    pub(crate) fn checkers(&self) -> Bitboard {
        self.check_info.checkers
    }

    /// king_blocker_pieces returns the pieces blocking SideT's king
    ///
    /// @marker: SideT - side whose king blockers should be returned
    /// @return: bitboard of SideT's king blockers
    #[inline]
    pub(crate) fn king_blocker_pieces<SideT: Side>(&self) -> Bitboard {
        self.check_info.king_blockers[SideT::SIDE]
    }

    /// pinning_pieces returns SideT's pieces pinning the opposing side
    ///
    /// @marker: SideT - side whose pinning pieces should be returned
    /// @return: bitboard of SideT's pinning pieces
    #[inline]
    pub(crate) fn pinning_pieces<SideT: Side>(&self) -> Bitboard {
        self.check_info.pinners[SideT::SIDE]
    }

    /// check_squares returns the squares where a piece would deliver check
    ///
    /// @param: piece - piece type whose checking squares should be returned
    /// @return: bitboard of squares that would deliver check
    #[inline]
    pub(crate) fn check_squares(&self, piece: Pieces) -> Bitboard {
        self.check_info.check_squares[piece]
    }

    /// set_draw_state replaces the incrementally maintained draw information
    ///
    /// @param: draw_state - draw information to set
    /// @return: void
    /// @side-effects: modifies the current state
    #[inline]
    pub(crate) fn set_draw_state(&mut self, draw_state: DrawState) {
        self.metadata.draw_state = draw_state;
    }

    /// set_turn replaces the side to move
    ///
    /// @param: turn - side to move
    /// @return: void
    /// @side-effects: modifies the current state
    #[inline]
    pub(crate) fn set_turn(&mut self, turn: Sides) {
        self.metadata.turn = turn;
    }

    /// set_castling replaces the current castling rights
    ///
    /// @param: castling - castling rights to set
    /// @return: void
    /// @side-effects: modifies the current state
    #[inline]
    pub(crate) fn set_castling(&mut self, castling: Castling) {
        self.metadata.castling = castling;
    }

    /// set_en_passant replaces the current en passant square
    ///
    /// @param: en_passant - en passant square to set, if any
    /// @return: void
    /// @side-effects: modifies the current state
    #[inline]
    pub(crate) fn set_en_passant(&mut self, en_passant: Option<Square>) {
        self.metadata.en_passant = en_passant;
    }

    /// set_captured_piece records the piece captured to enter this state
    ///
    /// @param: piece - captured piece to record
    /// @return: void
    /// @side-effects: modifies the current state
    #[inline]
    pub(crate) fn set_captured_piece(&mut self, piece: Pieces) {
        self.capture_info = CaptureInfo {
            captured_piece: piece,
        };
    }

    /// set_halfmoves replaces the current halfmove clock
    ///
    /// @param: halfmoves - halfmove clock to set
    /// @return: void
    /// @side-effects: modifies the current state
    #[inline]
    pub(crate) fn set_halfmoves(&mut self, halfmoves: Clock) {
        self.metadata.halfmoves = halfmoves;
    }

    /// inc_halfmoves increments the halfmove clock
    ///
    /// @return: void
    /// @side-effects: modifies the current state
    #[inline]
    pub(crate) fn inc_halfmoves(&mut self) {
        self.metadata.halfmoves += 1;
    }

    /// set_fullmoves replaces the current fullmove clock
    ///
    /// @param: fullmoves - fullmove clock to set
    /// @return: void
    /// @side-effects: modifies the current state
    #[inline]
    pub(crate) fn set_fullmoves(&mut self, fullmoves: Clock) {
        self.metadata.fullmoves = fullmoves;
    }

    /// inc_fullmoves increments the fullmove clock
    ///
    /// @return: void
    /// @side-effects: modifies the current state
    #[inline]
    pub(crate) fn inc_fullmoves(&mut self) {
        self.metadata.fullmoves += 1;
    }

    /// set_key replaces the incremental position key
    ///
    /// @param: key - position key to set
    /// @return: void
    /// @side-effects: modifies the current state
    #[inline]
    pub(crate) fn set_key(&mut self, key: ZobristKey) {
        self.metadata.key = key;
    }

    /// update_key applies a Zobrist delta to the current position key
    ///
    /// @param: key - Zobrist delta to apply
    /// @return: void
    /// @side-effects: modifies the current state
    #[inline]
    pub(crate) fn update_key(&mut self, key: ZobristKey) {
        self.metadata.key ^= key;
    }

    /// set_checkers replaces the pieces checking the side-to-move's king
    ///
    /// @param: checkers - bitboard of checking pieces
    /// @return: void
    /// @side-effects: modifies the current state
    #[inline]
    pub(crate) fn set_checkers(&mut self, checkers: Bitboard) {
        self.check_info.checkers = checkers;
    }

    /// set_king_blocker_pieces replaces the pieces blocking SideT's king
    ///
    /// @marker: SideT - side whose king blockers should be set
    /// @param: pieces - bitboard of king blockers
    /// @return: void
    /// @side-effects: modifies the current state
    #[inline]
    pub(crate) fn set_king_blocker_pieces<SideT: Side>(&mut self, pieces: Bitboard) {
        self.check_info.king_blockers[SideT::SIDE] = pieces;
    }

    /// set_pinning_pieces replaces SideT's pinning pieces
    ///
    /// @marker: SideT - side whose pinning pieces should be set
    /// @param: pieces - bitboard of pinning pieces
    /// @return: void
    /// @side-effects: modifies the current state
    #[inline]
    pub(crate) fn set_pinning_pieces<SideT: Side>(&mut self, pieces: Bitboard) {
        self.check_info.pinners[SideT::SIDE] = pieces;
    }

    /// set_check_squares replaces the checking squares for a piece type
    ///
    /// @param: piece - piece type whose checking squares should be set
    /// @param: squares - bitboard of checking squares
    /// @return: void
    /// @side-effects: modifies the current state
    #[inline]
    pub(crate) fn set_check_squares(&mut self, piece: Pieces, squares: Bitboard) {
        self.check_info.check_squares[piece] = squares;
    }
}

impl Copyable for PositionState {
    /// copy_from initializes this history entry from another state
    ///
    /// @impl: Copyable::copy_from
    #[inline(always)]
    fn copy_from(&mut self, other: &Self) {
        self.initialize_from(other);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn history_initialization_only_copies_metadata() {
        let mut previous = PositionState::default();
        previous.metadata.halfmoves = 12;
        previous.capture_info.captured_piece = Pieces::Queen;
        previous.check_info.checkers = Bitboard::square(Square::E4);

        let mut next = PositionState::default();
        next.initialize_from(&previous);

        assert_eq!(next.metadata.halfmoves, 12);
        assert_eq!(next.capture_info.captured_piece, Pieces::None);
        assert!(next.check_info.checkers.is_empty());
    }
}
