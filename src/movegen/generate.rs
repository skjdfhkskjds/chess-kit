use crate::attack_table::AttackTable;
use crate::movegen::{MoveGenerator, MoveType, SideToMove};
use crate::position::Position;
use crate::primitives::{
    Bitboard, Black, GameStateExt, MoveList, Pieces, Sides, Square, State, White,
};

impl<AT: AttackTable> MoveGenerator<AT> {
    // generate_moves generates all the pseudo-legal moves of the given move type
    // from the current position and pushes them to the move list
    //
    // @param: position - immutable reference to the position
    // @param: list - mutable reference to the move list
    // @param: move_type - move type to generate moves for
    // @return: void
    // @side-effects: modifies the `move list`
    pub fn generate_moves<StateT: State + GameStateExt>(
        &self,
        position: &Position<AT, StateT>,
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
    fn generate_moves_for_side<SideT: SideToMove, StateT: State + GameStateExt>(
        &self,
        position: &Position<AT, StateT>,
        list: &mut MoveList,
        move_type: MoveType,
    ) {
        self.generate_king_moves::<SideT, StateT>(position, list, move_type);
        self.generate_pawn_moves::<SideT, StateT>(position, list, move_type);
        self.generate_moves_for_piece::<SideT, StateT>(position, Pieces::Knight, list, move_type);
        self.generate_moves_for_piece::<SideT, StateT>(position, Pieces::Rook, list, move_type);
        self.generate_moves_for_piece::<SideT, StateT>(position, Pieces::Bishop, list, move_type);
        self.generate_moves_for_piece::<SideT, StateT>(position, Pieces::Queen, list, move_type);
    }

    // generate_moves_for_piece generates all the pseudo-legal moves of the given
    // move type for the given piece from the current position and pushes them to
    // the move list
    //
    // note: this function does not handle king or pawn move generation, as it is
    //       handled explicitly by the `generate_king_moves` and `generate_pawn_moves`
    //       functions respectively
    //
    // @param: position - immutable reference to the position
    // @param: piece - piece to generate moves of
    // @param: list - mutable reference to the move list
    // @param: move_type - move type to generate moves of
    // @return: void
    // @side-effects: modifies the `move list`
    fn generate_moves_for_piece<SideT: SideToMove, StateT: State + GameStateExt>(
        &self,
        position: &Position<AT, StateT>,
        piece: Pieces,
        list: &mut MoveList,
        move_type: MoveType,
    ) {
        // pawn and king move generation is not handled by this function
        debug_assert!(
            !matches!(piece, Pieces::Pawn | Pieces::King),
            "King/Pawn move generation is handled explicitly"
        );

        let occupancy = position.total_occupancy();
        let empty_squares = position.empty_squares();
        let our_occupancy = position.occupancy::<SideT>();
        let opponent_occupancy = position.occupancy::<SideT::Other>();

        // generate moves from all positions of the piece for the current side
        // to move
        let to_move = position.get_piece::<SideT>(piece);
        for from in to_move.iter() {
            let targets = match piece {
                Pieces::Knight => self.attack_table.knight_targets(from),
                Pieces::Bishop => self.attack_table.bishop_targets(from, occupancy),
                Pieces::Rook => self.attack_table.rook_targets(from, occupancy),
                Pieces::Queen => self.attack_table.queen_targets(from, occupancy),
                _ => unreachable!("Not a valid piece: {piece}"),
            };

            // filter the moves according to the requested move type
            let moves = match move_type {
                MoveType::All => targets & !our_occupancy,
                MoveType::Quiet => targets & empty_squares,
                MoveType::Capture => targets & opponent_occupancy,
            };

            self.push_moves(from, moves, list);
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
    fn generate_pawn_moves<SideT: SideToMove, StateT: State + GameStateExt>(
        &self,
        position: &Position<AT, StateT>,
        list: &mut MoveList,
        move_type: MoveType,
    ) {
        let en_passant = position.state().en_passant();
        let empty_squares = position.empty_squares();
        let occupancy = position.occupancy::<SideT::Other>();
        let double_step_rank = Bitboard::rank(SideT::DOUBLE_STEP_RANK);

        // generate moves for each of the pawns
        let pawn_squares = position.get_piece::<SideT>(Pieces::Pawn);
        for from in pawn_squares.iter() {
            let to = Square::from_idx((from.idx() as i8 + SideT::PAWN_PUSH_OFFSET) as usize);
            let mut moves = Bitboard::empty();

            // generate pawn pushes
            if move_type == MoveType::All || move_type == MoveType::Quiet {
                let single_step = Bitboard::square(to) & empty_squares;
                let double_step = single_step.rotate_left(SideT::PAWN_DOUBLE_STEP_OFFSET)
                    & empty_squares
                    & double_step_rank;
                moves |= single_step | double_step;
            }

            // generate pawn captures
            if move_type == MoveType::All || move_type == MoveType::Capture {
                let targets = self.attack_table.pawn_targets::<SideT>(from);
                let captures = targets & occupancy;
                let en_passant_captures = match en_passant {
                    Some(ep) => targets & Bitboard::square(ep),
                    None => Bitboard::empty(),
                };
                moves |= captures | en_passant_captures;
            }

            // push the pawn moves to the move list
            self.push_pawn_moves::<SideT>(from, moves, list, en_passant);
        }
    }

    // generate_king_moves generates all the pseudo-legal moves of the given
    // move type for the king from the current position and pushes them to the
    // move list
    //
    // @param: position - immutable reference to the position
    // @param: list - mutable reference to the move list
    // @param: move_type - move type to generate moves of
    // @return: void
    // @side-effects: modifies the `move list`
    fn generate_king_moves<SideT: SideToMove, StateT: State + GameStateExt>(
        &self,
        position: &Position<AT, StateT>,
        list: &mut MoveList,
        move_type: MoveType,
    ) {
        let from = position.king_square::<SideT>();
        let targets = self.attack_table.king_targets(from);

        // filter the moves according to the requested move type
        let empty_squares = position.empty_squares();
        let our_occupancy = position.occupancy::<SideT>();
        let opponent_occupancy = position.occupancy::<SideT::Other>();

        let moves = match move_type {
            MoveType::All => targets & !our_occupancy,
            MoveType::Quiet => targets & empty_squares,
            MoveType::Capture => targets & opponent_occupancy,
        };

        // push the king moves to the move list
        self.push_moves(from, moves, list);

        // generate castle moves if the move type is all or quiet
        if move_type == MoveType::All || move_type == MoveType::Quiet {
            self.generate_castle_moves::<SideT, StateT>(position, list);
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
    fn generate_castle_moves<SideT: SideToMove, StateT: State + GameStateExt>(
        &self,
        position: &Position<AT, StateT>,
        list: &mut MoveList,
    ) {
        // get the castling rights for the side to move
        let castling = position.state().castling();
        let (kingside, queenside) = (castling.kingside::<SideT>(), castling.queenside::<SideT>());

        // get the current king square
        let from = position.king_square::<SideT>();

        // check if the side to move can castle
        //
        // Note: a side can castle iff they have either kingside or queenside
        //       permissions and they are not currently in check
        if !(kingside || queenside) || position.is_attacked::<SideT>(from) {
            return;
        }

        // generate castle moves depending on the side to move
        let occupancy = position.total_occupancy();
        let mut moves = Bitboard::empty();

        if kingside {
            // get the blockers (squares in between the king and the rook)
            //
            // TOOD: refactor this call to use Bitboard::between
            let blockers = Bitboard::square(SideT::KINGSIDE_DESTINATION)
                | Bitboard::square(SideT::KINGSIDE_ROOK_DESTINATION);

            // if the squares along the path are empty and the king is not moving
            // "through" check, we can castle
            if (occupancy & blockers).is_empty()
                && !position.is_attacked::<SideT>(SideT::KINGSIDE_ROOK_DESTINATION)
            {
                moves |= Bitboard::square(SideT::KINGSIDE_DESTINATION);
            }
        }

        if queenside {
            // same as in the kingside implementation
            //
            // Note: the queenside blockers include an additional square, see
            //       `QUEENSIDE_ROOK_INTERMEDIATE` for more details.
            let blockers = Bitboard::square(SideT::QUEENSIDE_DESTINATION)
                | Bitboard::square(SideT::QUEENSIDE_ROOK_DESTINATION)
                | Bitboard::square(SideT::QUEENSIDE_ROOK_INTERMEDIATE);

            if (occupancy & blockers).is_empty()
                && !position.is_attacked::<SideT>(SideT::QUEENSIDE_ROOK_DESTINATION)
            {
                moves |= Bitboard::square(SideT::QUEENSIDE_DESTINATION);
            }
        }

        // push the castle moves to the move list
        self.push_castling_moves(from, moves, list);
    }
}
