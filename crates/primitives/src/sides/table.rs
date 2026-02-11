use super::Sides;
use std::ops::Index;

/// `SideTable` is a thin wrapper around a `[T; Sides::TOTAL]` array that
/// provides a type-safe way to store per-side data indexed by `Sides`
///
/// @type
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SideTable<T>(pub [T; Sides::TOTAL]);

impl<T> SideTable<T> {
    /// new creates a new `SideTable` with the given values for white and black
    ///
    /// @param: white - value for the white side
    /// @param: black - value for the black side
    /// @return: new `SideTable`
    pub const fn new(white: T, black: T) -> Self {
        Self([white, black])
    }
}

impl<T> Index<Sides> for SideTable<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, side: Sides) -> &Self::Output {
        &self.0[side as usize]
    }
}
