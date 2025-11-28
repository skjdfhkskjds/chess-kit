use crate::primitives::Piece;

pub struct Pieces;

impl Pieces {
    pub const TOTAL: usize = 6;

    pub const PAWN: Piece = Piece::new(0);
    pub const KNIGHT: Piece = Piece::new(1);
    pub const BISHOP: Piece = Piece::new(2);
    pub const ROOK: Piece = Piece::new(3);
    pub const QUEEN: Piece = Piece::new(4);
    pub const KING: Piece = Piece::new(5);
    pub const NONE: Piece = Piece::new(6);
}

impl Piece {
    // is_none checks if the piece is NONE
    //
    // @param: self - immutable reference to the piece
    // @return: true if the piece is NONE, false otherwise
    #[inline(always)]
    pub const fn is_none(&self) -> bool {
        self.0 == Pieces::NONE.unwrap()
    }

    // is_pawn checks if the piece is a pawn
    //
    // @param: self - immutable reference to the piece
    // @return: true if the piece is a pawn, false otherwise
    #[inline(always)]
    pub const fn is_pawn(&self) -> bool {
        self.0 == Pieces::PAWN.unwrap()
    }

    // is_knight checks if the piece is a knight
    //
    // @param: self - immutable reference to the piece
    // @return: true if the piece is a knight, false otherwise
    #[inline(always)]
    pub const fn is_knight(&self) -> bool {
        self.0 == Pieces::KNIGHT.unwrap()
    }
    
    // is_bishop checks if the piece is a bishop
    //
    // @param: self - immutable reference to the piece
    // @return: true if the piece is a bishop, false otherwise
    #[inline(always)]
    pub const fn is_bishop(&self) -> bool {
        self.0 == Pieces::BISHOP.unwrap()
    }

    // is_rook checks if the piece is a rook
    //
    // @param: self - immutable reference to the piece
    // @return: true if the piece is a rook, false otherwise
    #[inline(always)]
    pub const fn is_rook(&self) -> bool {
        self.0 == Pieces::ROOK.unwrap()
    }

    // is_queen checks if the piece is a queen
    //
    // @param: self - immutable reference to the piece
    // @return: true if the piece is a queen, false otherwise
    #[inline(always)]
    pub const fn is_queen(&self) -> bool {
        self.0 == Pieces::QUEEN.unwrap()
    }

    // is_king checks if the piece is a king
    //
    // @param: self - immutable reference to the piece
    // @return: true if the piece is a king, false otherwise
    #[inline(always)]
    pub const fn is_king(&self) -> bool {
        self.0 == Pieces::KING.unwrap()
    }
}
