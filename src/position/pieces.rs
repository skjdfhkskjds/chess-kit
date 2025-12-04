use crate::attack_table::AttackTable;
use crate::position::Position;
use crate::primitives::{Bitboard, GameStateExt, Pieces, Side, Square, State};

impl<AT, StateT> Position<AT, StateT>
where
    AT: AttackTable,
    StateT: State + GameStateExt,
{
    // king_square gets the square of the king for the given side
    //
    // @return: square of the king for the given side
    #[inline(always)]
    pub fn king_square<SideT: Side>(&self) -> Square {
        // TODO: refactor into bitboard.first() or something
        Square::from_idx(self.get_piece::<SideT>(Pieces::King).trailing_zeros() as usize)
    }

    // get_piece returns the bitboard of the given side and piece
    //
    // @param: piece - piece to get the bitboard for
    // @return: bitboard of the piece for the given side
    #[inline(always)]
    pub fn get_piece<SideT: Side>(&self, piece: Pieces) -> Bitboard {
        self.bitboards[SideT::INDEX][piece.idx()]
    }

    // remove_piece_no_incrementals removes the piece from the given side and
    // square without updating the zobrist key or any incremental game state
    //
    // @param: piece - piece to remove
    // @param: square - square to remove the piece from
    // @return: void
    // @side-effects: modifies the `position`
    #[inline(always)]
    pub(crate) fn remove_piece_no_incrementals<SideT: Side>(&mut self, piece: Pieces, square: Square) {
        self.bitboards[SideT::INDEX][piece.idx()].remove_at(square);
        self.sides[SideT::INDEX].remove_at(square);
        self.pieces[square.idx()] = Pieces::None;
    }

    // remove_piece removes the piece from the given side and square
    //
    // @param: piece - piece to remove
    // @param: square - square to remove the piece from
    // @return: void
    // @side-effects: modifies the `position`
    #[inline(always)]
    pub(crate) fn remove_piece<SideT: Side>(&mut self, piece: Pieces, square: Square) {
        self.remove_piece_no_incrementals::<SideT>(piece, square);
        self.state.update_key(self.zobrist.piece::<SideT>(piece, square));

        // TODO: make updates to game state for things like phase, turn, piece square valuation
    }

    // set_piece_no_incrementals sets the piece on the given side and square
    // without updating the zobrist key or any incremental game state
    //
    // @param: piece - piece to set on the board
    // @param: square - square to set the piece on
    // @return: void
    // @side-effects: modifies the `position`
    #[inline(always)]
    pub(crate) fn set_piece_no_incrementals<SideT: Side>(&mut self, piece: Pieces, square: Square) {
        self.bitboards[SideT::INDEX][piece.idx()].set_at(square);
        self.sides[SideT::INDEX].set_at(square);
        self.pieces[square.idx()] = piece;
    }

    // set_piece puts the piece on the given side and square
    //
    // @param: piece - piece to put on the board
    // @param: square - square to put the piece on
    // @return: void
    // @side-effects: modifies the `position`
    #[inline(always)]
    pub(crate) fn set_piece<SideT: Side>(&mut self, piece: Pieces, square: Square) {
        self.set_piece_no_incrementals::<SideT>(piece, square);
        self.state.update_key(self.zobrist.piece::<SideT>(piece, square));

        // TODO: make updates to game state for things like phase, turn, piece square valuation
    }

    // set_en_passant sets the en passant square for the given side
    //
    // @param: square - square to set the en passant square for
    // @return: void
    // @side-effects: modifies the `position`
    #[inline(always)]
    pub(crate) fn set_en_passant(&mut self, square: Square) {
        self.state.update_key(self.zobrist.en_passant(self.state.en_passant()));
        self.state.set_en_passant(Some(square));
        self.state.update_key(self.zobrist.en_passant(Some(square)));
    }

    // clear_en_passant clears the en passant square for the given side
    //
    // @return: void
    // @side-effects: modifies the `position`
    #[inline(always)]
    pub(crate) fn clear_en_passant(&mut self) {
        self.state.update_key(self.zobrist.en_passant(self.state.en_passant()));
        self.state.set_en_passant(None);
        self.state.update_key(self.zobrist.en_passant(None));
    }

    // has_bishop_pair checks if the given side has a bishop pair
    //
    // @return: true if the given side has a bishop pair, false otherwise
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
