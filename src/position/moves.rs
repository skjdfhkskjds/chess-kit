use crate::attack_table::AttackTable;
use crate::position::{Position, SideCastlingSquares};
use crate::primitives::{
    Black, GameStateExt, Move, MoveType, Pieces, Side, Sides, Square, State, White,
};

impl<AT, StateT> Position<AT, StateT>
where
    AT: AttackTable,
    StateT: State + GameStateExt,
{
    // make_move makes the given move from the current position
    //
    // @param: mv - move to make
    // @return: void
    // @side-effects: modifies the `position`
    // @side-effects: modifies incremental game state
    pub fn make_move(&mut self, mv: Move) {
        match self.turn() {
            Sides::White => self.make_move_for_side::<White>(mv),
            Sides::Black => self.make_move_for_side::<Black>(mv),
        }
    }

    // unmake_move unmakes the last move on the board
    //
    // @return: void
    // @side-effects: modifies the `position`
    // @side-effects: reverts the `state` back
    pub fn unmake_move(&mut self, mv: Move) {
        // unmake the move for the side that moved
        match self.turn() {
            Sides::White => self.unmake_move_for_side::<Black>(mv),
            Sides::Black => self.unmake_move_for_side::<White>(mv),
        }
    }

    // move_piece_no_incrementals moves the piece from the given square to the
    // given square without updating the zobrist key or any incremental game state
    //
    // @param: piece - piece to move
    // @param: from - square to move the piece from
    // @param: to - square to move the piece to
    // @return: void
    // @side-effects: modifies the `position`
    #[inline(always)]
    fn move_piece_no_incrementals<SideT: Side>(&mut self, piece: Pieces, from: Square, to: Square) {
        self.remove_piece_no_incrementals::<SideT>(piece, from);
        self.set_piece_no_incrementals::<SideT>(piece, to);
    }

    // move_piece moves the piece from the given square to the given square
    //
    // @param: piece - piece to move
    // @param: from - square to move the piece from
    // @param: to - square to move the piece to
    // @return: void
    // @side-effects: modifies the `position`
    #[inline(always)]
    fn move_piece<SideT: SideCastlingSquares>(&mut self, piece: Pieces, from: Square, to: Square) {
        self.remove_piece::<SideT>(piece, from);
        self.set_piece::<SideT>(piece, to);
    }

    // capture_piece captures the side's piece at the given square
    //
    // @param: piece - piece to capture
    // @param: square - square that the captured piece is on
    // @return: void
    // @side-effects: modifies the `position`
    // @side-effects: resets the halfmove clock
    // @side-effects: updates castling permissions (if applicable)
    #[inline(always)]
    fn capture_piece<SideT: SideCastlingSquares>(&mut self, piece: Pieces, square: Square) {
        // remove the piece from the board
        self.remove_piece::<SideT>(piece, square);

        // reset the halfmove clock since a capture has occurred
        self.state_mut().set_halfmoves(0);

        // if the captured piece is a rook (king captures are invalid), and
        // the side has castling permissions, then revoke the appropriate
        // castling permissions
        if piece == Pieces::Rook && self.state().castling().can_castle::<SideT>() {
            if square == SideT::QUEENSIDE_ROOK {
                self.set_castling(self.state().castling().revoke_queenside::<SideT>());
            } else if square == SideT::KINGSIDE_ROOK {
                self.set_castling(self.state().castling().revoke_kingside::<SideT>());
            }
        }
    }

    #[allow(dead_code)]
    fn update_blockers<SideT: Side>(&mut self, _square: Square) {
    }

    // make_move_for_side makes the given move from the current position for
    // the given side
    //
    // @param: mv - move to make
    // @return: void
    // @side-effects: modifies the `position`
    // @side-effects: modifies incremental game state
    #[inline(always)]
    fn make_move_for_side<SideT>(&mut self, mv: Move)
    where
        SideT: SideCastlingSquares,
        SideT::Other: SideCastlingSquares,
    {
        // push the current state into the history
        self.history.push_clone();

        // helper variables to avoid repeated calls
        let from = mv.from();
        let to = mv.to();
        let piece = self.piece_at(from);
        let move_type = mv.type_of();

        // increment the move counters
        //
        // Note: if black is moving, increment the fullmove counter as well
        self.state_mut().inc_halfmoves();
        if matches!(SideT::SIDE, Sides::Black) {
            self.state_mut().inc_fullmoves();
        }

        // handle a piece capture
        let captured = self.piece_at(to);
        if !matches!(captured, Pieces::None) {
            // capture the piece from the board
            self.capture_piece::<SideT::Other>(captured, to);
        }
        // set the captured piece for the state
        self.state_mut().set_captured_piece(captured);

        // move the piece
        if !matches!(piece, Pieces::Pawn) {
            // if the moving piece is not a pawn, just perform a regular move
            self.move_piece::<SideT>(piece, from, to);

            // if the moving piece is a king or a rook, we need to do some extra
            // work to handle castling and revoke castling permissions if needed
            if matches!(piece, Pieces::King) {
                // if the move is a castle, move the appropriate rook as well
                if matches!(move_type, MoveType::Castle) {
                    if to == SideT::KINGSIDE_DESTINATION {
                        // kingside castle
                        self.move_piece::<SideT>(
                            Pieces::Rook,
                            SideT::KINGSIDE_ROOK,
                            SideT::KINGSIDE_ROOK_DESTINATION,
                        );
                    } else {
                        // queenside castle
                        self.move_piece::<SideT>(
                            Pieces::Rook,
                            SideT::QUEENSIDE_ROOK,
                            SideT::QUEENSIDE_ROOK_DESTINATION,
                        );
                    }

                    // always revoke castling permissions after castling
                    self.set_castling(self.state().castling().revoke::<SideT>());
                } else if self.state().castling().can_castle::<SideT>() {
                    // if the side can still castle, revoke it since the king
                    // left the starting square
                    self.set_castling(self.state().castling().revoke::<SideT>());
                }
            } else if matches!(piece, Pieces::Rook) && self.state().castling().can_castle::<SideT>() {
                // if the moving piece is a rook and that side can still castle,
                // revoke the appropriate castling permissions if the rook is
                // leaving the starting square
                if from == SideT::KINGSIDE_ROOK {
                    self.set_castling(self.state().castling().revoke_kingside::<SideT>());
                } else if from == SideT::QUEENSIDE_ROOK {
                    self.set_castling(self.state().castling().revoke_queenside::<SideT>());
                }
            }
        } else {
            // if the move is a pawn move, check if the move is a promotion and
            // handle the piece move accordingly
            if matches!(move_type, MoveType::Promotion) {
                self.remove_piece::<SideT>(piece, from);
                self.set_piece::<SideT>(mv.promoted_to(), to);
            } else {
                self.move_piece::<SideT>(piece, from, to);
            }

            // if the move is an en passant capture, remove the opponent's pawn
            if matches!(move_type, MoveType::EnPassant) {
                self.remove_piece::<SideT::Other>(Pieces::Pawn, to ^ 8);
            }

            // reset the halfmove clock since a pawn moved
            self.state_mut().set_halfmoves(0);
        }

        // if the moving piece is a pawn, and the move is a double step, then an
        // en passant capture is possible
        //
        // TODO: use a smarter approach to filter further whether or not an en
        //       passant capture is possible
        if matches!(piece, Pieces::Pawn) && to.distance(from) == 16 {
            self.set_en_passant(to ^ 8);
        } else {
            self.clear_en_passant();
        }

        // swap the side to move
        self.swap_sides::<SideT>();
    }

    // unmake_move_for_side unmakes the last move from the current position for
    // the given side
    //
    // Note: since unmake pops from the history, we don't need to recompute
    //       any incremental game state since those are retrieved directly
    //
    // @return: void
    // @side-effects: modifies the `position`
    #[inline(always)]
    fn unmake_move_for_side<SideT>(&mut self, mv: Move)
    where
        SideT: SideCastlingSquares,
        SideT::Other: SideCastlingSquares,
    {
        // extract key move data
        let from = mv.from();
        let to = mv.to();
        let piece = self.piece_at(to);
        let move_type = mv.type_of();

        // move the piece back to the original square, or restore the pawn if
        // it was promoted
        if matches!(move_type, MoveType::Promotion) {
            self.remove_piece_no_incrementals::<SideT>(mv.promoted_to(), to);
            self.set_piece_no_incrementals::<SideT>(Pieces::Pawn, from);
        } else {
            self.move_piece_no_incrementals::<SideT>(piece, to, from);
        }

        // if the move was a castle, move the appropriate rook back as well
        if matches!(move_type, MoveType::Castle) {
            if to == SideT::KINGSIDE_DESTINATION {
                self.move_piece_no_incrementals::<SideT>(
                    Pieces::Rook,
                    SideT::KINGSIDE_ROOK_DESTINATION,
                    SideT::KINGSIDE_ROOK,
                );
            } else if to == SideT::QUEENSIDE_DESTINATION {
                self.move_piece_no_incrementals::<SideT>(
                    Pieces::Rook,
                    SideT::QUEENSIDE_ROOK_DESTINATION,
                    SideT::QUEENSIDE_ROOK,
                );
            }
        }

        // if the move was a capture, restore the captured piece
        let captured = self.state().captured_piece();
        if !matches!(captured, Pieces::None) {
            self.set_piece_no_incrementals::<SideT::Other>(captured, to);
        }

        // if the move was an en passant capture, restore the opponent's pawn
        if matches!(move_type, MoveType::EnPassant) {
            self.set_piece_no_incrementals::<SideT::Other>(Pieces::Pawn, to ^ 8);
        }

        // revert the state
        debug_assert!(!self.history.is_empty(), "history is empty on unmake move");
        self.history.pop();
    }
}
