pub type Side = usize;

pub struct Sides;

impl Sides {
    pub const WHITE: Side = 0;
    pub const BLACK: Side = 1;
    pub const TOTAL: Side = 2;
}

impl Sides {
    // other returns the other side
    // 
    // @param: side - side to get the other side of
    // @return: other side
    #[inline(always)]
    pub const fn other(side: Side) -> Side {
        side ^ 1
    }
}
