mod display;
mod rank;

use chess_kit_derive::IndexableEnum;
use crate::primitives::{White, Black};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, IndexableEnum)]
#[repr(u8)]
pub enum Rank {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

impl Rank {
    pub const TOTAL: usize = 8;
}

// 'SideRanks' is a trait that provides specific rank information with respect
// to a given side
// 
// @trait
pub trait SideRanks {
    // SINGLE_STEP_RANK is the rank that a pawn can single step to
    const SINGLE_STEP_RANK: Rank;

    // DOUBLE_STEP_RANK is the rank that a pawn can double step to
    const DOUBLE_STEP_RANK: Rank;

    // PROMOTABLE_RANK is the rank that a pawn is on when it promotes on its
    // next move
    const PROMOTABLE_RANK: Rank;

    // PROMOTION_RANK is the rank that a pawn promotes at
    const PROMOTION_RANK: Rank;
}

impl SideRanks for White {
    const SINGLE_STEP_RANK: Rank = Rank::R3;
    const DOUBLE_STEP_RANK: Rank = Rank::R4;
    const PROMOTABLE_RANK: Rank = Rank::R7;
    const PROMOTION_RANK: Rank = Rank::R8;
}

impl SideRanks for Black {
    const SINGLE_STEP_RANK: Rank = Rank::R6;
    const DOUBLE_STEP_RANK: Rank = Rank::R5;
    const PROMOTABLE_RANK: Rank = Rank::R2;
    const PROMOTION_RANK: Rank = Rank::R1;
}
