use crate::primitives::{Side};

pub type Rank = usize;

pub struct Ranks;

impl Ranks {
    pub const TOTAL: usize = 8;

    pub const R1: Rank = 0;
    pub const R2: Rank = 1;
    pub const R3: Rank = 2;
    pub const R4: Rank = 3;
    pub const R5: Rank = 4;
    pub const R6: Rank = 5;
    pub const R7: Rank = 6;
    pub const R8: Rank = 7;
}

impl Ranks {
    pub const fn double_step_rank(side: Side) -> Rank {
        match side {
            Side::White => Ranks::R4,
            Side::Black => Ranks::R5,
        }
    }

    pub const fn promotion_rank(side: Side) -> Rank {
        match side {
            Side::White => Ranks::R8,
            Side::Black => Ranks::R1,
        }
    }
}
