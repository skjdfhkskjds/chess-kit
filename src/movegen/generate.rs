use crate::attack_table::{AttackTable, PawnDirections};
use crate::movegen::{MoveGenerator, MoveType, SideToMove};
use crate::position::Position;
use crate::primitives::{
    Bitboard, Black, GameStateExt, MoveList, Pieces, Rank, Sides, State, White,
    moves::MoveType::EnPassant,
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

    // generate_legal_moves generates all the legal moves from the current position
    // and pushes them to the move list
    //
    // @param: position - immutable reference to the position
    // @param: list - mutable reference to the move list
    // @return: void
    // @side-effects: modifies the `move list`
    pub fn generate_legal_moves<StateT: State + GameStateExt>(
        &self,
        position: &Position<AT, StateT>,
        list: &mut MoveList,
    ) {
        // if the side to move is in check, just generate evasions during legal
        // move generation
        let move_type = if position.state().checkers().not_empty() {
            MoveType::Evasions
        } else {
            MoveType::NonEvasions
        };

        match position.turn() {
            Sides::White => {
                let king_square = position.king_square::<White>();
                let pinned =
                    position.state().king_blocker_pieces::<White>() & position.occupancy::<White>();

                // generate all the pseudo-legal moves
                self.generate_moves_for_side::<White, StateT>(position, list, move_type);

                // filter the moves to only include legal moves
                list.filter(|mv| {
                    !(((pinned.has_square(mv.from()))
                        || mv.from() == king_square
                        || matches!(mv.type_of(), EnPassant))
                        && !position.is_legal_move::<White>(mv))
                })
            }
            Sides::Black => {
                let king_square = position.king_square::<Black>();
                let pinned =
                    position.state().king_blocker_pieces::<Black>() & position.occupancy::<Black>();

                // generate all the pseudo-legal moves
                self.generate_moves_for_side::<Black, StateT>(position, list, move_type);

                // filter the moves to only include legal moves
                list.filter(|mv| {
                    !(((pinned.has_square(mv.from()))
                        || mv.from() == king_square
                        || matches!(mv.type_of(), EnPassant))
                        && !position.is_legal_move::<Black>(mv))
                })
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
    #[inline(always)]
    fn generate_moves_for_side<SideT: SideToMove, StateT: State + GameStateExt>(
        &self,
        position: &Position<AT, StateT>,
        list: &mut MoveList,
        move_type: MoveType,
    ) {
        // get the set of possible destination squares for our next move based
        // on the move type and the number of pieces delivering check to the
        // king
        //
        // additionally, if the king is in double-check, the only moves that are
        // legal would be king moves that physically evade the square currently
        // being attacked, so we can skip all other pieces and only consider the
        // king
        let mut destinations = Bitboard::empty();
        if move_type != MoveType::Evasions || !position.state().checkers().more_than_one() {
            destinations = match move_type {
                MoveType::Evasions => {
                    // if the move type is evasions, then there must be exactly
                    // one piece delivering check
                    //
                    // in this case, the only moves we should consider are ones
                    // that would either block the check or capture that piece
                    debug_assert!(
                        position.state().checkers().exactly_one(),
                        "checkers should be exactly one"
                    );
                    let checker = position.state().checkers().must_first();
                    Bitboard::between(position.king_square::<SideT>(), checker)
                }
                MoveType::NonEvasions => !position.occupancy::<SideT>(),
                MoveType::Capture => position.occupancy::<SideT::Other>(),
                MoveType::Quiet => position.empty_squares(),
            };

            self.generate_pawn_moves::<SideT, StateT>(position, list, destinations, move_type);
            self.generate_queen_moves::<SideT, StateT>(position, list, destinations);
            self.generate_rook_moves::<SideT, StateT>(position, list, destinations);
            self.generate_bishop_moves::<SideT, StateT>(position, list, destinations);
            self.generate_knight_moves::<SideT, StateT>(position, list, destinations);
        }

        self.generate_king_moves::<SideT, StateT>(position, list, destinations, move_type);
    }

    // generate_queen_moves generates all the pseudo-legal moves of the given
    // move type for the queen from the current position and pushes them to the
    // move list
    //
    // @param: position - immutable reference to the position
    // @param: list - mutable reference to the move list
    // @param: move_type - move type to generate moves of
    // @return: void
    // @side-effects: modifies the `move list`
    #[inline(always)]
    fn generate_queen_moves<SideT: SideToMove, StateT: State + GameStateExt>(
        &self,
        position: &Position<AT, StateT>,
        list: &mut MoveList,
        destinations: Bitboard,
    ) {
        let occupancy = position.total_occupancy();

        // generate moves from all positions of the queen for the current side
        // to move
        let to_move = position.get_piece::<SideT>(Pieces::Queen);
        for from in to_move.iter() {
            let targets = self.attack_table.queen_targets(from, occupancy) & destinations;
            self.push_moves(from, targets, list);
        }
    }

    // generate_rook_moves generates all the pseudo-legal moves of the given
    // move type for the rook from the current position and pushes them to the
    // move list
    //
    // @param: position - immutable reference to the position
    // @param: list - mutable reference to the move list
    // @param: move_type - move type to generate moves of
    // @return: void
    #[inline(always)]
    fn generate_rook_moves<SideT: SideToMove, StateT: State + GameStateExt>(
        &self,
        position: &Position<AT, StateT>,
        list: &mut MoveList,
        destinations: Bitboard,
    ) {
        let occupancy = position.total_occupancy();

        // generate moves from all positions of the rook for the current side
        // to move
        let to_move = position.get_piece::<SideT>(Pieces::Rook);
        for from in to_move.iter() {
            let targets = self.attack_table.rook_targets(from, occupancy) & destinations;
            self.push_moves(from, targets, list);
        }
    }

    // generate_bishop_moves generates all the pseudo-legal moves of the given
    // move type for the bishop from the current position and pushes them to the
    // move list
    //
    // @param: position - immutable reference to the position
    // @param: list - mutable reference to the move list
    // @param: move_type - move type to generate moves of
    // @return: void
    #[inline(always)]
    fn generate_bishop_moves<SideT: SideToMove, StateT: State + GameStateExt>(
        &self,
        position: &Position<AT, StateT>,
        list: &mut MoveList,
        destinations: Bitboard,
    ) {
        let occupancy = position.total_occupancy();

        // generate moves from all positions of the bishop for the current side
        // to move
        let to_move = position.get_piece::<SideT>(Pieces::Bishop);
        for from in to_move.iter() {
            let targets = self.attack_table.bishop_targets(from, occupancy) & destinations;
            self.push_moves(from, targets, list);
        }
    }

    // generate_knight_moves generates all the pseudo-legal moves of the given
    // move type for the knight from the current position and pushes them to the
    // move list
    //
    // @param: position - immutable reference to the position
    // @param: list - mutable reference to the move list
    // @param: move_type - move type to generate moves of
    // @return: void
    #[inline(always)]
    fn generate_knight_moves<SideT: SideToMove, StateT: State + GameStateExt>(
        &self,
        position: &Position<AT, StateT>,
        list: &mut MoveList,
        destinations: Bitboard,
    ) {
        // generate moves from all positions of the knight for the current side
        // to move
        let to_move = position.get_piece::<SideT>(Pieces::Knight);
        for from in to_move.iter() {
            let targets = self.attack_table.knight_targets(from) & destinations;
            self.push_moves(from, targets, list);
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
    #[inline(always)]
    fn generate_pawn_moves<SideT: SideToMove, StateT: State + GameStateExt>(
        &self,
        position: &Position<AT, StateT>,
        list: &mut MoveList,
        destinations: Bitboard,
        move_type: MoveType,
    ) {
        let empty_squares = position.empty_squares();
        let single_step_rank = Bitboard::rank(SideT::SINGLE_STEP_RANK);
        let promotable_rank = Bitboard::rank(SideT::PROMOTABLE_RANK);

        // get the enemies for the pawns to target
        let enemies = if matches!(move_type, MoveType::Evasions) {
            position.state().checkers()
        } else {
            position.occupancy::<SideT::Other>()
        };

        // get the promotable and non-promotable pawns
        let promotable_pawns = position.get_piece::<SideT>(Pieces::Pawn) & promotable_rank;
        let non_promotable_pawns = position.get_piece::<SideT>(Pieces::Pawn) & !promotable_rank;

        // invariant checks for the split pawn bitboards
        debug_assert!(
            !non_promotable_pawns.intersects(Bitboard::rank(SideT::PROMOTION_RANK)),
            "non-promotable pawns cannot be on the promotion rank"
        );
        debug_assert!(
            !non_promotable_pawns.intersects(Bitboard::rank(
                if SideT::PROMOTION_RANK == Rank::R1 {
                    Rank::R8
                } else {
                    Rank::R1
                }
            )),
            "promotable pawns cannot be on the promotion rank"
        );

        // generate pawn pushes for non-promotable pawns
        if !matches!(move_type, MoveType::Capture) {
            // regular pawn pushes are a single step from the current square
            let mut single_step_pawns = self
                .attack_table
                .all_pawn_targets::<SideT>(non_promotable_pawns, PawnDirections::Up)
                & empty_squares;

            // double step pawn pushes are two steps from the current square
            let mut double_step_pawns = self.attack_table.all_pawn_targets::<SideT>(
                single_step_pawns & single_step_rank,
                PawnDirections::Up,
            ) & empty_squares;

            // if the move type is evasions, we only need to generate moves for
            // squares that would block a check
            if matches!(move_type, MoveType::Evasions) {
                single_step_pawns &= destinations;
                double_step_pawns &= destinations;
            }

            // push the pawn pushes to the move list
            self.push_pawn_moves(single_step_pawns, SideT::PAWN_PUSH_OFFSET, list);
            self.push_pawn_moves(
                double_step_pawns,
                SideT::PAWN_PUSH_OFFSET + SideT::PAWN_PUSH_OFFSET,
                list,
            );
        }

        // generate captures and quiet moves for promotable pawns
        if promotable_pawns.not_empty() {
            // get the target squares on the right of the promotable pawns
            let right_targets = self
                .attack_table
                .all_pawn_targets::<SideT>(promotable_pawns, PawnDirections::Right)
                & enemies;

            // get the target squares on the left of the promotable pawns
            let left_targets = self
                .attack_table
                .all_pawn_targets::<SideT>(promotable_pawns, PawnDirections::Left)
                & enemies;

            // get the squares that a pawn can push to and promote
            let mut pushes = self
                .attack_table
                .all_pawn_targets::<SideT>(promotable_pawns, PawnDirections::Up)
                & empty_squares;

            // again, if the move type is evasions, we only need to generate
            // moves for squares that would block a check
            if matches!(move_type, MoveType::Evasions) {
                pushes &= destinations;
            }

            // push the all variants of pawn promotions to the move list
            self.push_pawn_promotions(pushes, SideT::PAWN_PUSH_OFFSET, false, list, move_type);
            self.push_pawn_promotions(
                right_targets,
                SideT::PAWN_RIGHT_TARGET_OFFSET,
                true,
                list,
                move_type,
            );
            self.push_pawn_promotions(
                left_targets,
                SideT::PAWN_LEFT_TARGET_OFFSET,
                true,
                list,
                move_type,
            );
        }

        // generate pawn captures for non-promotable pawns
        if !matches!(move_type, MoveType::Quiet) {
            // get the target squares on the right of the non-promotable pawns
            let right_targets = self
                .attack_table
                .all_pawn_targets::<SideT>(non_promotable_pawns, PawnDirections::Right)
                & enemies;

            // get the target squares on the left of the non-promotable pawns
            let left_targets = self
                .attack_table
                .all_pawn_targets::<SideT>(non_promotable_pawns, PawnDirections::Left)
                & enemies;

            // push the pawn captures to the move list
            self.push_pawn_moves(right_targets, SideT::PAWN_RIGHT_TARGET_OFFSET, list);
            self.push_pawn_moves(left_targets, SideT::PAWN_LEFT_TARGET_OFFSET, list);

            // generate en passant captures if possible
            let en_passant = position.state().en_passant();
            if en_passant.is_none() {
                return;
            }

            // get the en passant and source square of the pawn that was just
            // pushed to enable en passant
            let ep_square = en_passant.unwrap();
            let source_square = self.attack_table.pawn_pushes::<SideT>(ep_square);

            // if the move type is evasions, and the pawn that was just pushed
            // used to be on a square that is now a line of attack, the pawn push
            // delivered a discovered check, and an en passant capture would not
            // block the check, so we can skip it
            if matches!(move_type, MoveType::Evasions) && destinations.intersects(source_square) {
                return;
            }

            // get the attacking pawns that can capture en passant
            let attacking_pawns =
                self.attack_table.pawn_targets::<SideT::Other>(ep_square) & non_promotable_pawns;

            // push the en passant captures to the move list
            self.push_pawn_en_passant_captures(attacking_pawns, ep_square, list);
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
    #[inline(always)]
    fn generate_king_moves<SideT: SideToMove, StateT: State + GameStateExt>(
        &self,
        position: &Position<AT, StateT>,
        list: &mut MoveList,
        destinations: Bitboard,
        move_type: MoveType,
    ) {
        let king_square = position.king_square::<SideT>();
        let targets = self.attack_table.king_targets(king_square);

        // filter the moves according to the requested move type
        let moves = match move_type {
            MoveType::Evasions => targets & !position.occupancy::<SideT>(),
            _ => targets & destinations,
        };

        // push the king moves to the move list
        self.push_moves(king_square, moves, list);

        // castling is only legal if the king is not currently in check and the
        // squares along the path from the king to the rook are empty, thus only
        // non-evasions and quiet moves result in the consideration of castling
        if matches!(move_type, MoveType::NonEvasions | MoveType::Quiet) {
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
    #[inline(always)]
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
        let occupancy = position.total_occupancy();

        // check if the side to move can castle
        //
        // note: a side can castle iff they have either kingside or queenside
        //       permissions and they are not currently in check
        if !(kingside || queenside) || position.is_attacked::<SideT>(from, occupancy) {
            return;
        }

        // generate castle moves depending on the side to move
        let mut moves = Bitboard::empty();

        if kingside {
            // get the blockers (squares in between the king and the rook)
            //
            // TOOD: refactor this call to use Bitboard::between
            let blockers = Bitboard::square(SideT::KINGSIDE_DESTINATION)
                | Bitboard::square(SideT::KINGSIDE_ROOK_DESTINATION);

            // if the squares along the path are empty and the king is not moving
            // "through" check, we can castle
            if !occupancy.intersects(blockers)
                && !position.is_attacked::<SideT>(SideT::KINGSIDE_ROOK_DESTINATION, occupancy)
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

            if !occupancy.intersects(blockers)
                && !position.is_attacked::<SideT>(SideT::QUEENSIDE_ROOK_DESTINATION, occupancy)
            {
                moves |= Bitboard::square(SideT::QUEENSIDE_DESTINATION);
            }
        }

        // push the castle moves to the move list
        self.push_castling_moves(from, moves, list);
    }
}
