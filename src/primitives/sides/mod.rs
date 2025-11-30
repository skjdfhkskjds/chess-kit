mod display;
mod sides;

use chess_kit_derive::IndexableEnum;

// 'White' is a marker struct for compile-time generics over operations on the
// white side
//
// @marker-type
pub struct White;

// 'Black' is a marker struct for compile-time generics over operations on the
// black side
//
// @marker-type
pub struct Black;

// 'Side' is a trait that defines accessor constants for a given side
//
// @trait
pub trait Side {
    // Other is a reference to the marker type of the opposing side
    type Other: Side;

    // SIDE is a reference to the enum value of the side
    const SIDE: Sides;

    // INDEX is the value of the side as an index into an array of sides
    const INDEX: usize;
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, IndexableEnum)]
#[repr(u8)]
pub enum Sides {
    White,
    Black,
}

impl Sides {
    pub const TOTAL: usize = 2;
}

impl Side for White {
    type Other = Black;

    const SIDE: Sides = Sides::White;
    const INDEX: usize = Self::SIDE as usize;
}

impl Side for Black {
    type Other = White;

    const SIDE: Sides = Sides::Black;
    const INDEX: usize = Self::SIDE as usize;
}
