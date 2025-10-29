use super::{Bitboard, Piece, Pieces, Side, Sides, Squares};

pub struct Board {
    // sides: occupancy bitboard per side
    sides: [Bitboard; Sides::TOTAL],

    // bitboards: bitboard per piece per side
    bitboards: [[Bitboard; Pieces::TOTAL]; Sides::TOTAL],

    // pieces: piece type on each square
    pieces: [Piece; Squares::TOTAL],
}

impl Board {
    // init_sides initializes the `sides` bitboards by ORing the bitboards of
    // each side
    //
    // @param self: mutable reference to the board
    // @return: void
    // @requires: `bitboards` is initialized
    // @side-effects: modifies the `sides` bitboards
    fn init_sides(&mut self) {
        let white = self.bitboards[Sides::WHITE];
        let black = self.bitboards[Sides::BLACK];

        for (w, b) in white.iter().zip(black.iter()) {
            self.sides[Sides::WHITE] |= *w;
            self.sides[Sides::BLACK] |= *b;
        }
    }

    // init_pieces initializes the `pieces` array by iterating through the
    // bitboards of each side and setting the piece type on each square
    //
    // @param self: mutable reference to the board
    // @return: void
    // @requires: `bitboards` is initialized
    // @side-effects: modifies the `pieces` array
    fn init_pieces(&mut self) {
        let white = self.bitboards[Sides::WHITE];
        let black = self.bitboards[Sides::BLACK];

        // set the piece type on each square
        for square in 0..Squares::TOTAL {
            let mut on_square: Piece = Pieces::NONE;

            let mask = 1u64 << square; // bitmask for the square
            for (piece, (w, b)) in white.iter().zip(black.iter()).enumerate() {
                if w.bits() & mask != 0 {
                    on_square = piece;
                    break; // enforce exclusivity
                }
                if b.bits() & mask != 0 {
                    on_square = piece;
                    break; // enforce exclusivity
                }
            }

            self.pieces[square] = on_square;
        }
    }

    // get_piece returns the bitboard of the given side and piece
    //
    // @param self: immutable reference to the board
    // @param side: side to get the piece for
    // @param piece: piece to get the bitboard for
    // @return: bitboard of the piece for the given side
    pub fn get_piece(&self, side: Side, piece: Piece) -> Bitboard {
        self.bitboards[side][piece]
    }

    // empty_squares gets the bitboard of all empty squares on the board
    //
    // @param self: immutable reference to the board
    // @return: bitboard of all empty squares on the board
    pub fn empty_squares(&self) -> Bitboard {
        !(self.sides[Sides::WHITE] | self.sides[Sides::BLACK])
    }
}
