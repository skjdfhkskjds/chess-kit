use core::mem::MaybeUninit;
use crate::primitives::moves::Move;

const MAX_MOVES: usize = 256;

pub struct MoveList {
    len: usize,
    buf: [MaybeUninit<Move>; MAX_MOVES],
}

impl MoveList {
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

    #[inline]
    pub fn clear(&mut self) {
        // We don't need to drop anything because Move: Copy and has no Drop
        self.len = 0;
    }

    #[inline]
    pub fn push(&mut self, mv: Move) {
        debug_assert!(self.len < MAX_MOVES);
        self.buf[self.len].write(mv);
        self.len += 1;
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    // get returns the move at the given index
    // 
    // @param: idx - index of the move to get
    // @return: move at the given index
    // Note: this is an unsafe operation since it assumes that the index is
    //       valid, but we allow this for performance reasons.
    #[inline]
    pub fn get(&self, idx: usize) -> Move {
        unsafe { self.buf[idx].assume_init() }
    }

    // get_safe returns the move at the given index, or None if the index is
    // out of bounds.
    // 
    // @param: idx - index of the move to get
    // @return: move at the given index, or None if the index is out of bounds
    #[inline]
    pub fn get_safe(&self, idx: usize) -> Option<Move> {
        (idx < self.len).then(|| {
            self.get(idx)
        })
    }

    // get_mut returns a mutable reference to the move at the given index
    // 
    // @param: idx - index of the move to get
    // @return: mutable reference to the move at the given index
    // Note: this is an unsafe operation since it assumes that the index is
    //       valid, but we allow this for performance reasons.
    #[inline]
    pub fn get_mut(&mut self, idx: usize) -> &mut Move {
        unsafe { self.buf[idx].assume_init_mut() }
    }

    // get_mut_safe returns a mutable reference to the move at the given index,
    // or None if the index is out of bounds.
    // 
    // @param: idx - index of the move to get
    // @return: mutable reference to the move at the given index
    #[inline]
    pub fn get_mut_safe(&mut self, idx: usize) -> Option<&mut Move> {
        (idx < self.len).then(|| {
            self.get_mut(idx)
        })
    }

    /// Iterate by value (copying moves out)
    pub fn iter(&self) -> impl Iterator<Item = Move> + '_ {
        (0..self.len).map(move |i| {
            // SAFETY: same reasoning as in `get`.
            unsafe { self.buf[i].assume_init() }
        })
    }
}
