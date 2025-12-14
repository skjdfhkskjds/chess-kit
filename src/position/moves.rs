use crate::attack_table::AttackTable;
use crate::position::{Position, SideCastlingSquares};
use crate::primitives::{
    Bitboard, Black, GameStateExt, Move, MoveType, Pieces, Side, Sides, Square, State, White,
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
    // @side-effects: optionally revokes castling permissions
    // @side-effects: optionally sets the en passant square
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
    // @side-effects: reverts the `state` back to the previous state
    pub fn unmake_move(&mut self, mv: Move) {
        match self.turn() {
            Sides::White => self.unmake_move_for_side::<Black>(mv),
            Sides::Black => self.unmake_move_for_side::<White>(mv),
        }
    }

    // move_piece_no_incrementals moves SideT's piece from the given square to
    // the given square without updating the zobrist key or any incremental game
    // state
    //
    // @param: piece - piece to move
    // @param: from - square to move the piece from
    // @param: to - square to move the piece to
    // @return: void
    // @side-effects: modifies the `position`
    #[inline(always)]
    fn move_piece_no_incrementals<SideT: Side>(&mut self, piece: Pieces, from: Square, to: Square) {
        let from_to = Bitboard::square(from) | Bitboard::square(to);

        self.bitboards[SideT::INDEX][piece.idx()] ^= from_to;
        self.sides[SideT::INDEX] ^= from_to;
        self.sides[Sides::TOTAL] ^= from_to;
        self.pieces[from.idx()] = Pieces::None;
        self.pieces[to.idx()] = piece;
    }

    // move_piece moves SideT's piece from the given square to the given square
    //
    // @param: piece - piece to move
    // @param: from - square to move the piece from
    // @param: to - square to move the piece to
    // @return: void
    // @side-effects: modifies the `position`
    #[inline(always)]
    fn move_piece<SideT: SideCastlingSquares>(&mut self, piece: Pieces, from: Square, to: Square) {
        self.move_piece_no_incrementals::<SideT>(piece, from, to);
        let key = self.zobrist.piece::<SideT>(piece, from) ^ self.zobrist.piece::<SideT>(piece, to);
        self.state_mut().update_key(key);
    }

    // capture_piece captures SideT's piece at the given square
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

    // is_preventing_check returns true if the given square is preventing a
    // check from SideT::Other to SideT
    //
    // that is, if, theoretically, the piece on the given square were to move,
    // it would result in SideT being in check
    //
    // note: the piece on the given square is not necessarily on the same side
    //       as the king it is blocking the check from
    //
    // @param: square - square to check
    // @return: true if the given square is preventing a check to SideT
    #[inline(always)]
    fn is_preventing_check<SideT: Side>(&self, square: Square) -> bool {
        self.state()
            .king_blocker_pieces::<SideT>()
            .has_square(square)
    }

    // update_blockers updates the blockers for SideT and pinners for
    // SideT::Other at the king square of SideT
    //
    // @return: void
    // @side-effects: modifies the `state`
    #[inline(always)]
    fn update_blockers<SideT: Side>(&mut self) {
        let king_square = self.king_square::<SideT>();

        // blockers is the occupied squares preventing the king square from
        // being attacked by the snipers of SideT::Other
        let mut blockers = Bitboard::empty();

        // pinners is the occupied squares of SideT::Other that are pinning a
        // piece of SideT to the king square of SideT
        let mut pinners = Bitboard::empty();

        // get the snipers of SideT::Other that can see the king square
        let snipers = self.is_sniped_by::<SideT>(king_square);

        // get the occupancy of the board excluding the snipers
        let occupancy = self.total_occupancy() ^ snipers;

        for sniper in snipers.iter() {
            // between contains the occupied squares between the king square
            // and the sniper, excluding the snipers themselves
            let between = Bitboard::between(king_square, sniper) & occupancy;

            // if the between bitboard has exactly one bit set, then it is a
            // blocker to the king square
            if between.exactly_one() {
                blockers |= between;

                // if the blocker is on SideT, then the sniper is pinning the
                // blocker piece to the king
                if self.occupancy::<SideT>().intersects(between) {
                    pinners.set_at(sniper);
                }
            }
        }

        // set the blockers and pinners in the state
        self.state_mut().set_king_blocker_pieces::<SideT>(blockers);
        self.state_mut().set_pinning_pieces::<SideT::Other>(pinners);
    }

    // update_check_info updates the check information for SideT to deliver
    // check to SideT::Other
    //
    // @return: void
    // @side-effects: modifies the `state`
    #[inline(always)]
    pub(crate) fn update_check_info<SideT: Side>(&mut self) {
        // update the blockers and pinners for each side
        self.update_blockers::<White>();
        self.update_blockers::<Black>();

        let king_square = self.king_square::<SideT::Other>();
        let occupancy = self.total_occupancy();

        // calculate the set of squares that each piece would have to be on to
        // deliver check to SideT::Other
        //
        // again, the pattern is to calculate all the possible source squares
        // for each piece type, and we later use this mask to determine if a
        // move delivers check by the intersection of that piece's destination
        // square and the mask
        //
        // see `Position::is_attacked_by` for more details on the pattern
        let pawn_targets = AT::pawn_targets::<SideT::Other>(king_square);
        let knight_targets = AT::knight_targets(king_square);
        let rook_targets = AT::rook_targets(king_square, occupancy);
        let bishop_targets = AT::bishop_targets(king_square, occupancy);
        let queen_targets = rook_targets | bishop_targets;

        // set the squares that would deliver check to SideT::Other for each
        // piece type in the state
        //
        // note: the king can never deliver check to SideT::Other, so pass an
        //       empty bitboard for the king
        self.state_mut()
            .set_check_squares::<SideT>(Pieces::Pawn, pawn_targets);
        self.state_mut()
            .set_check_squares::<SideT>(Pieces::Knight, knight_targets);
        self.state_mut()
            .set_check_squares::<SideT>(Pieces::Rook, rook_targets);
        self.state_mut()
            .set_check_squares::<SideT>(Pieces::Bishop, bishop_targets);
        self.state_mut()
            .set_check_squares::<SideT>(Pieces::Queen, queen_targets);
        self.state_mut()
            .set_check_squares::<SideT>(Pieces::King, Bitboard::empty());
    }

    // is_legal_move checks if the given move played by SideT is legal
    //
    // note: this method does not check that the king is still in check after
    //       the move is made, instead, we delegate this logic to the move
    //       generator. as a result, this method only checks that a king not
    //       currently in check would be left in check after the move is made
    //
    // @param: mv - move to check if is legal
    // @return: true if the move is legal, false otherwise
    #[inline(always)]
    pub fn is_legal_move<SideT: Side>(&self, mv: Move) -> bool {
        let from = mv.from();
        let to = mv.to();

        // if the move is an en passant capture, check whether or not the king
        // is attacked by SideT::Other's sliders
        //
        // note: skip non-sliding attacks since an en passant capture is only
        //       possible if there are no other pieces delivering check other
        //       than the pawn to be captured
        if matches!(mv.type_of(), MoveType::EnPassant) {
            let en_passant_square = AT::pawn_pushes::<SideT::Other>(to);
            let occupancy = (self.total_occupancy() ^ Bitboard::square(from) ^ en_passant_square)
                | Bitboard::square(to);

            return !self.is_attacked_by_sliders::<SideT>(self.king_square::<SideT>(), occupancy);
        }

        // if the moving piece is a king, check whether or not the destination
        // square is attacked by SideT::Other
        if matches!(self.piece_at(from), Pieces::King) {
            let occupancy = self.total_occupancy() ^ Bitboard::square(from);
            return !self.is_attacked::<SideT>(to, occupancy);
        }

        // otherwise, any non-king move is legal iff it is not pinned or it is
        // moving along the line on which it is pinned
        return !self.is_preventing_check::<SideT>(from)
            || Bitboard::in_line(from, to, self.king_square::<SideT>());
    }

    // delivers_check checks if the given move played by SideT delivers a check
    // to SideT::Other
    //
    // @param: mv - move to check if delivers a check
    // @return: true if the move delivers a check, false otherwise
    #[inline(always)]
    pub fn delivers_check<SideT: SideCastlingSquares>(&self, mv: Move) -> bool {
        let from = mv.from();
        let to = mv.to();
        let piece = self.piece_at(from);

        // check if the move directly delivers a check to SideT::Other
        //
        // note: this is the moment where we leverage the precomputed check
        //       squares to determine if the move directly gives a check
        //
        // see `Position::update_check_info` for more details on the precomputed
        // check squares
        if !matches!(piece, Pieces::King)
            && self.state().check_squares::<SideT>(piece).has_square(to)
        {
            return true;
        }

        // check if the move possibly enables a discovered check to SideT::Other
        //
        // that is, if the piece moved from a square that was preventing a check
        // to SideT::Other to a new square not on the same ray that it was on to
        // block the check, then SideT::Other is now in check via the sniper that
        // was previously blocked by the piece that moved
        //
        // note: a castle move where SideT's king was the blocker will result in
        //       a discovered check, so we can return true immediately
        if self.is_preventing_check::<SideT::Other>(from) {
            return matches!(mv.type_of(), MoveType::Castle)
                || !Bitboard::in_line(from, to, self.king_square::<SideT::Other>());
        }

        match mv.type_of() {
            MoveType::Normal => false,
            MoveType::Promotion => {
                // if the move is a promotion, check if the promoted piece would
                // deliver a check to SideT::Other
                //
                // note: exclude the current occupancy of the pawn to promote
                let targets = match mv.promoted_to() {
                    Pieces::Knight => AT::knight_targets(to),
                    Pieces::Bishop => {
                        AT::bishop_targets(to, self.total_occupancy() ^ Bitboard::square(from))
                    }
                    Pieces::Rook => {
                        AT::rook_targets(to, self.total_occupancy() ^ Bitboard::square(from))
                    }
                    Pieces::Queen => {
                        AT::queen_targets(to, self.total_occupancy() ^ Bitboard::square(from))
                    }
                    _ => unreachable!("promotion to non-promotable piece"),
                };

                // check if the opponent's king is being attacked by the promoted
                // piece
                targets.has_square(self.king_square::<SideT::Other>())
            }
            MoveType::EnPassant => {
                let captured_square = AT::pawn_pushes::<SideT::Other>(to);

                // invariant checks for the captured square
                debug_assert!(
                    captured_square.exactly_one(),
                    "captured square should be exactly one"
                );
                debug_assert!(
                    captured_square.must_first().file() == to.file(),
                    "captured square and the square that the pawn just moved to should be on the same file"
                );
                debug_assert!(
                    captured_square.must_first().distance(to) == 8,
                    "captured square and the square that the pawn just moved to should be one rank apart"
                );

                let occupancy = (self.total_occupancy() ^ Bitboard::square(from) ^ captured_square)
                    | Bitboard::square(to);

                // check if the opponent's king is being attacked by a discovered
                // check via SideT's sliding pieces
                //
                // note: we can ignore the possibility of check with knights or
                //       pawns since an en passant capture is not possible if a
                //       non-sliding piece is currently delivering check
                self.is_attacked_by_sliders::<SideT::Other>(
                    self.king_square::<SideT::Other>(),
                    occupancy,
                )
            }
            MoveType::Castle => {
                let rook_square = if to == SideT::KINGSIDE_DESTINATION {
                    SideT::KINGSIDE_ROOK_DESTINATION
                } else {
                    SideT::QUEENSIDE_ROOK_DESTINATION
                };

                // check if the opponent's king is being attacked by the rook
                // after the castle
                self.state()
                    .check_squares::<SideT>(Pieces::Rook)
                    .has_square(rook_square)
            }
        }
    }

    // make_move_for_side makes the given move from the current position as SideT
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
        // TODO: move the delivers check logic outside of the make move logic
        let delivers_check = self.delivers_check::<SideT>(mv);

        // push the current state into the history
        self.history.push_next();

        // helper variables to avoid repeated calls
        let from = mv.from();
        let to = mv.to();
        let piece = self.piece_at(from);
        let mut check_en_passant = false;

        // increment the move counters
        //
        // note: if black is moving, increment the fullmove counter as well
        self.state_mut().inc_halfmoves();
        if matches!(SideT::SIDE, Sides::Black) {
            self.state_mut().inc_fullmoves();
        }

        // handle a piece capture
        let captured = self.piece_at(to);
        if !matches!(captured, Pieces::None) {
            self.capture_piece::<SideT::Other>(captured, to);
        }
        // set the captured piece for the state
        self.state_mut().set_captured_piece(captured);

        // move the piece
        match piece {
            Pieces::King => {
                self.move_piece::<SideT>(Pieces::King, from, to);

                // if the move is a castle, move the appropriate rook as well
                if matches!(mv.type_of(), MoveType::Castle) {
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
            }
            Pieces::Rook => {
                self.move_piece::<SideT>(Pieces::Rook, from, to);

                // if the moving piece is a rook and that side can still castle,
                // revoke the appropriate castling permissions if the rook is
                // leaving the starting square
                if self.state().castling().can_castle::<SideT>() {
                    if from == SideT::KINGSIDE_ROOK {
                        self.set_castling(self.state().castling().revoke_kingside::<SideT>());
                    } else if from == SideT::QUEENSIDE_ROOK {
                        self.set_castling(self.state().castling().revoke_queenside::<SideT>());
                    }
                }
            }
            Pieces::Pawn => {
                match mv.type_of() {
                    MoveType::Promotion => {
                        // remove the pawn from the board and set the promoted piece
                        self.remove_piece::<SideT>(Pieces::Pawn, from);
                        self.set_piece::<SideT>(mv.promoted_to(), to);
                    }
                    MoveType::EnPassant => {
                        self.move_piece::<SideT>(Pieces::Pawn, from, to);

                        // if the move is an en passant capture, remove the opponent's pawn
                        self.remove_piece::<SideT::Other>(Pieces::Pawn, to ^ 8);
                    }
                    _ => {
                        self.move_piece::<SideT>(Pieces::Pawn, from, to);

                        // if the move is a double step, then an en passant capture may
                        // be possible, and we should check for it later
                        check_en_passant = to.distance(from) == 16;
                    }
                }

                // reset the halfmove clock since a pawn moved
                self.state_mut().set_halfmoves(0);
            }
            _ => {
                self.move_piece::<SideT>(piece, from, to);
            }
        }

        // set the checkers to SideT::Other's king square if the move gives a
        // check
        let checkers = if delivers_check {
            self.is_checked_by::<SideT::Other>()
        } else {
            Bitboard::empty()
        };
        self.state_mut().set_checkers(checkers);

        // invariant checks for the move data after making the move
        debug_assert!(
            delivers_check == self.is_checked::<SideT::Other>(),
            "delivers check flag should be equal to the checked status of SideT::Other after move: {}, in position {}",
            mv,
            self
        );
        debug_assert!(
            checkers == self.is_checked_by::<SideT::Other>(),
            "checkers should be equal to the bitboard of squares that SideT::Other is checked by after move: {}, in position {}",
            mv,
            self
        );
        debug_assert!(
            !self.is_checked::<SideT>(),
            "SideT cannot be in check after move: {}, in position {}, blockers for king: {}",
            mv,
            self,
            self.state().king_blocker_pieces::<SideT>()
        );

        // if the moving piece is a pawn, and the move is a double step, then an
        // en passant capture may be possible
        //
        // TODO: use a smarter approach to filter further whether or not an en
        //       passant capture is possible
        // note: the while loop is a hack to conditionally break out
        while check_en_passant {
            let en_passant_square = AT::pawn_pushes::<SideT::Other>(to).must_first();

            // invariant checks for the en passant square
            debug_assert!(
                AT::pawn_pushes::<SideT::Other>(to).exactly_one(),
                "en passant square should be exactly one"
            );
            debug_assert!(
                en_passant_square.file() == to.file(),
                "en passant square and the square that the pawn just moved to should be on the same file"
            );
            debug_assert!(
                en_passant_square.distance(to) == 8,
                "en passant square and the square that the pawn just moved to should be one rank apart"
            );

            let mut attacking_pawns = self.get_piece::<SideT::Other>(Pieces::Pawn)
                & AT::pawn_targets::<SideT>(en_passant_square);

            // if there are no pawns that can attack the en passant square, then
            // no en passant capture is possible
            if attacking_pawns.is_empty() {
                check_en_passant = false;
                break;
            }

            // if there are other pieces delivering check other than the pawn
            // to be captured, then en passant is illegal
            if self.state().checkers().intersects(!Bitboard::square(to)) {
                check_en_passant = false;
                break;
            }

            // if multiple (two) pawns are attacking the en passant square, then
            // we need to check some more conditions to determine if en passant
            // is legal
            if attacking_pawns.more_than_one() {
                // if neither pawn is pinned to the king, then en passant is
                // legal
                if !((self.state().king_blocker_pieces::<SideT::Other>() & attacking_pawns)
                    .more_than_one())
                {
                    self.set_en_passant(en_passant_square);
                    break;
                }

                // if both pawns are pinned to the king and neither is on the
                // same file as the king, then both pawns are pinned by bishops
                // and en passant is illegal
                let king_file = Bitboard::file(self.king_square::<SideT::Other>().file());
                if (king_file & attacking_pawns).is_empty() {
                    check_en_passant = false;
                    break;
                }

                // otherwise, there is a horizontally pinned pawn on the king's
                // file, and remove it from consideration since an en passant
                // from that pawn is illegal
                attacking_pawns &= !king_file;
            }

            debug_assert!(
                attacking_pawns.exactly_one(),
                "attacking pawns should be exactly one"
            );
            let occupancy = (self.total_occupancy() ^ Bitboard::square(to) ^ attacking_pawns)
                | Bitboard::square(en_passant_square);

            // if the king is attacked after capturing en passant, then it is
            // illegal
            if self.is_attacked_by_sliders::<SideT::Other>(
                self.king_square::<SideT::Other>(),
                occupancy,
            ) {
                check_en_passant = false;
                break;
            }

            // otherwise, en passant is legal, so set the square
            self.set_en_passant(en_passant_square);
            break;
        }

        // if after all the checks, en passant is not legal, then clear the en
        // passant square
        if !check_en_passant {
            self.clear_en_passant();
        }

        // swap the side to move
        self.swap_sides::<SideT>();

        // update the new check info for the new side to move
        self.update_check_info::<SideT::Other>();
    }

    // unmake_move_for_side unmakes the last move from the current position as
    // SideT
    //
    // note: since unmake pops from the history, we don't need to recompute
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

        // move the piece back to the original square, or restore the pawn if
        // it was promoted
        match mv.type_of() {
            MoveType::Promotion => {
                self.remove_piece_no_incrementals::<SideT>(self.piece_at(to), to);
                self.set_piece_no_incrementals::<SideT>(Pieces::Pawn, from);
            }
            MoveType::EnPassant => {
                self.move_piece_no_incrementals::<SideT>(Pieces::Pawn, to, from);

                // if the move was an en passant capture, restore the opponent's pawn
                self.set_piece_no_incrementals::<SideT::Other>(Pieces::Pawn, to ^ 8);
            }
            MoveType::Castle => {
                self.move_piece_no_incrementals::<SideT>(Pieces::King, to, from);

                // if the move was a castle, move the appropriate rook back as well
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
            _ => {
                self.move_piece_no_incrementals::<SideT>(self.piece_at(to), to, from);
            }
        }

        // if the move was a capture, restore the captured piece
        let captured = self.state().captured_piece();
        if !matches!(captured, Pieces::None) {
            self.set_piece_no_incrementals::<SideT::Other>(captured, to);
        }

        // revert the state
        debug_assert!(!self.history.is_empty(), "history is empty on unmake move");
        self.history.pop();
    }
}
