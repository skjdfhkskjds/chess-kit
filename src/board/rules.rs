use crate::board::board::Board;
use crate::primitives::{Pieces, Side, Square};

impl Board {
    // is_draw checks if the position is a draw
    //
    // @param: self - immutable reference to the board
    // @return: true if the position is a draw, false otherwise
    pub fn is_draw(&self) -> bool {
        self.is_draw_by_fifty_moves() || !self.can_force_checkmate() || self.is_draw_by_repetition()
    }

    // is_draw_by_fifty_moves checks if the position is a draw according to the
    // 50-move rule
    //
    // @param: self - immutable reference to the board
    // @return: true if the position is a draw by the rule, false otherwise
    pub fn is_draw_by_fifty_moves(&self) -> bool {
        // Note: 100 since we are using the halfmove clock
        self.state.halfmoves >= 100
    }

    // is_draw_by_insufficient_material checks if the position is a draw according
    // to the draw by insufficient material rule
    //
    // @param: self - immutable reference to the board
    // @return: true if the position is a draw by the rule, false otherwise
    pub fn is_draw_by_insufficient_material(&self) -> bool {
        // Get the piece bitboards for white and black.
        let w = self.bitboards[Side::White.idx()];
        let b = self.bitboards[Side::Black.idx()];

        // check if either side has sufficient solo material to deliver
        // checkmate
        //
        // that is, if either side has a queen, rook, or a pawn.
        let sufficient_solo_material = !w[Pieces::QUEEN.unwrap()].is_empty()
            || !w[Pieces::ROOK.unwrap()].is_empty()
            || !w[Pieces::PAWN.unwrap()].is_empty()
            || !b[Pieces::QUEEN.unwrap()].is_empty()
            || !b[Pieces::ROOK.unwrap()].is_empty()
            || !b[Pieces::PAWN.unwrap()].is_empty();
        if sufficient_solo_material {
            return false;
        }

        let white_bishops = w[Pieces::BISHOP.unwrap()].count_ones();
        let black_bishops = b[Pieces::BISHOP.unwrap()].count_ones();
        let white_knights = w[Pieces::KNIGHT.unwrap()].count_ones();
        let black_knights = b[Pieces::KNIGHT.unwrap()].count_ones();
        let piece_count = white_bishops + black_bishops + white_knights + black_knights;

        // check the number of pieces on the board
        //
        // 0: only kings on the board -> draw
        // 1: only one side has a non-king piece -> draw
        // 2: draw iff both sides have one bishop AND they are on the same colour
        // else: winnable
        match piece_count {
            0 => true,
            1 => true,
            2 => {
                if white_bishops != 1 || black_bishops != 1 {
                    return false;
                }

                // check if both bishops are on the same colour
                // 
                // TODO: refactor into bitboard.first() or something
                let wb_sq = Square::from_idx(w[Pieces::BISHOP.unwrap()].trailing_zeros() as usize);
                let bb_sq = Square::from_idx(b[Pieces::BISHOP.unwrap()].trailing_zeros() as usize);
                wb_sq.is_white() == bb_sq.is_white()
            }
            _ => false,
        }
    }

    // is_draw_by_repetition checks if the position is a draw according to the
    // draw by repetition rule
    //
    // @param: self - immutable reference to the board
    // @return: true if the position is a draw by the rule, false otherwise
    pub fn is_draw_by_repetition(&self) -> bool {
        let mut count = 0;

        // walk backwards through the history
        for historic_state in self.history.iter().rev() {
            // if the zobrist keys match, we have a repetition
            if historic_state.zobrist_key == self.state.zobrist_key {
                count += 1;
            }

            // if the halfmove clock is 0, the history position is a result
            // of a capture or a pawn move, so no previous positions can be
            // the same
            if historic_state.halfmoves == 0 {
                break;
            }
        }

        // if we have found 3 or more repetitions, the position is a draw
        count >= 3
    }

    // can_force_checkmate checks if either side can force checkmate
    //
    // @param: self - immutable reference to the board
    // @return: true if either side can force checkmate, false otherwise
    pub fn can_force_checkmate(&self) -> bool {
        let w = self.bitboards[Side::White.idx()];
        let b = self.bitboards[Side::Black.idx()];

        // check if either side has sufficient solo material to deliver
        // checkmate
        //
        // that is, if either side has a queen, rook, or a pawn.
        let sufficient_solo_material = !w[Pieces::QUEEN.unwrap()].is_empty()
            || !w[Pieces::ROOK.unwrap()].is_empty()
            || !w[Pieces::PAWN.unwrap()].is_empty()
            || !b[Pieces::QUEEN.unwrap()].is_empty()
            || !b[Pieces::ROOK.unwrap()].is_empty()
            || !b[Pieces::PAWN.unwrap()].is_empty();

        // if either side has sufficient solo material or a bishop pair,
        // then that side can force checkmate
        if sufficient_solo_material
            || self.has_bishop_pair(Side::White)
            || self.has_bishop_pair(Side::Black)
        {
            return true;
        }

        let white_knights = w[Pieces::KNIGHT.unwrap()].count_ones();
        let black_knights = b[Pieces::KNIGHT.unwrap()].count_ones();

        // if either side has a knight-bishop pair, OR they have at least 3
        // knights, then that side can force checkmate
        (!w[Pieces::BISHOP.unwrap()].is_empty() && white_knights > 0)
            || (!b[Pieces::BISHOP.unwrap()].is_empty() && black_knights > 0)
            || white_knights >= 3
            || black_knights >= 3
    }
}
