use crate::board::Board;
use crate::movegen::MoveGenerator;
use crate::primitives::{
    BITBOARD_RANKS, BITBOARD_SQUARES, Bitboard, Move, MoveList, MoveType, Piece, Pieces, Ranks,
    Side, Sides, Square, Squares,
};

// This is a list of all pieces a pawn can promote to.
const PROMOTION_PIECES: [Piece; 4] = [Pieces::QUEEN, Pieces::ROOK, Pieces::BISHOP, Pieces::KNIGHT];

impl MoveGenerator {
    // Generates moves for the side that is to move. The MoveType parameter
    // determines if all moves, or only captures need to be generated.
    pub fn generate_moves(&self, board: &Board, ml: &mut MoveList, mt: MoveType) {
        self.generate_moves_for_piece(board, Pieces::KING, ml, mt);
        self.generate_moves_for_piece(board, Pieces::KNIGHT, ml, mt);
        self.generate_moves_for_piece(board, Pieces::ROOK, ml, mt);
        self.generate_moves_for_piece(board, Pieces::BISHOP, ml, mt);
        self.generate_moves_for_piece(board, Pieces::QUEEN, ml, mt);
        self.generate_pawn_moves(board, ml, mt);

        if mt == MoveType::All || mt == MoveType::Quiet {
            self.generate_castle_moves(board, ml);
        }
    }
}

// *** === Getting the actual pseudo-legal moves. === *** //

impl MoveGenerator {
    pub fn generate_moves_for_piece(
        &self,
        board: &Board,
        piece: Piece,
        list: &mut MoveList,
        mt: MoveType,
    ) {
        if piece == Pieces::PAWN {
            return self.generate_pawn_moves(board, list, mt);
        }

        let occupancy = board.occupancy();
        let empty_squares = board.empty_squares();
        let our_occupancy = board.sides[board.turn()];
        let opponent_occupancy = board.sides[board.opponent()];

        // generate moves from all positions of the piece for the current side
        // to move
        let to_move = board.get_piece(board.turn(), piece);
        for from in to_move.iter() {
            let bb_target = match piece {
                Pieces::KING => self.get_king_attacks(from),
                Pieces::KNIGHT => self.get_knight_attacks(from),
                Pieces::BISHOP => self.get_bishop_attacks(from, &occupancy),
                Pieces::ROOK => self.get_rook_attacks(from, &occupancy),
                Pieces::QUEEN => self.get_queen_attacks(from, &occupancy),
                _ => panic!("Not a piece: {piece}"),
            };

            // filter the moves according to the requested move type
            let moves = match mt {
                MoveType::All => bb_target & !our_occupancy,
                MoveType::Quiet => bb_target & empty_squares,
                MoveType::Capture => bb_target & opponent_occupancy,
            };

            self.push_moves(board, piece, from, moves, list);
        }
    }

    pub fn generate_pawn_moves(&self, board: &Board, list: &mut MoveList, mt: MoveType) {
        // Create shorthand variables.
        let empty_squares = board.empty_squares();
        let double_step_rank = BITBOARD_RANKS[Ranks::double_step_rank(board.turn())];
        // TODO: figure out how to abstract this away appropriately
        let direction = match board.turn() {
            Sides::WHITE => 8,
            Sides::BLACK => -8,
            _ => unreachable!(),
        };
        let rotation_count = (Squares::TOTAL as i8 + direction) as u32;

        // As long as there are pawns, generate moves for each of them.
        let pawn_squares = board.get_piece(board.turn(), Pieces::PAWN);
        for from in pawn_squares.iter() {
            let to = (from.unwrap() as i8 + direction) as usize;
            let mut moves = Bitboard::empty();

            // Generate pawn pushes
            if mt == MoveType::All || mt == MoveType::Quiet {
                let bb_push = BITBOARD_SQUARES[to];
                let one_step = bb_push & empty_squares;
                let double_step =
                    one_step.rotate_left(rotation_count) & empty_squares & double_step_rank;
                moves |= one_step | double_step;
            }

            // Generate pawn captures
            if mt == MoveType::All || mt == MoveType::Capture {
                let bb_targets = self.get_pawn_attacks(from, board.turn());
                let bb_captures = bb_targets & board.sides[board.opponent()];
                let bb_ep_capture = match board.state.en_passant {
                    Some(ep) => bb_targets & BITBOARD_SQUARES[ep.unwrap()],
                    None => Bitboard::empty(),
                };
                moves |= bb_captures | bb_ep_capture;
            }

            self.push_moves(board, Pieces::PAWN, from, moves, list);
        }
    }

    pub fn generate_castle_moves(&self, board: &Board, list: &mut MoveList) {
        // Create shorthand variables.
        let opponent = board.opponent();
        let occupancy = board.occupancy();

        let king_squares = board.get_piece(board.turn(), Pieces::KING);
        let from = king_squares.iter().next().unwrap();
        match board.turn() {
            Sides::WHITE => {
                if !board.state.castling.can_castle(Sides::WHITE) {
                    return;
                }

                if board.state.castling.kingside(Sides::WHITE) {
                    let bb_kingside_blockers = BITBOARD_SQUARES[Squares::F1.unwrap()]
                        | BITBOARD_SQUARES[Squares::G1.unwrap()];
                    let is_kingside_blocked = !(occupancy & bb_kingside_blockers).is_empty();

                    if !is_kingside_blocked
                        && !self.square_attacked(board, opponent, Squares::E1)
                        && !self.square_attacked(board, opponent, Squares::F1)
                    {
                        let to = BITBOARD_SQUARES[from.unwrap()] << 2u8;
                        self.push_moves(board, Pieces::KING, from, to, list);
                    }
                }

                if board.state.castling.queenside(Sides::WHITE) {
                    let bb_queenside_blockers = BITBOARD_SQUARES[Squares::B1.unwrap()]
                        | BITBOARD_SQUARES[Squares::C1.unwrap()]
                        | BITBOARD_SQUARES[Squares::D1.unwrap()];
                    let is_queenside_blocked = !(occupancy & bb_queenside_blockers).is_empty();

                    if !is_queenside_blocked
                        && !self.square_attacked(board, opponent, Squares::E1)
                        && !self.square_attacked(board, opponent, Squares::D1)
                    {
                        let to = BITBOARD_SQUARES[from.unwrap()] >> 2u8;
                        self.push_moves(board, Pieces::KING, from, to, list);
                    }
                }
            }
            Sides::BLACK => {
                if !board.state.castling.can_castle(Sides::BLACK) {
                    return;
                }

                if board.state.castling.kingside(Sides::BLACK) {
                    let bb_kingside_blockers = BITBOARD_SQUARES[Squares::F8.unwrap()]
                        | BITBOARD_SQUARES[Squares::G8.unwrap()];
                    let is_kingside_blocked = !(occupancy & bb_kingside_blockers).is_empty();

                    if !is_kingside_blocked
                        && !self.square_attacked(board, opponent, Squares::E8)
                        && !self.square_attacked(board, opponent, Squares::F8)
                    {
                        let to = BITBOARD_SQUARES[from.unwrap()] << 2u8;
                        self.push_moves(board, Pieces::KING, from, to, list);
                    }
                }

                if board.state.castling.queenside(Sides::BLACK) {
                    let bb_queenside_blockers = BITBOARD_SQUARES[Squares::B8.unwrap()]
                        | BITBOARD_SQUARES[Squares::C8.unwrap()]
                        | BITBOARD_SQUARES[Squares::D8.unwrap()];
                    let is_queenside_blocked = !(occupancy & bb_queenside_blockers).is_empty();

                    if !is_queenside_blocked
                        && !self.square_attacked(board, opponent, Squares::E8)
                        && !self.square_attacked(board, opponent, Squares::D8)
                    {
                        let to = BITBOARD_SQUARES[from.unwrap()] >> 2u8;
                        self.push_moves(board, Pieces::KING, from, to, list);
                    }
                }
            }
            _ => unreachable!(),
        }
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
    pub fn push_moves(
        &self,
        board: &Board,
        piece: Piece,
        from: Square,
        to_squares: Bitboard,
        list: &mut MoveList,
    ) {
        let promotion_rank = Ranks::promotion_rank(board.turn());
        let is_pawn = piece.is_pawn();

        // push a move for each of the `to` squares
        for to in to_squares.iter() {
            // check if the move is an en passant capture
            let en_passant = match board.state.en_passant {
                Some(square) => is_pawn && (square == to),
                None => false,
            };

            // get the captured piece given by the existing piece at `to`
            let capture = board.pieces[to.unwrap()];
            
            let promotion = is_pawn && to.on_rank(promotion_rank);
            let double_step = is_pawn && (to.distance(from) == 16);
            let castling = piece.is_king() && (to.distance(from) == 2);

            // if the move is a promotion, push possible promotion moves
            // TODO: figure out nice abstraction for deduplicating the code
            //       right now the issue is that overwriting the promotion
            //       piece needs to first unset the old NONE flags.
            if promotion {
                PROMOTION_PIECES.iter().for_each(|promotion_piece| {
                    list.push(Move::new(
                        piece,
                        from,
                        to,
                        capture,
                        *promotion_piece,
                        en_passant,
                        double_step,
                        castling,
                    ));
                });
            } else {
                list.push(Move::new(
                    piece,
                    from,
                    to,
                    capture,
                    Pieces::NONE,
                    en_passant,
                    double_step,
                    castling,
                ));
            }
        }
    }
}

impl MoveGenerator {
    // Determine if a square is attacked by 'attacker', on the given board.
    pub fn square_attacked(&self, board: &Board, attacker: Side, square: Square) -> bool {
        let attackers = board.bitboards[attacker];

        // Use the super-piece method: get the moves for each piece,
        // starting from the given square. This provides the sqaures where
        // a piece has to be, to be able to reach the given square.
        let occupancy = board.occupancy();
        let bb_king = self.get_king_attacks(square);
        let bb_rook = self.get_rook_attacks(square, &occupancy);
        let bb_bishop = self.get_bishop_attacks(square, &occupancy);
        let bb_knight = self.get_knight_attacks(square);
        let bb_pawns = self.get_pawn_attacks(square, Sides::other(attacker));
        let bb_queen = bb_rook | bb_bishop;

        // Then determine if such a piece is actually there: see if a rook
        // is on one of the squares a rook has to be to reach the given
        // square. Same for the queen, knight, etc... As soon as one is
        // found, the square is attacked.
        !(bb_king & attackers[Pieces::KING.unwrap()]).is_empty()
            || !(bb_rook & attackers[Pieces::ROOK.unwrap()]).is_empty()
            || !(bb_queen & attackers[Pieces::QUEEN.unwrap()]).is_empty()
            || !(bb_bishop & attackers[Pieces::BISHOP.unwrap()]).is_empty()
            || !(bb_knight & attackers[Pieces::KNIGHT.unwrap()]).is_empty()
            || !(bb_pawns & attackers[Pieces::PAWN.unwrap()]).is_empty()
    }
}
