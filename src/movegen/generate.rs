use crate::attack_table::AttackTable;
use crate::movegen::{MoveGenerator, SideToMove};
use crate::position::Position;
use crate::primitives::{
    BITBOARD_RANKS, BITBOARD_SQUARES, Bitboard, Black, MoveList, MoveType, Pieces, Side, Sides,
    State, White,
};

impl<A: AttackTable> MoveGenerator<A> {
    // generate_moves generates all the pseudo-legal moves of the given move type
    // from the current position and pushes them to the move list
    //
    // @param: position - immutable reference to the position
    // @param: list - mutable reference to the move list
    // @param: move_type - move type to generate moves for
    // @return: void
    // @side-effects: modifies the `move list`
    pub fn generate_moves<StateT: State>(
        &self,
        position: &Position<StateT>,
        list: &mut MoveList,
        move_type: MoveType,
    ) {
        match position.turn() {
            Sides::White => {
                self.generate_moves_for_side::<White, StateT>(position, list, move_type)
            }
            Sides::Black => {
                self.generate_moves_for_side::<Black, StateT>(position, list, move_type)
            }
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
    fn generate_moves_for_side<SideT: SideToMove, StateT: State>(
        &self,
        position: &Position<StateT>,
        list: &mut MoveList,
        move_type: MoveType,
    ) {
        self.generate_moves_for_piece::<SideT, StateT>(position, Pieces::King, list, move_type);
        self.generate_moves_for_piece::<SideT, StateT>(position, Pieces::Knight, list, move_type);
        self.generate_moves_for_piece::<SideT, StateT>(position, Pieces::Rook, list, move_type);
        self.generate_moves_for_piece::<SideT, StateT>(position, Pieces::Bishop, list, move_type);
        self.generate_moves_for_piece::<SideT, StateT>(position, Pieces::Queen, list, move_type);
        self.generate_moves_for_piece::<SideT, StateT>(position, Pieces::Pawn, list, move_type);

        if move_type == MoveType::All || move_type == MoveType::Quiet {
            self.generate_castle_moves::<SideT, StateT>(position, list);
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
    fn generate_moves_for_piece<SideT: SideToMove, StateT: State>(
        &self,
        position: &Position<StateT>,
        piece: Pieces,
        list: &mut MoveList,
        move_type: MoveType,
    ) {
        // if the piece is a pawn,
        if matches!(piece, Pieces::Pawn) {
            return self.generate_pawn_moves::<SideT, StateT>(position, list, move_type);
        }

        let occupancy = position.occupancy::<SideT>();
        let empty_squares = position.empty_squares::<SideT>();
        let our_occupancy = position.sides[SideT::INDEX];
        let opponent_occupancy = position.sides[SideT::Other::INDEX];

        // generate moves from all positions of the piece for the current side
        // to move
        let to_move = position.get_piece::<SideT>(piece);
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

            self.push_moves::<SideT, StateT>(position, piece, from, moves, list);
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
    fn generate_pawn_moves<SideT: SideToMove, StateT: State>(
        &self,
        position: &Position<StateT>,
        list: &mut MoveList,
        move_type: MoveType,
    ) {
        let en_passant = position.state.en_passant();
        let empty_squares = position.empty_squares::<SideT>();
        let double_step_rank = BITBOARD_RANKS[SideT::DOUBLE_STEP_RANK.idx()];

        // generate moves for each of the pawns
        let pawn_squares = position.get_piece::<SideT>(Pieces::Pawn);
        for from in pawn_squares.iter() {
            let to = (from.idx() as i8 + SideT::PAWN_PUSH_OFFSET) as usize;
            let mut moves = Bitboard::empty();

            // generate pawn pushes
            if move_type == MoveType::All || move_type == MoveType::Quiet {
                let single_step = BITBOARD_SQUARES[to] & empty_squares;
                let double_step = single_step.rotate_left(SideT::PAWN_DOUBLE_STEP_OFFSET)
                    & empty_squares
                    & double_step_rank;
                moves |= single_step | double_step;
            }

            // generate pawn captures
            if move_type == MoveType::All || move_type == MoveType::Capture {
                let targets = self.attack_table.pawn_targets::<SideT>(from);
                let captures = targets & position.sides[SideT::Other::INDEX];
                let en_passant_captures = match en_passant {
                    Some(ep) => targets & BITBOARD_SQUARES[ep.idx()],
                    None => Bitboard::empty(),
                };
                moves |= captures | en_passant_captures;
            }

            self.push_moves::<SideT, StateT>(position, Pieces::Pawn, from, moves, list);
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
    fn generate_castle_moves<SideT: SideToMove, StateT: State>(
        &self,
        position: &Position<StateT>,
        list: &mut MoveList,
    ) {
        // get the castling rights for the side to move
        let castling = position.state.castling();
        let (kingside, queenside) = (castling.kingside::<SideT>(), castling.queenside::<SideT>());

        // get the current king square
        let from = position.king_square::<SideT>();

        // check if the side to move can castle
        //
        // Note: a side can castle iff they have either kingside or queenside
        //       permissions and they are not currently in check
        if !(kingside || queenside)
            || self
                .attack_table
                .is_attacked::<SideT, StateT>(position, from)
        {
            return;
        }

        // generate castle moves depending on the side to move
        let occupancy = position.occupancy::<SideT>();
        let mut moves = Bitboard::empty();

        if kingside {
            // get the blockers (squares in between the king and the rook)
            let blockers = BITBOARD_SQUARES[SideT::KINGSIDE_DESTINATION.idx()]
                | BITBOARD_SQUARES[SideT::KINGSIDE_ROOK_DESTINATION.idx()];

            // if the squares along the path are empty and the king is not moving
            // "through" check, we can castle
            if (occupancy & blockers).is_empty()
                && !self
                    .attack_table
                    .is_attacked::<SideT, StateT>(position, SideT::KINGSIDE_ROOK_DESTINATION)
            {
                moves |= BITBOARD_SQUARES[SideT::KINGSIDE_DESTINATION.idx()];
            }
        }

        if queenside {
            // same as in the kingside implementation
            //
            // Note: the queenside blockers include an additional square, see
            //       `QUEENSIDE_ROOK_INTERMEDIATE` for more details.
            let blockers = BITBOARD_SQUARES[SideT::QUEENSIDE_DESTINATION.idx()]
                | BITBOARD_SQUARES[SideT::QUEENSIDE_ROOK_DESTINATION.idx()]
                | BITBOARD_SQUARES[SideT::QUEENSIDE_ROOK_INTERMEDIATE.idx()];

            if (occupancy & blockers).is_empty()
                && !self
                    .attack_table
                    .is_attacked::<SideT, StateT>(position, SideT::QUEENSIDE_ROOK_DESTINATION)
            {
                moves |= BITBOARD_SQUARES[SideT::QUEENSIDE_DESTINATION.idx()];
            }
        }

        // push the castle moves to the move list
        self.push_moves::<SideT, StateT>(position, Pieces::King, from, moves, list);
    }
}
