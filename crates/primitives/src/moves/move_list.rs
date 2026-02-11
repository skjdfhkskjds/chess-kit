use crate::moves::Move;
use core::mem::MaybeUninit;

const MAX_MOVES: usize = 256;

/// MoveList is a fixed-size list of moves
///
/// @type
pub struct MoveList {
    len: usize,
    buf: [MaybeUninit<Move>; MAX_MOVES],
}

impl MoveList {
    /// new creates a new move list
    ///
    /// @return: new move list
    pub const fn new() -> Self {
        // Note: the internal buffer is uninitialized since the API contract
        //       does not assume that there is any valid data in the buffer
        //       on construction. When used in the move generator, the buffer
        //       is filled with moves from `push`, so this is safe.
        const UNINIT: MaybeUninit<Move> = MaybeUninit::uninit();
        Self {
            len: 0,
            buf: [UNINIT; MAX_MOVES],
        }
    }

    /// clear clears the move list
    ///
    /// @return: void
    #[inline(always)]
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// push pushes a move to the move list
    ///
    /// @param: mv - move to push
    /// @return: void
    /// @side-effects: modifies the `move list`
    #[inline(always)]
    pub fn push(&mut self, mv: Move) {
        debug_assert!(self.len < MAX_MOVES);
        self.buf[self.len].write(mv);
        self.len += 1;
    }

    /// len returns the number of moves in the move list
    ///
    /// @return: number of moves in the move list
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    /// is_empty returns true if the move list is empty
    ///
    /// @return: true if the move list is empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// get returns the move at the given index
    ///
    /// Note: this is an unsafe operation since it assumes that the index is
    ///       valid, but we allow this for performance reasons.
    ///
    /// @param: idx - index of the move to get
    /// @return: move at the given index
    #[inline(always)]
    pub fn get(&self, idx: usize) -> Move {
        unsafe { self.buf[idx].assume_init() }
    }

    /// get_safe returns the move at the given index, or None if the index is
    /// out of bounds.
    ///
    /// @param: idx - index of the move to get
    /// @return: move at the given index, or None if the index is out of bounds
    #[inline(always)]
    pub fn get_safe(&self, idx: usize) -> Option<Move> {
        (idx < self.len).then(|| self.get(idx))
    }

    /// get_mut returns a mutable reference to the move at the given index
    ///
    /// Note: this is an unsafe operation since it assumes that the index is
    ///       valid, but we allow this for performance reasons.
    ///
    /// @param: idx - index of the move to get
    /// @return: mutable reference to the move at the given index
    #[inline(always)]
    pub fn get_mut(&mut self, idx: usize) -> &mut Move {
        unsafe { self.buf[idx].assume_init_mut() }
    }

    /// get_mut_safe returns a mutable reference to the move at the given index,
    /// or None if the index is out of bounds.
    ///
    /// @param: idx - index of the move to get
    /// @return: mutable reference to the move at the given index
    #[inline(always)]
    pub fn get_mut_safe(&mut self, idx: usize) -> Option<&mut Move> {
        (idx < self.len).then(|| self.get_mut(idx))
    }

    /// iter iterates over the moves in the move list
    ///
    /// @return: iterator over the moves in the move list
    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = Move> + '_ {
        (0..self.len).map(move |i| unsafe { self.buf[i].assume_init() })
    }

    /// filter filters the move list by the given predicate
    ///
    /// @param: predicate - predicate to filter the move list by
    /// @return: void
    /// @side-effects: modifies the `move list`
    #[inline(always)]
    pub fn filter(&mut self, predicate: impl Fn(Move) -> bool) {
        let mut write = 0;

        for read in 0..self.len {
            if predicate(unsafe { self.buf[read].assume_init() }) {
                if write != read {
                    self.buf.swap(write, read);
                }
                write += 1;
            }
        }

        self.len = write;
    }
}

impl Default for MoveList {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
