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

    // king_square gets the square of the king for the given side
    //
    // @param: self - immutable reference to the board
    // @param: side - side to get the king square for
    // @return: square of the king for the given side
    pub fn king_square(&self, side: Side) -> Square {
        Square::new(self.get_piece(side, Pieces::KING).bits().trailing_zeros() as usize)
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
