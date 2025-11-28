use crate::board::board::Board;
use crate::primitives::moves::Move;
use crate::primitives::{CastleFlags, Castling, Pieces, Sides, Square, Squares};

// TODO: refactor all of this
impl Board {
    // make_move makes the given move on the board
    //
    // @param: self - mutable reference to the board
    // @param: m - move to make
    // @return: void
    // @side-effects: modifies the `board`
    pub fn make_move(&mut self, m: Move) {
        // Create the unmake info and store it.
        let mut current_state = self.state;
        current_state.next_move = m;
        self.history.push(current_state);

        // Set "us" and "opponent"
        let us = self.turn();
        let opponent = self.opponent();

        // Dissect the move so we don't need "m.function()" and type casts everywhere.
        let piece = m.piece();
        let from = m.from();
        let to = m.to();
        let captured = m.captured();
        let promoted = m.promoted();
        let castling = m.castling();
        let double_step = m.double_step();
        let en_passant = m.en_passant();

        // Shorthands
        let is_promotion = promoted != Pieces::NONE;
        let is_capture = captured != Pieces::NONE;
        let has_permissions = self.state.castling != Castling::none();

        // Assume this is not a pawn move or a capture.
        self.state.halfmoves += 1;

        // Every move except double_step unsets the ep-square.
        if self.state.en_passant.is_some() {
            self.clear_en_passant();
        }

        // If a piece was captured with this move then remove it. Also reset half_move_clock.
        if is_capture {
            self.remove_piece(opponent, captured, to);
            self.state.halfmoves = 0;
            // Change castling permissions on rook capture in the corner.
            if captured == Pieces::ROOK && has_permissions {
                let revoked_perms = CastleFlags::ALL & !match from {
                    Squares::A1 => CastleFlags::WHITE_QUEEN,
                    Squares::H1 => CastleFlags::WHITE_KING,
                    Squares::A8 => CastleFlags::BLACK_QUEEN,
                    Squares::H8 => CastleFlags::BLACK_KING,
                    _ => CastleFlags::NONE,
                };
                self.set_castling(self.state.castling & revoked_perms);
            }
        }

        // Make the move. Just move the piece if it's not a pawn.
        if piece != Pieces::PAWN {
            self.move_piece(us, piece, from, to);
        } else {
            // It's a pawn move. Take promotion into account and reset half_move_clock.
            self.remove_piece(us, piece, from);
            self.set_piece(us, if !is_promotion { piece } else { promoted }, to);
            self.state.halfmoves = 0;

            // After an en-passant maneuver, the opponent's pawn must also be removed.
            if en_passant {
                self.remove_piece(opponent, Pieces::PAWN, to ^ 8);
            }

            // A double-step is the only move that sets the ep-square.
            if double_step {
                self.set_en_passant(to ^ 8);
            }
        }

        // Remove castling permissions if king/rook leaves from starting square.
        // (This will also adjust permissions when castling, because the king moves.)
        if (piece == Pieces::KING || piece == Pieces::ROOK) && has_permissions {
            let revoked_perms = CastleFlags::ALL & !match from {
                Squares::A1 => CastleFlags::WHITE_QUEEN,
                Squares::E1 => CastleFlags::WHITE,
                Squares::H1 => CastleFlags::WHITE_KING,
                Squares::A8 => CastleFlags::BLACK_QUEEN,
                Squares::E8 => CastleFlags::BLACK,
                Squares::H8 => CastleFlags::BLACK_KING,
                _ => CastleFlags::NONE,
            };
            self.set_castling(self.state.castling & revoked_perms);
        }

        // If the king is castling, then also move the rook.
        if castling {
            match to {
                Squares::G1 => self.move_piece(us, Pieces::ROOK, Squares::H1, Squares::F1),
                Squares::C1 => self.move_piece(us, Pieces::ROOK, Squares::A1, Squares::D1),
                Squares::G8 => self.move_piece(us, Pieces::ROOK, Squares::H8, Squares::F8),
                Squares::C8 => self.move_piece(us, Pieces::ROOK, Squares::A8, Squares::D8),
                _ => panic!("Invalid king move during castling. {from} -> {to}"),
            }
        }

        // Swap the side to move.
        self.swap_sides();

        // Increase full move number if black has moved
        if us == Sides::BLACK {
            self.state.fullmoves += 1;
        }
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
        let m = self.state.next_move;
        let piece = m.piece();
        let from = m.from();
        let to = m.to();
        let captured = m.captured();
        let promoted = m.promoted();
        let castling = m.castling();
        let en_passant = m.en_passant();

        // Moving backwards...
        if promoted == Pieces::NONE {
            self.move_piece_no_incrementals(us, piece, to, from);
        } else {
            self.remove_piece_no_incrementals(us, promoted, to);
            self.set_piece_no_incrementals(us, Pieces::PAWN, from);
        }

        // The king's move was already undone as a normal move.
        // Now undo the correct castling rook move.
        if castling {
            match to {
                Squares::G1 => {
                    self.move_piece_no_incrementals(us, Pieces::ROOK, Squares::F1, Squares::H1)
                }
                Squares::C1 => {
                    self.move_piece_no_incrementals(us, Pieces::ROOK, Squares::D1, Squares::A1)
                }
                Squares::G8 => {
                    self.move_piece_no_incrementals(us, Pieces::ROOK, Squares::F8, Squares::H8)
                }
                Squares::C8 => {
                    self.move_piece_no_incrementals(us, Pieces::ROOK, Squares::D8, Squares::A8)
                }
                _ => panic!("Error: Reversing castling rook move."),
            };
        }

        // If a piece was captured, put it back onto the to-square
        if captured != Pieces::NONE {
            self.set_piece_no_incrementals(opponent, captured, to);
        }

        // If this was an e-passant move, put the opponent's pawn back
        if en_passant {
            self.set_piece_no_incrementals(opponent, Pieces::PAWN, to ^ Square::new(8));
        }
    }
}
