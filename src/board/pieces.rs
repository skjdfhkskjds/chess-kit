use crate::board::board::Board;
use crate::primitives::{Bitboard, Piece, Pieces, Side, Square};

impl Board {
    // get_piece returns the bitboard of the given side and piece
    //
    // @param: self - immutable reference to the board
    // @param: side - side to get the piece for
    // @param: piece - piece to get the bitboard for
    // @return: bitboard of the piece for the given side
    #[inline(always)]
    pub fn get_piece(&self, side: Side, piece: Piece) -> Bitboard {
        self.bitboards[side][piece.unwrap()]
    }

    // remove_piece_no_incrementals removes the piece from the given side and
    // square without updating the zobrist key or any incremental game state
    // 
    // @param: self - mutable reference to the board
    // @param: side - side to remove the piece from
    // @param: piece - piece to remove
    // @param: square - square to remove the piece from
    // @return: void
    // @side-effects: modifies the `board`
    pub fn remove_piece_no_incrementals(&mut self, side: Side, piece: Piece, square: Square) {
        self.bitboards[side][piece.unwrap()].remove_at(square);
        self.sides[side].remove_at(square);
        self.pieces[square.unwrap()] = Pieces::NONE;
    }

    // remove_piece removes the piece from the given side and square
    //
    // @param: self - mutable reference to the board
    // @param: side - side to remove the piece from
    // @param: piece - piece to remove
    // @param: square - square to remove the piece from
    // @return: void
    // @side-effects: modifies the `board`
    pub fn remove_piece(&mut self, side: Side, piece: Piece, square: Square) {
        self.remove_piece_no_incrementals(side, piece, square);
        self.state.zobrist_key ^= self.zobrist.piece(side, piece, square);

        // TODO: make updates to game state for things like phase, turn, piece square valuation
    }

    // set_piece_no_incrementals sets the piece on the given side and square
    // without updating the zobrist key or any incremental game state
    //
    // @param: self - mutable reference to the board
    // @param: side - side to set the piece on
    // @param: piece - piece to set on the board
    // @param: square - square to set the piece on
    // @return: void
    // @side-effects: modifies the `board`
    pub fn set_piece_no_incrementals(&mut self, side: Side, piece: Piece, square: Square) {
        self.bitboards[side][piece.unwrap()].set_at(square);
        self.sides[side].set_at(square);
        self.pieces[square.unwrap()] = piece;
    }

    // set_piece puts the piece on the given side and square
    //
    // @param: self - mutable reference to the board
    // @param: side - side to put the piece on
    // @param: piece - piece to put on the board
    // @param: square - square to put the piece on
    // @return: void
    // @side-effects: modifies the `board`
    pub fn set_piece(&mut self, side: Side, piece: Piece, square: Square) {
        self.set_piece_no_incrementals(side, piece, square);
        self.state.zobrist_key ^= self.zobrist.piece(side, piece, square);

        // TODO: make updates to game state for things like phase, turn, piece square valuation
    }
    
    // move_piece_no_incrementals moves the piece from the given square to the given square
    // without updating the zobrist key or any incremental game state
    //
    // @param: self - mutable reference to the board
    // @param: side - side to move the piece for
    // @param: piece - piece to move
    // @param: from - square to move the piece from
    // @param: to - square to move the piece to
    // @return: void
    // @side-effects: modifies the `board`
    pub fn move_piece_no_incrementals(&mut self, side: Side, piece: Piece, from: Square, to: Square) {
        self.remove_piece_no_incrementals(side, piece, from);
        self.set_piece_no_incrementals(side, piece, to);
    }

    // move_piece moves the piece from the given square to the given square
    //
    // @param: self - mutable reference to the board
    // @param: side - side to move the piece for
    // @param: piece - piece to move
    // @param: from - square to move the piece from
    // @param: to - square to move the piece to
    // @return: void
    // @side-effects: modifies the `board`
    pub fn move_piece(&mut self, side: Side, piece: Piece, from: Square, to: Square) {
        self.remove_piece(side, piece, from);
        self.set_piece(side, piece, to);
    }

    // set_en_passant sets the en passant square for the given side
    //
    // @param: self - mutable reference to the board
    // @param: square - square to set the en passant square for
    // @return: void
    // @side-effects: modifies the `board`
    pub fn set_en_passant(&mut self, square: Square) {
        self.state.zobrist_key ^= self.zobrist.en_passant(self.state.en_passant);
        self.state.en_passant = Some(square);
        self.state.zobrist_key ^= self.zobrist.en_passant(self.state.en_passant);
    }

    // clear_en_passant clears the en passant square for the given side
    //
    // @param: self - mutable reference to the board
    // @return: void
    // @side-effects: modifies the `board`
    pub fn clear_en_passant(&mut self) {
        self.state.zobrist_key ^= self.zobrist.en_passant(self.state.en_passant);
        self.state.en_passant = None;
        self.state.zobrist_key ^= self.zobrist.en_passant(self.state.en_passant);
    }

    // king_square gets the square of the king for the given side
    //
    // @param: self - immutable reference to the board
    // @param: side - side to get the king square for
    // @return: square of the king for the given side
    pub fn king_square(&self, side: Side) -> Square {
        Square::new(self.get_piece(side, Pieces::KING).trailing_zeros() as usize)
    }

    // has_bishop_pair checks if the given side has a bishop pair
    //
    // @param: self - immutable reference to the board
    // @param: side - side to check for a bishop pair
    // @return: true if the given side has a bishop pair, false otherwise
    pub fn has_bishop_pair(&self, side: Side) -> bool {
        let bitboard = self.get_piece(side, Pieces::BISHOP);
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
