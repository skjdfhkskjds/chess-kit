pub type Side = usize;

pub struct Sides;

impl Sides {
    pub const WHITE: Side = 0;
    pub const BLACK: Side = 1;
    pub const TOTAL: Side = 2;
}

impl Sides {
    pub const fn other(side: Side) -> Side {
        side ^ 1
    }
}