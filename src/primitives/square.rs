#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Default, Hash)]
pub struct Square(usize);

impl Square {
    // new creates a new square with the given usize value
    //
    // @param: square - usize value to create the square from
    // @return: new square
    #[inline(always)]
    pub fn new(square: usize) -> Self {
        Self(square)
    }

    // unwrap unwraps the square to get the underlying usize value
    //
    // @param: self - immutable reference to the square
    // @return: underlying usize value
    #[inline(always)]
    pub fn unwrap(&self) -> usize {
        self.0
    }

    // is_white returns true if the square is a white square
    //
    // @param: self - immutable reference to the square
    // @return: true if the square is a white square, false otherwise
    #[inline(always)]
    pub fn is_white(&self) -> bool {
        let even_rank = ((self.0 / 8) & 1) == 0;
        let even_square = (self.0 & 1) == 0;
        even_rank ^ even_square
    }
}

pub struct Squares;

impl Squares {
    pub const TOTAL: usize = 64;
}
