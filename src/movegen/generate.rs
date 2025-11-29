use crate::board::Board;
use crate::movegen::MoveGenerator;
use crate::primitives::{
    BITBOARD_RANKS, BITBOARD_SQUARES, Bitboard, Move, MoveList, MoveType, Piece, Rank, Side, Square,
};

// list of pieces that a pawn can promote to
const PROMOTION_PIECES: [Piece; 4] = [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight];

impl MoveGenerator {
    // generate_moves generates all the pseudo-legal moves of the given move
    // type from the current board and pushes them to the move list
    //
    // @param: self - immutable reference to the move generator
    // @param: board - immutable reference to the board
    // @param: ml - mutable reference to the move list
    // @param: mt - move type to generate moves for
    // @return: void
    // @side-effects: modifies the `move list`
    pub fn generate_moves(&self, board: &Board, moves: &mut MoveList, move_type: MoveType) {
        self.generate_moves_for_piece(board, Piece::King, moves, move_type);
        self.generate_moves_for_piece(board, Piece::Knight, moves, move_type);
        self.generate_moves_for_piece(board, Piece::Rook, moves, move_type);
        self.generate_moves_for_piece(board, Piece::Bishop, moves, move_type);
        self.generate_moves_for_piece(board, Piece::Queen, moves, move_type);
        self.generate_moves_for_piece(board, Piece::Pawn, moves, move_type);

        if move_type == MoveType::All || move_type == MoveType::Quiet {
            self.generate_castle_moves(board, moves);
        }
    }
}

impl MoveGenerator {
    // generate_moves_for_piece generates all the pseudo-legal moves of the given
    // move type for the given piece from the current board and pushes them to
    // the move list
    //
    // @param: self - immutable reference to the move generator
    // @param: board - immutable reference to the board
    // @param: piece - piece to generate moves of
    // @param: list - mutable reference to the move list
    // @param: move_type - move type to generate moves of
    // @return: void
    // @side-effects: modifies the `move list`
    pub fn generate_moves_for_piece(
        &self,
        board: &Board,
        piece: Piece,
        list: &mut MoveList,
        move_type: MoveType,
    ) {
        // if the piece is a pawn,
        if matches!(piece, Piece::Pawn) {
            return self.generate_pawn_moves(board, list, move_type);
        }

        let occupancy = board.occupancy();
        let empty_squares = board.empty_squares();
        let our_occupancy = board.sides[board.turn().idx()];
        let opponent_occupancy = board.sides[board.opponent().idx()];

        // generate moves from all positions of the piece for the current side
        // to move
        let to_move = board.get_piece(board.turn(), piece);
        for from in to_move.iter() {
            let targets = match piece {
                Piece::King => self.get_king_targets(from),
                Piece::Knight => self.get_knight_targets(from),
                Piece::Bishop => self.get_bishop_attacks(from, &occupancy),
                Piece::Rook => self.get_rook_attacks(from, &occupancy),
                Piece::Queen => self.get_queen_attacks(from, &occupancy),
                _ => unreachable!("Not a valid piece: {piece}"),
            };

            // filter the moves according to the requested move type
            let moves = match move_type {
                MoveType::All => targets & !our_occupancy,
                MoveType::Quiet => targets & empty_squares,
                MoveType::Capture => targets & opponent_occupancy,
            };

            self.push_moves(board, piece, from, moves, list);
        }
    }

    // generate_pawn_moves generates all the pseudo-legal moves of the given
    // move type for the pawns from the current board and pushes them to the
    // move list
    //
    // @param: self - immutable reference to the move generator
    // @param: board - immutable reference to the board
    // @param: list - mutable reference to the move list
    // @param: move_type - move type to generate moves of
    // @return: void
    // @side-effects: modifies the `move list`
    fn generate_pawn_moves(&self, board: &Board, list: &mut MoveList, move_type: MoveType) {
        let empty_squares = board.empty_squares();
        let double_step_rank = BITBOARD_RANKS[Rank::double_step_rank(board.turn()).idx()];
        // TODO: figure out how to abstract this away appropriately
        let direction = match board.turn() {
            Side::White => 8,
            Side::Black => -8,
        };
        let rotation_count = (Square::TOTAL as i8 + direction) as u32;

        // generate moves for each of the pawns
        let pawn_squares = board.get_piece(board.turn(), Piece::Pawn);
        for from in pawn_squares.iter() {
            let to = (from.idx() as i8 + direction) as usize;
            let mut moves = Bitboard::empty();

            // generate pawn pushes
            if move_type == MoveType::All || move_type == MoveType::Quiet {
                let single_step = BITBOARD_SQUARES[to] & empty_squares;
                let double_step =
                    single_step.rotate_left(rotation_count) & empty_squares & double_step_rank;
                moves |= single_step | double_step;
            }

            // generate pawn captures
            if move_type == MoveType::All || move_type == MoveType::Capture {
                let targets = self.get_pawn_targets(from, board.turn());
                let captures = targets & board.sides[board.opponent().idx()];
                let en_passant_captures = match board.state.en_passant {
                    Some(ep) => targets & BITBOARD_SQUARES[ep.idx()],
                    None => Bitboard::empty(),
                };
                moves |= captures | en_passant_captures;
            }

            self.push_moves(board, Piece::Pawn, from, moves, list);
        }
    }

    // generate_castle_moves generates all the pseudo-legal moves of the given
    // move type for the castling from the current board and pushes them to the
    // move list
    //
    // @param: self - immutable reference to the move generator
    // @param: board - immutable reference to the board
    // @param: list - mutable reference to the move list
    // @param: move_type - move type to generate moves of
    // @return: void
    // TODO: current implementation does not support chess960, as it assumes the
    //       squares along the path from the king and rook
    fn generate_castle_moves(&self, board: &Board, list: &mut MoveList) {
        let us = board.turn();
        
        // get the castling rights for the side to move
        let (kingside, queenside) = match us {
            Side::White => (
                board.state.castling.kingside(Side::White),
                board.state.castling.queenside(Side::White),
            ),
            Side::Black => (
                board.state.castling.kingside(Side::Black),
                board.state.castling.queenside(Side::Black),
            ),
        };
        
        // get the current king square
        let from = board.king_square(us);

        // check if the side to move can castle
        // 
        // Note: a side can castle iff they have either kingside or queenside
        //       permissions and they are not currently in check
        if !(kingside || queenside) || self.is_attacked(board, us, from) {
            return;
        }
        
        // generate castle moves depending on the side to move
        let occupancy = board.occupancy();
        let mut moves = Bitboard::empty();

        if kingside {
            // get path information for the kingside castle
            // 
            // to: the destination square of the king
            // intermediate: the intermediate square between the king and to
            // blockers: the squares in between the king and the rook
            let (to, intermediate, blockers) = match us {
                Side::White => (
                    Square::G1,
                    Square::F1,
                    BITBOARD_SQUARES[Square::F1.idx()] | BITBOARD_SQUARES[Square::G1.idx()],
                ),
                Side::Black => (
                    Square::G8,
                    Square::F8,
                    BITBOARD_SQUARES[Square::F8.idx()] | BITBOARD_SQUARES[Square::G8.idx()],
                ),
            };

            // if the squares along the path are empty and the king is not moving
            // "through" check, we can castle
            if (occupancy & blockers).is_empty() && !self.is_attacked(board, us, intermediate) {
                moves |= BITBOARD_SQUARES[to.idx()];
            }
        }

        if queenside {
            // identical to the kingside implementation
            let (to, intermediate, blockers) = match us {
                Side::White => (
                    Square::C1,
                    Square::D1,
                    BITBOARD_SQUARES[Square::B1.idx()]
                        | BITBOARD_SQUARES[Square::C1.idx()]
                        | BITBOARD_SQUARES[Square::D1.idx()],
                ),
                Side::Black => (
                    Square::C8,
                    Square::D8,
                    BITBOARD_SQUARES[Square::B8.idx()]
                        | BITBOARD_SQUARES[Square::C8.idx()]
                        | BITBOARD_SQUARES[Square::D8.idx()],
                ),
            };

            if (occupancy & blockers).is_empty() && !self.is_attacked(board, us, intermediate) {
                moves |= BITBOARD_SQUARES[to.idx()];
            }
        }

        // push the castle moves to the move list
        self.push_moves(board, Piece::King, from, moves, list);
    }

    // push_moves pushes a set of moves to the move list as defined by the
    // given piece at the from square to the each of the to squares.
    //
    // @param: self - immutable reference to the move generator
    // @param: board - immutable reference to the board
    // @param: piece - piece to push the moves for
    // @param: from - square to push the moves from
    // @param: to_squares - bitboard of squares to push the moves to
    // @param: list - mutable reference to the move list
    // @return: void
    // @side-effects: modifies the `move list`
    fn push_moves(
        &self,
        board: &Board,
        piece: Piece,
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
            let captured = board.pieces[to.idx()];
            if !matches!(captured, Piece::None) {
                mv = mv.with_capture(captured);
            }

            // handle the special cases for the piece
            match piece {
                Piece::Pawn => {
                    // a pawn is moving, so we need to handle the cases
                    //
                    // 1. en passant capture
                    // 2. double step pawn push
                    // 3. promotion

                    // check if the move is an en passant capture
                    let is_en_passant = match board.state.en_passant {
                        Some(square) => square == to,
                        None => false,
                    };

                    if is_en_passant {
                        // the move is an en passant capture
                        mv = mv.with_en_passant();
                    } else if to.distance(from) == 16 {
                        // the move is a double step pawn push
                        mv = mv.with_double_step();
                    } else if to.on_rank(Rank::promotion_rank(board.turn())) {
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
                Piece::King => {
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

impl MoveGenerator {
    // is_attacked returns true if the given square on the given side is attacked
    // by the opponent.
    //
    // @param: self - immutable reference to the move generator
    // @param: board - immutable reference to the board
    // @param: side - side to check if is attacked
    // @param: square - square to check if is attacked
    // @return: true if the square is attacked, false otherwise
    pub fn is_attacked(&self, board: &Board, side: Side, square: Square) -> bool {
        // idea: our square `T` is attacked iff the opponent has at least one
        //       piece in square `S` such that attack board generated from `T`
        //       includes `S`
        //
        // effectively relies on this idea of, if i can see you, you can see me
        //
        // the nuance not covered above is pawn attacks are not symmetric, so we
        // reconcile this by checking the pawn attacks for our side instead of the
        // opponent's

        // generate the attack boards for each piece
        let occupancy = board.occupancy();
        let king_attacks = self.get_king_targets(square);
        let rook_attacks = self.get_rook_attacks(square, &occupancy);
        let bishop_attacks = self.get_bishop_attacks(square, &occupancy);
        let knight_attacks = self.get_knight_targets(square);
        let pawn_attacks = self.get_pawn_targets(square, side);
        let queen_attacks = rook_attacks | bishop_attacks;

        // check if there is an intersection between the attack board and that
        // piece's respective occupancy
        let opponent = board.bitboards[side.other().idx()];
        !(king_attacks & opponent[Piece::King.idx()]).is_empty()
            || !(rook_attacks & opponent[Piece::Rook.idx()]).is_empty()
            || !(queen_attacks & opponent[Piece::Queen.idx()]).is_empty()
            || !(bishop_attacks & opponent[Piece::Bishop.idx()]).is_empty()
            || !(knight_attacks & opponent[Piece::Knight.idx()]).is_empty()
            || !(pawn_attacks & opponent[Piece::Pawn.idx()]).is_empty()
    }

    // is_checked returns true if the given side is checked
    //
    // @param: self - immutable reference to the move generator
    // @param: board - immutable reference to the board
    // @param: side - side to check if is checked
    // @return: true if the side is checked, false otherwise
    #[inline(always)]
    pub fn is_checked(&self, board: &Board, side: Side) -> bool {
        self.is_attacked(board, side, board.king_square(side))
    }
}
