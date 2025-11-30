use crate::board::Board;
use crate::primitives::{Bitboard, Piece, Side, Square};

impl Board {
    // king_square gets the square of the king for the given side
    //
    // @return: square of the king for the given side
    #[inline(always)]
    pub fn king_square<S: Side>(&self) -> Square {
        // TODO: refactor into bitboard.first() or something
        Square::from_idx(self.get_piece::<S>(Piece::King).trailing_zeros() as usize)
    }

    // get_piece returns the bitboard of the given side and piece
    //
    // @param: piece - piece to get the bitboard for
    // @return: bitboard of the piece for the given side
    #[inline(always)]
    pub(crate) fn get_piece<S: Side>(&self, piece: Piece) -> Bitboard {
        self.bitboards[S::INDEX][piece.idx()]
    }

    // remove_piece_no_incrementals removes the piece from the given side and
    // square without updating the zobrist key or any incremental game state
    //
    // @param: piece - piece to remove
    // @param: square - square to remove the piece from
    // @return: void
    // @side-effects: modifies the `board`
    #[inline(always)]
    pub(crate) fn remove_piece_no_incrementals<S: Side>(&mut self, piece: Piece, square: Square) {
        self.bitboards[S::INDEX][piece.idx()].remove_at(square);
        self.sides[S::INDEX].remove_at(square);
        self.pieces[square.idx()] = Piece::None;
    }

    // remove_piece removes the piece from the given side and square
    //
    // @param: piece - piece to remove
    // @param: square - square to remove the piece from
    // @return: void
    // @side-effects: modifies the `board`
    #[inline(always)]
    pub(crate) fn remove_piece<S: Side>(&mut self, piece: Piece, square: Square) {
        self.remove_piece_no_incrementals::<S>(piece, square);
        self.state.zobrist_key ^= self.zobrist.piece::<S>(piece, square);

        // TODO: make updates to game state for things like phase, turn, piece square valuation
    }

    // set_piece_no_incrementals sets the piece on the given side and square
    // without updating the zobrist key or any incremental game state
    //
    // @param: piece - piece to set on the board
    // @param: square - square to set the piece on
    // @return: void
    // @side-effects: modifies the `board`
    #[inline(always)]
    pub(crate) fn set_piece_no_incrementals<S: Side>(&mut self, piece: Piece, square: Square) {
        self.bitboards[S::INDEX][piece.idx()].set_at(square);
        self.sides[S::INDEX].set_at(square);
        self.pieces[square.idx()] = piece;
    }

    // set_piece puts the piece on the given side and square
    //
    // @param: piece - piece to put on the board
    // @param: square - square to put the piece on
    // @return: void
    // @side-effects: modifies the `board`
    #[inline(always)]
    pub(crate) fn set_piece<S: Side>(&mut self, piece: Piece, square: Square) {
        self.set_piece_no_incrementals::<S>(piece, square);
        self.state.zobrist_key ^= self.zobrist.piece::<S>(piece, square);

        // TODO: make updates to game state for things like phase, turn, piece square valuation
    }

    // set_en_passant sets the en passant square for the given side
    //
    // @param: square - square to set the en passant square for
    // @return: void
    // @side-effects: modifies the `board`
    #[inline(always)]
    pub(crate) fn set_en_passant(&mut self, square: Square) {
        self.state.zobrist_key ^= self.zobrist.en_passant(self.state.en_passant);
        self.state.en_passant = Some(square);
        self.state.zobrist_key ^= self.zobrist.en_passant(self.state.en_passant);
    }

    // clear_en_passant clears the en passant square for the given side
    //
    // @return: void
    // @side-effects: modifies the `board`
    #[inline(always)]
    pub(crate) fn clear_en_passant(&mut self) {
        self.state.zobrist_key ^= self.zobrist.en_passant(self.state.en_passant);
        self.state.en_passant = None;
        self.state.zobrist_key ^= self.zobrist.en_passant(self.state.en_passant);
    }

    // has_bishop_pair checks if the given side has a bishop pair
    //
    // @return: true if the given side has a bishop pair, false otherwise
    pub(crate) fn has_bishop_pair<S: Side>(&self) -> bool {
        let bitboard = self.get_piece::<S>(Piece::Bishop);
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
