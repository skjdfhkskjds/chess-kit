/// bit-shift offsets to parse the draw state according to the schema below
const MATERIAL_DRAW_SHIFT: u32 = 16;

/// data-type masks to extract the data values from the draw state
const REPETITION_MASK: u32 = 0xFFFF;
const MATERIAL_DRAW_MASK: u32 = 0x1;

/// DrawState is a compact, typed representation of the incrementally maintained
/// draw information for a position
///
/// The data is stored in a u32 with the following schema:
///
/// |       | repetition | material_draw | reserved |
/// | ----- | ---------- | ------------- | -------- |
/// | bits  |       0-15 |            16 |    17-31 |
/// | mask  |     0xffff |           0x1 |          |
/// | shift |          0 |            16 |          |
///
/// note: repetition is stored as a signed, two's-complement ply distance. a
///       positive distance represents a second occurrence of a position, while
///       a negative distance represents a third or later occurrence
///
/// note: a second occurrence is considered a search repetition only when the
///       earlier position is strictly after the root. a negative distance is a
///       repetition regardless of the root since it represents threefold
///
/// @type
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct DrawState {
    data: u32,
}

impl DrawState {
    /// new creates a new, empty draw state
    ///
    /// @return: new, empty draw state
    #[inline]
    pub const fn new() -> Self {
        Self { data: 0 }
    }

    /// repetition returns the signed ply distance to the previous occurrence
    /// of this position
    ///
    /// @return: positive distance for a second occurrence, negative distance
    ///          for a third or later occurrence, or 0 if there is no repetition
    #[inline]
    pub const fn repetition(&self) -> i16 {
        (self.data & REPETITION_MASK) as u16 as i16
    }

    /// is_repetition checks if the draw state represents a repetition relative
    /// to the given search ply
    ///
    /// @param: ply - ply of the current position relative to the search root
    /// @return: true if the position is a search repetition, false otherwise
    #[inline]
    pub const fn is_repetition(&self, ply: usize) -> bool {
        let repetition = self.repetition();
        repetition < 0 || (repetition > 0 && (repetition as usize) < ply)
    }

    /// is_threefold_repetition checks if the position has occurred at least
    /// three times
    ///
    /// @return: true if the position is a threefold repetition, false otherwise
    #[inline]
    pub const fn is_threefold_repetition(&self) -> bool {
        self.repetition() < 0
    }

    /// is_material_draw checks if neither side has sufficient material to force
    /// checkmate
    ///
    /// @return: true if the position is a material draw, false otherwise
    #[inline]
    pub const fn is_material_draw(&self) -> bool {
        ((self.data >> MATERIAL_DRAW_SHIFT) & MATERIAL_DRAW_MASK) != 0
    }

    /// with_repetition sets the signed ply distance to the previous occurrence
    /// of this position
    ///
    /// @param: repetition - signed ply distance to the previous occurrence
    /// @return: draw state with the repetition distance set
    #[inline]
    pub const fn with_repetition(mut self, repetition: i16) -> Self {
        self.data = (self.data & !REPETITION_MASK) | ((repetition as u16 as u32) & REPETITION_MASK);
        self
    }

    /// with_material_draw sets whether neither side has sufficient material to
    /// force checkmate
    ///
    /// @param: is_material_draw - whether the position is a material draw
    /// @return: draw state with the material draw flag set
    #[inline]
    pub const fn with_material_draw(mut self, is_material_draw: bool) -> Self {
        let mask = MATERIAL_DRAW_MASK << MATERIAL_DRAW_SHIFT;
        self.data = (self.data & !mask) | ((is_material_draw as u32) << MATERIAL_DRAW_SHIFT);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_an_empty_draw_state() {
        let state = DrawState::new();

        assert_eq!(state.repetition(), 0);
        assert!(!state.is_repetition(1));
        assert!(!state.is_threefold_repetition());
        assert!(!state.is_material_draw());
    }

    #[test]
    fn repetition_preserves_material_draw_flag() {
        let state = DrawState::new()
            .with_material_draw(true)
            .with_repetition(-12);

        assert_eq!(state.repetition(), -12);
        assert!(state.is_threefold_repetition());
        assert!(state.is_material_draw());
    }

    #[test]
    fn second_occurrence_must_be_strictly_after_root() {
        let state = DrawState::new().with_repetition(4);

        assert!(!state.is_repetition(4));
        assert!(state.is_repetition(5));
    }

    #[test]
    fn threefold_is_a_repetition_at_any_search_ply() {
        let state = DrawState::new().with_repetition(-4);

        assert!(state.is_repetition(0));
        assert!(state.is_repetition(4));
    }
}
