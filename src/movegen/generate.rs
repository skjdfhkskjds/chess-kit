use crate::attack_table::AttackTable;
use crate::movegen::{MoveGenerator, SideToMove};
use crate::position::Position;
use crate::primitives::{
    BITBOARD_RANKS, BITBOARD_SQUARES, Bitboard, Black, Move, MoveList, MoveType, Pieces, Side,
    SideRanks, Sides, Square, White,
};

// list of pieces that a pawn can promote to
const PROMOTION_PIECES: [Pieces; 4] = [Pieces::Queen, Pieces::Rook, Pieces::Bishop, Pieces::Knight];

impl<A: AttackTable> MoveGenerator<A> {
    // generate_moves generates all the pseudo-legal moves of the given move type
    // from the current position and pushes them to the move list
    //
    // @param: position - immutable reference to the position
    // @param: list - mutable reference to the move list
    // @param: move_type - move type to generate moves for
    // @return: void
    // @side-effects: modifies the `move list`
    pub fn generate_moves(&self, position: &Position, list: &mut MoveList, move_type: MoveType) {
        match position.turn() {
            Sides::White => self.generate_moves_for_side::<White>(position, list, move_type),
            Sides::Black => self.generate_moves_for_side::<Black>(position, list, move_type),
        }
    }

    // generate_moves_for_side generates all the pseudo-legal moves of the given
    // move type for the side to move from the current position and pushes them to
    // the move list
    //
    // @param: position - immutable reference to the position
    // @param: list - mutable reference to the move list
    // @param: move_type - move type to generate moves for
    // @return: void
    // @side-effects: modifies the `move list`
    fn generate_moves_for_side<S: SideToMove>(
        &self,
        position: &Position,
        list: &mut MoveList,
        move_type: MoveType,
    ) {
        self.generate_moves_for_piece::<S>(position, Pieces::King, list, move_type);
        self.generate_moves_for_piece::<S>(position, Pieces::Knight, list, move_type);
        self.generate_moves_for_piece::<S>(position, Pieces::Rook, list, move_type);
        self.generate_moves_for_piece::<S>(position, Pieces::Bishop, list, move_type);
        self.generate_moves_for_piece::<S>(position, Pieces::Queen, list, move_type);
        self.generate_moves_for_piece::<S>(position, Pieces::Pawn, list, move_type);

        if move_type == MoveType::All || move_type == MoveType::Quiet {
            self.generate_castle_moves::<S>(position, list);
        }
    }

    // generate_moves_for_piece generates all the pseudo-legal moves of the given
    // move type for the given piece from the current position and pushes them to
    // the move list
    //
    // @param: position - immutable reference to the position
    // @param: piece - piece to generate moves of
    // @param: list - mutable reference to the move list
    // @param: move_type - move type to generate moves of
    // @return: void
    // @side-effects: modifies the `move list`
    fn generate_moves_for_piece<S: SideToMove>(
        &self,
        position: &Position,
        piece: Pieces,
        list: &mut MoveList,
        move_type: MoveType,
    ) {
        // if the piece is a pawn,
        if matches!(piece, Pieces::Pawn) {
            return self.generate_pawn_moves::<S>(position, list, move_type);
        }

        let occupancy = position.occupancy::<S>();
        let empty_squares = position.empty_squares::<S>();
        let our_occupancy = position.sides[S::INDEX];
        let opponent_occupancy = position.sides[S::Other::INDEX];

        // generate moves from all positions of the piece for the current side
        // to move
        let to_move = position.get_piece::<S>(piece);
        for from in to_move.iter() {
            let targets = match piece {
                Pieces::King => self.attack_table.king_targets(from),
                Pieces::Knight => self.attack_table.knight_targets(from),
                Pieces::Bishop => self.attack_table.bishop_targets(from, &occupancy),
                Pieces::Rook => self.attack_table.rook_targets(from, &occupancy),
                Pieces::Queen => self.attack_table.queen_targets(from, &occupancy),
                _ => unreachable!("Not a valid piece: {piece}"),
            };

            // filter the moves according to the requested move type
            let moves = match move_type {
                MoveType::All => targets & !our_occupancy,
                MoveType::Quiet => targets & empty_squares,
                MoveType::Capture => targets & opponent_occupancy,
            };

            self.push_moves::<S>(position, piece, from, moves, list);
        }
    }

    // generate_pawn_moves generates all the pseudo-legal moves of the given
    // move type for the pawns from the current position and pushes them to the
    // move list
    //
    // @param: position - immutable reference to the position
    // @param: list - mutable reference to the move list
    // @param: move_type - move type to generate moves of
    // @return: void
    // @side-effects: modifies the `move list`
    fn generate_pawn_moves<S: SideToMove>(
        &self,
        position: &Position,
        list: &mut MoveList,
        move_type: MoveType,
    ) {
        let empty_squares = position.empty_squares::<S>();
        let double_step_rank = BITBOARD_RANKS[S::DOUBLE_STEP_RANK.idx()];

        // generate moves for each of the pawns
        let pawn_squares = position.get_piece::<S>(Pieces::Pawn);
        for from in pawn_squares.iter() {
            let to = (from.idx() as i8 + S::PAWN_PUSH_OFFSET) as usize;
            let mut moves = Bitboard::empty();

            // generate pawn pushes
            if move_type == MoveType::All || move_type == MoveType::Quiet {
                let single_step = BITBOARD_SQUARES[to] & empty_squares;
                let double_step = single_step.rotate_left(S::PAWN_DOUBLE_STEP_OFFSET)
                    & empty_squares
                    & double_step_rank;
                moves |= single_step | double_step;
            }

            // generate pawn captures
            if move_type == MoveType::All || move_type == MoveType::Capture {
                let targets = self.attack_table.pawn_targets::<S>(from);
                let captures = targets & position.sides[S::Other::INDEX];
                let en_passant_captures = match position.state.en_passant {
                    Some(ep) => targets & BITBOARD_SQUARES[ep.idx()],
                    None => Bitboard::empty(),
                };
                moves |= captures | en_passant_captures;
            }

            self.push_moves::<S>(position, Pieces::Pawn, from, moves, list);
        }
    }

    // generate_castle_moves generates all the pseudo-legal moves of the given
    // move type for the castling from the current position and pushes them to the
    // move list
    //
    // @param: position - immutable reference to the position
    // @param: list - mutable reference to the move list
    // @return: void
    // TODO: current implementation does not support chess960, as it assumes the
    //       squares along the path from the king and rook
    fn generate_castle_moves<S: SideToMove>(&self, position: &Position, list: &mut MoveList) {
        // get the castling rights for the side to move
        let (kingside, queenside) = (
            position.state.castling.kingside::<S>(),
            position.state.castling.queenside::<S>(),
        );

        // get the current king square
        let from = position.king_square::<S>();

        // check if the side to move can castle
        //
        // Note: a side can castle iff they have either kingside or queenside
        //       permissions and they are not currently in check
        if !(kingside || queenside) || self.attack_table.is_attacked::<S>(position, from) {
            return;
        }

        // generate castle moves depending on the side to move
        let occupancy = position.occupancy::<S>();
        let mut moves = Bitboard::empty();

        if kingside {
            // get the blockers (squares in between the king and the rook)
            let blockers = BITBOARD_SQUARES[S::KINGSIDE_DESTINATION.idx()]
                | BITBOARD_SQUARES[S::KINGSIDE_ROOK_DESTINATION.idx()];

            // if the squares along the path are empty and the king is not moving
            // "through" check, we can castle
            if (occupancy & blockers).is_empty()
                && !self
                    .attack_table
                    .is_attacked::<S>(position, S::KINGSIDE_ROOK_DESTINATION)
            {
                moves |= BITBOARD_SQUARES[S::KINGSIDE_DESTINATION.idx()];
            }
        }

        if queenside {
            // same as in the kingside implementation
            //
            // Note: the queenside blockers include an additional square, see
            //       `QUEENSIDE_ROOK_INTERMEDIATE` for more details.
            let blockers = BITBOARD_SQUARES[S::QUEENSIDE_DESTINATION.idx()]
                | BITBOARD_SQUARES[S::QUEENSIDE_ROOK_DESTINATION.idx()]
                | BITBOARD_SQUARES[S::QUEENSIDE_ROOK_INTERMEDIATE.idx()];

            if (occupancy & blockers).is_empty()
                && !self
                    .attack_table
                    .is_attacked::<S>(position, S::QUEENSIDE_ROOK_DESTINATION)
            {
                moves |= BITBOARD_SQUARES[S::QUEENSIDE_DESTINATION.idx()];
            }
        }

        // push the castle moves to the move list
        self.push_moves::<S>(position, Pieces::King, from, moves, list);
    }

    // push_moves pushes a set of moves to the move list as defined by the
    // given piece at the from square to the each of the to squares.
    //
    // @param: position - immutable reference to the position
    // @param: piece - piece to push the moves for
    // @param: from - square to push the moves from
    // @param: to_squares - bitboard of squares to push the moves to
    // @param: list - mutable reference to the move list
    // @return: void
    // @side-effects: modifies the `move list`
    fn push_moves<S: SideRanks>(
        &self,
        position: &Position,
        piece: Pieces,
        from: Square,
        to_squares: Bitboard,
        list: &mut MoveList,
    ) {
        // push a move for each of the `to` squares
        for to in to_squares.iter() {
            let mut mv = Move::new(piece, from, to);

            // set the captured piece for the move if there is one
            //
            // Note: a captured piece is the piece that currently occupies the
            //       target square. Notice that this definition excludes en-passant
            //       captures.
            let captured = position.pieces[to.idx()];
            if !matches!(captured, Pieces::None) {
                mv = mv.with_capture(captured);
            }

            // handle the special cases for the piece
            match piece {
                Pieces::Pawn => {
                    // a pawn is moving, so we need to handle the cases
                    //
                    // 1. en passant capture
                    // 2. double step pawn push
                    // 3. promotion

                    // check if the move is an en passant capture
                    let is_en_passant = match position.state.en_passant {
                        Some(square) => square == to,
                        None => false,
                    };

                    if is_en_passant {
                        // the move is an en passant capture
                        mv = mv.with_en_passant();
                    } else if to.distance(from) == 16 {
                        // the move is a double step pawn push
                        mv = mv.with_double_step();
                    } else if to.on_rank(S::PROMOTION_RANK) {
                        // generate all possible promotion moves
                        PROMOTION_PIECES.iter().for_each(|promotion_piece| {
                            list.push(mv.with_promotion(*promotion_piece));
                        });

                        // all move variants have been generated, move on to the
                        // next move instead of exiting out of the conditional
                        // block
                        continue;
                    }
                }
                Pieces::King => {
                    // check if the move is a castle
                    if to.distance(from) == 2 {
                        mv = mv.with_castle();
                    }
                }
                _ => {
                    // no special handling required for other pieces
                }
            }

            // push the move to the list
            list.push(mv);
        }
    }
}
