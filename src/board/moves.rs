use crate::board::board::Board;
use crate::primitives::moves::Move;
use crate::primitives::{Piece, Side, Square};

// TODO: refactor all of this
impl Board {
    // capture_piece captures the side's piece at the given square
    //
    // @param: self - mutable reference to the board
    // @param: side - side to whose piece is being captured
    // @param: piece - piece to capture
    // @param: square - square that the captured piece is on
    // @return: void
    // @side-effects: modifies the `board`
    // @side-effects: resets the halfmove clock
    // @side-effects: updates castling permissions (if applicable)
    fn capture_piece(&mut self, side: Side, piece: Piece, square: Square) {
        // remove the piece from the board
        self.remove_piece(side, piece, square);

        // reset the halfmove clock since a capture has occurred
        self.state.halfmoves = 0;

        // if the captured piece is a rook (king captures are invalid), and
        // the side has castling permissions, then revoke the appropriate
        // castling permissions
        if piece == Piece::Rook && self.state.castling.can_castle(side) {
            self.set_castling(match square {
                Square::A1 => self.state.castling.revoke_queenside(Side::White),
                Square::H1 => self.state.castling.revoke_kingside(Side::White),
                Square::A8 => self.state.castling.revoke_queenside(Side::Black),
                Square::H8 => self.state.castling.revoke_kingside(Side::Black),
                _ => self.state.castling,
            });
        }
    }

    // make_move makes the given move on the board
    //
    // @param: self - mutable reference to the board
    // @param: m - move to make
    // @return: void
    // @side-effects: modifies the `board`
    pub fn make_move(&mut self, mv: Move) {
        // push the current state into the history
        let mut current_state = self.state;
        current_state.next_move = mv;
        self.history.push(current_state);

        // helper variables to avoid repeated calls
        let us = self.turn();
        let opponent = self.opponent();
        let piece = mv.piece();
        let from = mv.from();
        let to = mv.to();

        // increment the move counters
        //
        // Note: if black is moving, increment the fullmove counter as well
        self.state.halfmoves += 1;
        if us == Side::Black {
            self.state.fullmoves += 1;
        }

        // handle a piece capture
        let captured = mv.captured();
        if captured != Piece::None {
            self.capture_piece(opponent, captured, to);
        }

        // move the piece
        if piece != Piece::Pawn {
            // if the moving piece is not a pawn, just perform a regular move
            self.move_piece(us, piece, from, to);

            // revoke castling permissions if king/rook leaves from starting
            // square
            if (piece == Piece::King || piece == Piece::Rook) && self.state.castling.can_castle(us)
            {
                let new_castling = match from {
                    Square::E1 => self.state.castling.revoke(Side::White),
                    Square::E8 => self.state.castling.revoke(Side::Black),
                    Square::A1 => self.state.castling.revoke_queenside(Side::White),
                    Square::H1 => self.state.castling.revoke_kingside(Side::White),
                    Square::A8 => self.state.castling.revoke_queenside(Side::Black),
                    Square::H8 => self.state.castling.revoke_kingside(Side::Black),
                    _ => self.state.castling,
                };

                if new_castling != self.state.castling {
                    self.set_castling(new_castling);
                }
            }

            // if the move is a castle, move the appropriate rook as well
            if mv.is_castle() {
                match to {
                    Square::G1 => self.move_piece(us, Piece::Rook, Square::H1, Square::F1),
                    Square::C1 => self.move_piece(us, Piece::Rook, Square::A1, Square::D1),
                    Square::G8 => self.move_piece(us, Piece::Rook, Square::H8, Square::F8),
                    Square::C8 => self.move_piece(us, Piece::Rook, Square::A8, Square::D8),
                    _ => panic!("Invalid king move during castling. {from} -> {to}"),
                }
            }
        } else {
            let promoted = mv.promoted();
            let is_promotion = promoted != Piece::None;

            // if the move is a pawn move, handle the promotion case and reset
            // the halfmove clock
            self.remove_piece(us, piece, from);
            self.set_piece(us, if !is_promotion { piece } else { promoted }, to);
            self.state.halfmoves = 0;

            // if the move is an en passant capture, remove the opponent's pawn
            if mv.is_en_passant() {
                self.remove_piece(opponent, Piece::Pawn, to ^ 8);
            }
        }

        // if the move is a double step, set the en passant square, otherwise
        // clear it
        if mv.is_double_step() {
            self.set_en_passant(to ^ 8);
        } else {
            self.clear_en_passant();
        }

        // swap the side to move
        self.swap_sides();
    }

    // unmake unmakes the last move on the board
    //
    // Note: since unmake pops from the history, we don't need to recompute
    //       any incremental game state since those are retrieved directly
    //
    // @param: self - mutable reference to the board
    // @return: void
    // @side-effects: modifies the `board`
    pub fn unmake_move(&mut self) {
        // Set the previous game state from the game state history. If
        // there is none (because we're at the starting position), we can
        // immediately return without unmaking a move.
        if let Some(gs) = self.history.pop() {
            self.state = gs;
        } else {
            return;
        }

        // Set "us" and "opponent"
        let us = self.turn();
        let opponent = self.opponent();

        // Dissect the move to undo
        let mv = self.state.next_move;
        let piece = mv.piece();
        let from = mv.from();
        let to = mv.to();
        let captured = mv.captured();
        let promoted = mv.promoted();
        let castling = mv.is_castle();
        let en_passant = mv.is_en_passant();

        // Moving backwards...
        if promoted == Piece::None {
            self.move_piece_no_incrementals(us, piece, to, from);
        } else {
            self.remove_piece_no_incrementals(us, promoted, to);
            self.set_piece_no_incrementals(us, Piece::Pawn, from);
        }

        // The king's move was already undone as a normal move.
        // Now undo the correct castling rook move.
        if castling {
            match to {
                Square::G1 => {
                    self.move_piece_no_incrementals(us, Piece::Rook, Square::F1, Square::H1)
                }
                Square::C1 => {
                    self.move_piece_no_incrementals(us, Piece::Rook, Square::D1, Square::A1)
                }
                Square::G8 => {
                    self.move_piece_no_incrementals(us, Piece::Rook, Square::F8, Square::H8)
                }
                Square::C8 => {
                    self.move_piece_no_incrementals(us, Piece::Rook, Square::D8, Square::A8)
                }
                _ => panic!("Error: Reversing castling rook move."),
            };
        }

        // If a piece was captured, put it back onto the to-square
        if captured != Piece::None {
            self.set_piece_no_incrementals(opponent, captured, to);
        }

        // If this was an e-passant move, put the opponent's pawn back
        if en_passant {
            self.set_piece_no_incrementals(opponent, Piece::Pawn, to ^ 8);
        }
    }
}
