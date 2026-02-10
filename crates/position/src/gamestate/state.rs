use super::{Clock, State, StateReader, StateWriter};
use chess_kit_primitives::{Bitboard, Castling, Pieces, Side, Sides, Square, ZobristKey};
use chess_kit_collections::Copyable;

// StateHeader is a header for a state that contains the parts of the state up
// to (and excluding) the state key
//
// note: this is the struct that is copied when deriving a new state entry from
//       the current state in the history
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateHeader {
    pub(super) turn: Sides,                // side to move
    pub(super) captured_piece: Pieces,     // piece that was captured to arrive at this state
    pub(super) castling: Castling,         // castling rights
    pub(super) en_passant: Option<Square>, // active en passant square, if any
    pub(super) halfmoves: Clock,           // halfmove clock
    pub(super) fullmoves: Clock,           // fullmove clock
    pub(super) key: ZobristKey,            // key for the current state
}

impl StateHeader {
    // new creates a new, empty state header
    //
    // @return: new, empty state header
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            turn: Sides::White,
            captured_piece: Pieces::None,
            castling: Castling::all(),
            en_passant: None,
            halfmoves: 0,
            fullmoves: 0,
            key: ZobristKey::default(),
        }
    }

    // reset resets the state header to a new initial state
    //
    // @return: void
    // @side-effects: modifies the `state header`
    #[inline(always)]
    pub fn reset(&mut self) {
        self.turn = Sides::White;
        self.captured_piece = Pieces::None;
        self.castling = Castling::all();
        self.en_passant = None;
        self.halfmoves = 0;
        self.fullmoves = 0;
        self.key = ZobristKey::default();
    }
}

// DefaultState is the default implementation of the State trait
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DefaultState {
    pub(super) header: StateHeader, // header of the state

    // ==============================
    //           NOT COPIED
    // ==============================
    checkers: Bitboard, // bitboard of pieces that are checking the opponent's king
    king_blockers: [Bitboard; Sides::TOTAL], // bitboard of the side's king's blockers
    pinners: [Bitboard; Sides::TOTAL], // bitboard of the pieces that are pinning the opponent's king
    check_squares: [Bitboard; Pieces::TOTAL], // bitboard of squares that each piece would deliver check
}

impl State for DefaultState {
    // new creates a new, empty state
    //
    // @impl: State::new
    #[inline(always)]
    fn new() -> Self {
        Self {
            header: StateHeader::new(),
            checkers: Bitboard::empty(),
            king_blockers: [Bitboard::empty(); Sides::TOTAL],
            pinners: [Bitboard::empty(); Sides::TOTAL],
            check_squares: [Bitboard::empty(); Pieces::TOTAL],
        }
    }

    // reset resets the state to a new initial state
    //
    // @impl: State::reset
    #[inline(always)]
    fn reset(&mut self) {
        self.header.reset();
        self.checkers = Bitboard::empty();
        self.king_blockers = [Bitboard::empty(); Sides::TOTAL];
        self.pinners = [Bitboard::empty(); Sides::TOTAL];
        self.check_squares = [Bitboard::empty(); Pieces::TOTAL];
    }
}

impl StateReader for DefaultState {
    // turn returns the side to move
    //
    // @impl: StateReader::turn
    #[inline(always)]
    fn turn(&self) -> Sides {
        self.header.turn
    }

    // castling returns the current castling rights
    //
    // @impl: StateReader::castling
    #[inline(always)]
    fn castling(&self) -> Castling {
        self.header.castling
    }

    // en_passant returns the current en passant square, if any
    //
    // @impl: StateReader::en_passant
    #[inline(always)]
    fn en_passant(&self) -> Option<Square> {
        self.header.en_passant
    }

    // captured_piece returns the piece that was captured to arrive at this state
    //
    // @impl: StateReader::captured_piece
    #[inline(always)]
    fn captured_piece(&self) -> Pieces {
        self.header.captured_piece
    }

    // halfmoves returns the value of the current halfmove clock
    //
    // @impl: StateReader::halfmoves
    #[inline(always)]
    fn halfmoves(&self) -> Clock {
        self.header.halfmoves
    }

    // fullmoves returns the value of the current fullmove clock
    //
    // @impl: StateReader::fullmoves
    #[inline(always)]
    fn fullmoves(&self) -> Clock {
        self.header.fullmoves
    }

    // key returns a key representing a unique identifier of the state
    //
    // @impl: StateReader::key
    #[inline(always)]
    fn key(&self) -> ZobristKey {
        self.header.key
    }

    // checkers returns the bitboard of pieces that are checking the opponent's king
    //
    // @impl: StateReader::checkers
    #[inline(always)]
    fn checkers(&self) -> Bitboard {
        self.checkers
    }

    // king_blocker_pieces returns the bitboard of the side's king's blocker
    // pieces
    //
    // @impl: StateReader::king_blocker_pieces
    #[inline(always)]
    fn king_blocker_pieces<SideT: Side>(&self) -> Bitboard {
        self.king_blockers[SideT::INDEX]
    }

    // pinning_pieces returns the bitboard of the pieces that are pinning the
    // opponent's king
    //
    // @impl: StateReader::pinning_pieces
    #[inline(always)]
    fn pinning_pieces<SideT: Side>(&self) -> Bitboard {
        self.pinners[SideT::INDEX]
    }

    // check_squares returns the bitboard of squares that a given piece would
    // have to be on to deliver check to SideT::Other's king
    //
    // @impl: StateReader::check_squares
    #[inline(always)]
    fn check_squares<SideT: Side>(&self, piece: Pieces) -> Bitboard {
        self.check_squares[piece]
    }
}

impl StateWriter for DefaultState {
    // set_turn sets the side to move
    //
    // @impl: StateWriter::set_turn
    #[inline(always)]
    fn set_turn(&mut self, turn: Sides) {
        self.header.turn = turn;
    }

    // set_castling sets the castling rights
    //
    // @impl: StateWriter::set_castling
    #[inline(always)]
    fn set_castling(&mut self, castling: Castling) {
        self.header.castling = castling;
    }

    // set_en_passant sets the en passant square, if any
    //
    // @impl: StateWriter::set_en_passant
    #[inline(always)]
    fn set_en_passant(&mut self, en_passant: Option<Square>) {
        self.header.en_passant = en_passant;
    }

    // set_captured_piece sets the piece that was captured to arrive at this state
    //
    // @impl: StateWriter::set_captured_piece
    #[inline(always)]
    fn set_captured_piece(&mut self, piece: Pieces) {
        self.header.captured_piece = piece;
    }

    // set_halfmoves sets the value of the current halfmove clock
    //
    // @impl: StateWriter::set_halfmoves
    #[inline(always)]
    fn set_halfmoves(&mut self, halfmoves: Clock) {
        self.header.halfmoves = halfmoves;
    }

    // inc_halfmoves increments the value of the current halfmove clock by one
    //
    // @impl: StateWriter::inc_halfmoves
    #[inline(always)]
    fn inc_halfmoves(&mut self) {
        self.header.halfmoves += 1;
    }

    // dec_halfmoves decrements the value of the current halfmove clock by one
    //
    // @impl: StateWriter::dec_halfmoves
    #[inline(always)]
    fn dec_halfmoves(&mut self) {
        self.header.halfmoves -= 1;
    }

    // set_fullmoves sets the value of the current fullmove clock
    //
    // @impl: StateWriter::set_fullmoves
    #[inline(always)]
    fn set_fullmoves(&mut self, fullmoves: Clock) {
        self.header.fullmoves = fullmoves;
    }

    // inc_fullmoves increments the value of the current fullmove clock by one
    //
    // @impl: StateWriter::inc_fullmoves
    #[inline(always)]
    fn inc_fullmoves(&mut self) {
        self.header.fullmoves += 1;
    }

    // dec_fullmoves decrements the value of the current fullmove clock by one
    //
    // @impl: StateWriter::dec_fullmoves
    #[inline(always)]
    fn dec_fullmoves(&mut self) {
        self.header.fullmoves -= 1;
    }

    // set_key sets the key for the current state
    //
    // @impl: StateWriter::set_key
    #[inline(always)]
    fn set_key(&mut self, key: ZobristKey) {
        self.header.key = key;
    }

    // update_key updates the key for the current state
    //
    // note: XOR's the current key with the given key, not a `set`
    //
    // @impl: StateWriter::update_key
    #[inline(always)]
    fn update_key(&mut self, key: ZobristKey) {
        self.header.key ^= key;
    }

    // set_checkers sets the bitboard of pieces that are checking the opponent's king
    //
    // @impl: StateWriter::set_checkers
    #[inline(always)]
    fn set_checkers(&mut self, checkers: Bitboard) {
        self.checkers = checkers;
    }

    // set_king_blocker_pieces sets the bitboard of the side's king's blocker
    // pieces
    //
    // @impl: StateWriter::set_king_blocker_pieces
    #[inline(always)]
    fn set_king_blocker_pieces<SideT: Side>(&mut self, pieces: Bitboard) {
        self.king_blockers[SideT::INDEX] = pieces;
    }

    // set_pinning_pieces sets the bitboard of the pieces that are pinning the
    // opponent's king
    //
    // @impl: StateWriter::set_pinning_pieces
    #[inline(always)]
    fn set_pinning_pieces<SideT: Side>(&mut self, pieces: Bitboard) {
        self.pinners[SideT::INDEX] = pieces;
    }

    // set_check_squares sets the bitboard of squares that a given piece would
    // have to be on to deliver check to SideT::Other's king
    //
    // @impl: StateWriter::set_check_squares
    #[inline(always)]
    fn set_check_squares<SideT: Side>(&mut self, piece: Pieces, squares: Bitboard) {
        self.check_squares[piece] = squares;
    }
}

impl Copyable for DefaultState {
    // copy_from copies the header of another state into this state
    //
    // @impl: Copyable::copy_from
    #[inline(always)]
    fn copy_from(&mut self, other: &Self) {
        self.header = other.header;
    }
}

impl Default for DefaultState {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
