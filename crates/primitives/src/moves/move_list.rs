use crate::moves::Move;
use chess_kit_collections::{FixedArray, Retain};
use core::slice;

const MAX_MOVES: usize = 256;

/// MoveList is a fixed-size list of moves
///
/// @type
pub struct MoveList {
    moves: FixedArray<Move, MAX_MOVES>,
}

impl MoveList {
    /// new creates a new move list
    ///
    /// @return: new move list
    pub const fn new() -> Self {
        Self {
            moves: FixedArray::new(),
        }
    }

    /// clear clears the move list.
    #[inline]
    pub fn clear(&mut self) {
        self.moves.clear();
    }

    /// push pushes a move to the move list.
    #[inline]
    pub fn push(&mut self, mv: Move) {
        self.moves.push(mv);
    }

    /// len returns the number of moves in the move list.
    #[inline]
    pub fn len(&self) -> usize {
        self.moves.len()
    }

    /// capacity returns the maximum number of moves this list can hold.
    #[inline]
    pub const fn capacity(&self) -> usize {
        self.moves.capacity()
    }

    /// is_empty returns true if the move list is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.moves.is_empty()
    }

    /// is_full returns true if the move list cannot hold another move.
    #[inline]
    pub fn is_full(&self) -> bool {
        self.moves.is_full()
    }

    /// get returns the move at the given index, or None if it is out of bounds.
    #[inline]
    pub fn get(&self, idx: usize) -> Option<&Move> {
        self.moves.get(idx)
    }

    /// get_mut returns a mutable move at the given index, or None if it is out
    /// of bounds.
    #[inline]
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Move> {
        self.moves.get_mut(idx)
    }

    /// as_slice returns the generated moves in insertion order.
    #[inline]
    pub fn as_slice(&self) -> &[Move] {
        self.moves.as_slice()
    }

    /// as_mut_slice returns the generated moves in insertion order.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [Move] {
        self.moves.as_mut_slice()
    }

    /// filter removes moves that do not satisfy `predicate`.
    #[inline]
    pub fn filter(&mut self, mut predicate: impl FnMut(Move) -> bool) {
        Retain::retain(&mut self.moves, |mv| predicate(*mv));
    }
}

impl<'a> IntoIterator for &'a MoveList {
    type Item = &'a Move;
    type IntoIter = slice::Iter<'a, Move>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        (&self.moves).into_iter()
    }
}

impl<'a> IntoIterator for &'a mut MoveList {
    type Item = &'a mut Move;
    type IntoIter = slice::IterMut<'a, Move>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        (&mut self.moves).into_iter()
    }
}

impl Default for MoveList {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Square::{A1, A2, B1, B2, C1, C2};

    #[test]
    fn exposes_collection_behavior_without_exposing_storage() {
        let first = Move::new(A1, A2);
        let second = Move::new(B1, B2);
        let mut moves = MoveList::new();

        assert_eq!(moves.capacity(), 256);
        assert!(moves.is_empty());
        assert!(!moves.is_full());

        moves.push(first);
        moves.push(second);

        assert_eq!(moves.get(0), Some(&first));
        assert_eq!(moves.get(2), None);
        assert_eq!(
            (&moves).into_iter().copied().collect::<Vec<_>>(),
            vec![first, second]
        );

        *moves.get_mut(0).unwrap() = Move::new(C1, C2);
        assert_eq!(moves.as_slice()[0], Move::new(C1, C2));
    }

    #[test]
    fn filter_keeps_moves_in_order() {
        let mut moves = MoveList::new();
        moves.push(Move::new(A1, A2));
        moves.push(Move::new(B1, B2));
        moves.push(Move::new(C1, C2));

        moves.filter(|mv| mv.from() != B1);

        assert_eq!(moves.len(), 2);
        assert_eq!(moves.as_slice(), &[Move::new(A1, A2), Move::new(C1, C2)]);
    }
}
