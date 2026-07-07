use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone, Default, Debug)]
pub struct PieceValue(i32, i32);

impl PieceValue {
    /// new creates a new piece value
    ///
    /// @param: middlegame - the middlegame value of the piece
    /// @param: endgame - the endgame value of the piece
    /// @return: new piece value
    #[inline]
    pub const fn new(middlegame: i32, endgame: i32) -> Self {
        Self(middlegame, endgame)
    }

    /// middlegame returns the middlegame value of the piece
    ///
    /// @return: middlegame value of the piece
    #[inline]
    pub const fn middlegame(&self) -> i32 {
        self.0
    }

    /// endgame returns the endgame value of the piece
    ///
    /// @return: endgame value of the piece
    #[inline]
    pub const fn endgame(&self) -> i32 {
        self.1
    }
}

// ================================================
//              ARITHMETIC OPERATIONS
// ================================================

impl Add for PieceValue {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for PieceValue {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl Sub for PieceValue {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl SubAssign for PieceValue {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
    }
}
