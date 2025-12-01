use crate::primitives::Sides;

impl Sides {
    // other returns the opposing side
    //
    // @param: self - side to get the opposing side for
    // @return: opposing side
    pub const fn other(self) -> Self {
        match self {
            Sides::White => Sides::Black,
            Sides::Black => Sides::White,
        }
    }
}
