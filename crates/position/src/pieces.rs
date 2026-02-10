use super::{DefaultPosition, PositionState, State};
use chess_kit_attack_table::AttackTable;
use crate::eval::EvalState;
use chess_kit_primitives::{Pieces, Side, Sides, Square, ZobristTable};

impl<AT, StateT> DefaultPosition<AT, StateT>
where
    AT: AttackTable,
    StateT: State,
{
    // remove_piece_no_incrementals removes SideT's piece from the given square
    // without updating the zobrist key or any incremental game state
    //
    // @param: piece - piece to remove
    // @param: square - square to remove the piece from
    // @return: void
    // @side-effects: modifies the `position`
    #[inline(always)]
    pub(crate) fn remove_piece_no_incrementals<SideT: Side>(
        &mut self,
        piece: Pieces,
        square: Square,
    ) {
        self.bitboards[SideT::INDEX][piece.idx()].remove_at(square);
        self.sides[SideT::INDEX].remove_at(square);
        self.sides[Sides::TOTAL].remove_at(square);
        self.pieces[square.idx()] = Pieces::None;
    }

    // remove_piece removes SideT's piece from the given square
    //
    // @param: piece - piece to remove
    // @param: square - square to remove the piece from
    // @param: eval - mutable reference to the evaluation state to update
    // @return: void
    // @side-effects: modifies the `position`
    // @side-effects: modifies the evaluation state
    #[inline(always)]
    pub(crate) fn remove_piece<SideT: Side, EvalStateT: EvalState>(
        &mut self,
        piece: Pieces,
        square: Square,
        eval: &mut EvalStateT,
    ) {
        self.remove_piece_no_incrementals::<SideT>(piece, square);
        let delta = ZobristTable::piece::<SideT>(piece, square);
        self.state_mut().update_key(delta);

        // handle the incremental evaluation callback
        eval.on_remove_piece::<SideT>(piece, square);
    }

    // set_piece_no_incrementals sets SideT's piece on the given square without
    // updating the zobrist key or any incremental game state
    //
    // @param: piece - piece to set on the board
    // @param: square - square to set the piece on
    // @return: void
    // @side-effects: modifies the `position`
    #[inline(always)]
    pub(crate) fn set_piece_no_incrementals<SideT: Side>(&mut self, piece: Pieces, square: Square) {
        self.bitboards[SideT::INDEX][piece.idx()].set_at(square);
        self.sides[SideT::INDEX].set_at(square);
        self.sides[Sides::TOTAL].set_at(square);
        self.pieces[square.idx()] = piece;
    }

    // set_piece puts SideT's piece on the given square
    //
    // @param: piece - piece to put on the board
    // @param: square - square to put the piece on
    // @param: eval - mutable reference to the evaluation state to update
    // @return: void
    // @side-effects: modifies the `position`
    // @side-effects: modifies the evaluation state
    #[inline(always)]
    pub(crate) fn set_piece<SideT: Side, EvalStateT: EvalState>(
        &mut self,
        piece: Pieces,
        square: Square,
        eval: &mut EvalStateT,
    ) {
        self.set_piece_no_incrementals::<SideT>(piece, square);
        let delta = ZobristTable::piece::<SideT>(piece, square);
        self.state_mut().update_key(delta);

        // handle the incremental evaluation callback
        eval.on_set_piece::<SideT>(piece, square);
    }

    // set_en_passant sets the en passant square in the state
    //
    // @param: square - square to set the en passant square for
    // @return: void
    // @side-effects: modifies the `position`
    #[inline(always)]
    pub(crate) fn set_en_passant(&mut self, square: Square) {
        let current_ep = self.state().en_passant();
        let key = ZobristTable::en_passant(current_ep) ^ ZobristTable::en_passant(Some(square));
        self.state_mut().update_key(key);
        self.state_mut().set_en_passant(Some(square));
    }

    // clear_en_passant clears the en passant square in the state
    //
    // @return: void
    // @side-effects: modifies the `position`
    #[inline(always)]
    pub(crate) fn clear_en_passant(&mut self) {
        let current_ep = self.state().en_passant();
        if current_ep.is_none() {
            return;
        }

        let key = ZobristTable::en_passant(current_ep) ^ ZobristTable::en_passant(None);
        self.state_mut().set_en_passant(None);
        self.state_mut().update_key(key);
    }

    // has_bishop_pair checks if SideT has a bishop pair
    //
    // @return: true if SideT has a bishop pair, false otherwise
    pub(crate) fn has_bishop_pair<SideT: Side>(&self) -> bool {
        let bitboard = self.get_piece::<SideT>(Pieces::Bishop);
        let mut white_bishops = 0;
        let mut black_bishops = 0;

        for square in bitboard.iter() {
            if square.is_white() {
                white_bishops += 1;
            } else {
                black_bishops += 1;
            }
        }

        white_bishops >= 1 && black_bishops >= 1
    }
}
