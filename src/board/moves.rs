use crate::board::{Board, SideCastlingSquares};
use crate::primitives::{Black, Move, Pieces, Side, Sides, Square, White};

impl Board {
    // make_move makes the given move on the board
    //
    // @param: mv - move to make
    // @return: void
    // @side-effects: modifies the `board`
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
    // @side-effects: modifies the `board`
    // @side-effects: reverts the `state` back
    pub fn unmake_move(&mut self) {
        // revert the last move on the board
        if let Some(state) = self.history.pop() {
            self.state = state;
        } else {
            return;
        }

        // unmake the move for the side that moved
        match self.turn() {
            Sides::White => self.unmake_move_for_side::<White>(self.state.next_move),
            Sides::Black => self.unmake_move_for_side::<Black>(self.state.next_move),
        }
    }

    // move_piece_no_incrementals moves the piece from the given square to the
    // given square without updating the zobrist key or any incremental game state
    //
    // @param: piece - piece to move
    // @param: from - square to move the piece from
    // @param: to - square to move the piece to
    // @return: void
    // @side-effects: modifies the `board`
    fn move_piece_no_incrementals<S: Side>(&mut self, piece: Pieces, from: Square, to: Square) {
        self.remove_piece_no_incrementals::<S>(piece, from);
        self.set_piece_no_incrementals::<S>(piece, to);
    }

    // move_piece moves the piece from the given square to the given square
    //
    // @param: piece - piece to move
    // @param: from - square to move the piece from
    // @param: to - square to move the piece to
    // @return: void
    // @side-effects: modifies the `board`
    // @side-effects: revokes castling permissions if needed
    fn move_piece<S: SideCastlingSquares>(&mut self, piece: Pieces, from: Square, to: Square) {
        self.remove_piece::<S>(piece, from);
        self.set_piece::<S>(piece, to);

        // revoke castling permissions if king/rook leaves from starting
        // square
        if (piece == Pieces::King || piece == Pieces::Rook) && self.state.castling.can_castle::<S>() {
            if from == S::KING {
                self.set_castling(self.state.castling.revoke::<S>());
            } else if from == S::KINGSIDE_ROOK {
                self.set_castling(self.state.castling.revoke_kingside::<S>());
            } else if from == S::QUEENSIDE_ROOK {
                self.set_castling(self.state.castling.revoke_queenside::<S>());
            }
        }
    }

    // capture_piece captures the side's piece at the given square
    //
    // @param: piece - piece to capture
    // @param: square - square that the captured piece is on
    // @return: void
    // @side-effects: modifies the `board`
    // @side-effects: resets the halfmove clock
    // @side-effects: updates castling permissions (if applicable)
    fn capture_piece<S: SideCastlingSquares>(&mut self, piece: Pieces, square: Square) {
        // remove the piece from the board
        self.remove_piece::<S>(piece, square);

        // reset the halfmove clock since a capture has occurred
        self.state.halfmoves = 0;

        // if the captured piece is a rook (king captures are invalid), and
        // the side has castling permissions, then revoke the appropriate
        // castling permissions
        if piece == Pieces::Rook && self.state.castling.can_castle::<S>() {
            if square == S::QUEENSIDE_ROOK {
                self.set_castling(self.state.castling.revoke_queenside::<S>());
            } else if square == S::KINGSIDE_ROOK {
                self.set_castling(self.state.castling.revoke_kingside::<S>());
            }
        }
    }

    // make_move_for_side makes the given move on the board for the given side
    //
    // @param: mv - move to make
    // @return: void
    // @side-effects: modifies the `board`
    // @side-effects: modifies incremental game state
    fn make_move_for_side<S>(&mut self, mv: Move)
    where
        S: SideCastlingSquares,
        S::Other: SideCastlingSquares,
    {
        // push the current state into the history
        let mut current_state = self.state;
        current_state.next_move = mv;
        self.history.push(current_state);

        // helper variables to avoid repeated calls
        let piece = mv.piece();
        let from = mv.from();
        let to = mv.to();

        // increment the move counters
        //
        // Note: if black is moving, increment the fullmove counter as well
        self.state.halfmoves += 1;
        if matches!(S::SIDE, Sides::Black) {
            self.state.fullmoves += 1;
        }

        // handle a piece capture
        let captured = mv.captured();
        if captured != Pieces::None {
            self.capture_piece::<S::Other>(captured, to);
        }

        // move the piece
        if piece != Pieces::Pawn {
            // if the moving piece is not a pawn, just perform a regular move
            self.move_piece::<S>(piece, from, to);

            // if the move is a castle, move the appropriate rook as well
            //
            // TODO: consider asserting that the destination square matches either
            //       possible destination square
            if mv.is_castle() {
                if to == S::KINGSIDE_DESTINATION {
                    self.move_piece::<S>(
                        Pieces::Rook,
                        S::KINGSIDE_ROOK,
                        S::KINGSIDE_ROOK_DESTINATION,
                    );
                } else if to == S::QUEENSIDE_DESTINATION {
                    self.move_piece::<S>(
                        Pieces::Rook,
                        S::QUEENSIDE_ROOK,
                        S::QUEENSIDE_ROOK_DESTINATION,
                    );
                }
            }
        } else {
            // if the move is a pawn move, check if the move is a promotion and
            // handle the piece move accordingly
            let promoted = mv.promoted();
            let is_promotion = promoted != Pieces::None;
            self.remove_piece::<S>(piece, from);
            self.set_piece::<S>(if !is_promotion { piece } else { promoted }, to);

            // if the move is an en passant capture, remove the opponent's pawn
            if mv.is_en_passant() {
                self.remove_piece::<S::Other>(Pieces::Pawn, to ^ 8);
            }

            // reset the halfmove clock since a pawn moved
            self.state.halfmoves = 0;
        }

        // if the move is a double step, set the en passant square, otherwise
        // clear it
        if mv.is_double_step() {
            self.set_en_passant(to ^ 8);
        } else {
            self.clear_en_passant();
        }

        // swap the side to move
        self.swap_sides::<S>();
    }

    // unmake_move_for_side unmakes the last move on the board for the given
    // side
    //
    // Note: since unmake pops from the history, we don't need to recompute
    //       any incremental game state since those are retrieved directly
    //
    // @param: self - mutable reference to the board
    // @return: void
    // @side-effects: modifies the `board`
    fn unmake_move_for_side<S>(&mut self, mv: Move)
    where
        S: SideCastlingSquares,
        S::Other: SideCastlingSquares,
    {
        // extract key move data
        let piece = mv.piece();
        let from = mv.from();
        let to = mv.to();

        // move the piece back to the original square, or restore the pawn if
        // it was promoted
        let promoted = mv.promoted();
        if matches!(promoted, Pieces::None) {
            self.move_piece_no_incrementals::<S>(piece, to, from);
        } else {
            self.remove_piece_no_incrementals::<S>(promoted, to);
            self.set_piece_no_incrementals::<S>(Pieces::Pawn, from);
        }

        // if the move was a castle, move the appropriate rook back as well
        if mv.is_castle() {
            if to == S::KINGSIDE_DESTINATION {
                self.move_piece_no_incrementals::<S>(
                    Pieces::Rook,
                    S::KINGSIDE_ROOK_DESTINATION,
                    S::KINGSIDE_ROOK,
                );
            } else if to == S::QUEENSIDE_DESTINATION {
                self.move_piece_no_incrementals::<S>(
                    Pieces::Rook,
                    S::QUEENSIDE_ROOK_DESTINATION,
                    S::QUEENSIDE_ROOK,
                );
            }
        }

        // if the move was a capture, restore the captured piece
        let captured = mv.captured();
        if !matches!(captured, Pieces::None) {
            self.set_piece_no_incrementals::<S::Other>(captured, to);
        }

        // if the move was an en passant capture, restore the opponent's pawn
        if mv.is_en_passant() {
            self.set_piece_no_incrementals::<S::Other>(Pieces::Pawn, to ^ 8);
        }
    }

}
