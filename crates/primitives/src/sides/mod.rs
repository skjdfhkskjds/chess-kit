mod display;
mod table;

use chess_kit_derive::IndexableEnum;

pub use table::SideTable;

/// `define_sides!` generates a unit struct with associated `SideTable` constants,
/// providing a clean namespace for per-side data without requiring trait hierarchies.
///
/// Usage:
///
/// ```
/// use chess_kit_primitives::{Castling, Sides, define_sides};
///
/// define_sides! {
///     SideCastling: Castling {
///         ALL => (Castling::WHITE, Castling::BLACK),
///         KINGSIDE => (Castling::WHITE_KING, Castling::BLACK_KING),
///     }
/// }
///
/// assert_eq!(SideCastling::ALL[Sides::White], Castling::WHITE);
/// ```
///
/// Expands to:
///
/// ```
/// use chess_kit_primitives::{Castling, SideTable};
///
/// pub struct SideCastling;
/// impl SideCastling {
///     pub const ALL: SideTable<Castling> =
///         SideTable::new(Castling::WHITE, Castling::BLACK);
///     pub const KINGSIDE: SideTable<Castling> =
///         SideTable::new(Castling::WHITE_KING, Castling::BLACK_KING);
/// }
/// ```
#[macro_export]
macro_rules! define_sides {
    ($name:ident: $ty:ty { $($field:ident => ($white:expr, $black:expr)),* $(,)? }) => {
        pub struct $name;
        impl $name {
            $(pub const $field: $crate::SideTable<$ty> = $crate::SideTable::new($white, $black);)*
        }
    };
}

/// `call_as!` dispatches a runtime [`Sides`] value to a compile-time side marker
///
/// The marker identifier supplied after the runtime side is bound to [`White`]
/// or [`Black`] in the selected branch. The body can be any expression, so
/// method arguments and compound operations do not require dedicated variadic
/// macro syntax. The runtime side expression is evaluated exactly once.
///
/// Usage:
///
/// ```
/// use chess_kit_primitives::{Side, Sides, call_as};
///
/// fn side_number<SideT: Side>(offset: usize) -> usize {
///     SideT::SIDE.idx() + offset
/// }
///
/// let side = Sides::Black;
/// let number = call_as!(side, |SideT| side_number::<SideT>(10));
/// assert_eq!(number, 11);
/// ```
///
/// Expands conceptually to a `match` whose branches define the requested
/// marker alias and evaluate the supplied body.
#[macro_export]
macro_rules! call_as {
    ($side:expr, |$side_type:ident| $body:expr $(,)?) => {{
        match $side {
            $crate::Sides::White => {
                type $side_type = $crate::White;
                $body
            }
            $crate::Sides::Black => {
                type $side_type = $crate::Black;
                $body
            }
        }
    }};
}

/// 'White' is a marker struct for compile-time generics over operations on the
/// white side
///
/// @marker-type
pub struct White;

/// 'Black' is a marker struct for compile-time generics over operations on the
/// black side
///
/// @marker-type
pub struct Black;

/// 'Side' is a trait that defines accessor constants for a given side
///
/// @trait
pub trait Side {
    /// Other is a reference to the marker type of the opposing side
    type Other: Side;

    /// SIDE is a reference to the enum value of the side
    const SIDE: Sides;
}

/// Sides is a type that enumerates both white and black sides, in addition to providing
/// a constant for the total number of sides. It also implements the `IndexableEnum` trait,
/// which allows for indexing into an array of sides using this type.
///
/// @type
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

#[cfg(test)]
mod tests {
    use super::*;

    fn side_number<SideT: Side>(offset: usize) -> usize {
        SideT::SIDE.idx() + offset
    }

    #[test]
    fn call_as_dispatches_both_markers_and_returns_the_body_value() {
        assert_eq!(
            crate::call_as!(Sides::White, |SideT| side_number::<SideT>(10)),
            10
        );
        assert_eq!(
            crate::call_as!(Sides::Black, |SideT| side_number::<SideT>(10)),
            11
        );
    }

    #[test]
    fn call_as_evaluates_the_runtime_side_once() {
        let mut evaluations = 0;
        let result = crate::call_as!(
            {
                evaluations += 1;
                Sides::White
            },
            |SideT| SideT::SIDE,
        );

        assert_eq!(result, Sides::White);
        assert_eq!(evaluations, 1);
    }
}
