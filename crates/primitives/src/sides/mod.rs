mod display;
mod table;

use chess_kit_derive::IndexableEnum;

pub use table::SideTable;

// `define_sides!` generates a unit struct with associated `SideTable` constants,
// providing a clean namespace for per-side data without requiring trait hierarchies.
//
// Usage:
//
// ``` rust
// define_sides! {
//     SideCastling: Castling {
//         ALL => (Castling::WHITE, Castling::BLACK),
//         KINGSIDE => (Castling::WHITE_KING, Castling::BLACK_KING),
//     }
// }
// ```
//
// Expands to:
//
// ``` rust
// pub struct SideCastling;
// impl SideCastling {
//    pub const ALL: SideTable<Castling> = SideTable::new(Castling::WHITE, Castling::BLACK);
//     pub const KINGSIDE: SideTable<Castling> = SideTable::new(Castling::WHITE_KING, Castling::BLACK_KING);
// }
// ```
#[macro_export]
macro_rules! define_sides {
    ($name:ident: $ty:ty { $($field:ident => ($white:expr, $black:expr)),* $(,)? }) => {
        pub struct $name;
        impl $name {
            $(pub const $field: $crate::SideTable<$ty> = $crate::SideTable::new($white, $black);)*
        }
    };
}

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
}

impl Side for Black {
    type Other = White;

    const SIDE: Sides = Sides::Black;
}
