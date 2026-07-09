use std::slice;

/// Copyable is a trait that defines the behavior of a copyable item
///
/// note: this trait is different from `Copy`, with the explicit intention of
///       defining custom copy behaviour without sacrificing support from the
///       standard `Copy` behaviour
///
/// @trait
pub trait Copyable: Default + Copy + Clone {
    /// copy_from copies the contents of another Copyable item into this one
    ///
    /// @param: other - the item to copy from
    /// @return: void
    /// @side-effects: modifies this item
    fn copy_from(&mut self, other: &Self);
}

/// DEFAULT_CAPACITY is the default capacity of the stack
const DEFAULT_CAPACITY: usize = u8::MAX as usize;

/// Stack is a non-opinionated stack data structure that can be used to store any
/// type of item that is copyable
///
/// note: we do a custom definition of stack so that we can use the `push_next`
///       pattern to do lightweight derivation of new items from existing ones
///
/// @type
pub struct Stack<T: Copyable, const N: usize = DEFAULT_CAPACITY> {
    pub(super) current: usize,  // number of active items
    pub(super) items: [T; N],   // stack of previous states
}

impl<T: Copyable, const N: usize> Stack<T, N> {
    /// new creates a new stack with all items initialized to the default
    ///
    /// @return: new stack
    #[inline]
    pub fn new() -> Self {
        Self {
            current: 0,
            items: [T::default(); N],
        }
    }

    /// push adds a new item to the stack
    ///
    /// @param: item - item to add to the stack
    /// @return: void
    /// @side-effects: modifies the stack, increments the current index
    /// @requires: the current index is less than the stack capacity
    #[inline]
    pub fn push(&mut self, item: T) {
        debug_assert!(self.current < N, "stack is full");
        self.items[self.current] = item;
        self.current += 1;
    }

    /// push_next adds a new item to the stack by deriving it from the copy of
    /// the current item
    ///
    /// @return: mutable reference to the newly pushed item
    /// @side-effects: modifies the stack, increments the current index
    /// @requires: the stack is non-empty and not full
    #[inline]
    pub fn push_next(&mut self) -> &mut T {
        debug_assert!(self.current > 0, "cannot clone from an empty stack");
        debug_assert!(self.current < N, "stack is full");

        let src_item = self.items[self.current - 1];
        let dst_idx = self.current;
        self.current += 1;
        self.items[dst_idx].copy_from(&src_item);
        &mut self.items[dst_idx]
    }

    /// pop removes the last item from the stack and returns it
    ///
    /// @side-effects: modifies the stack, decrements the current index
    /// @requires: the current index is greater than 1
    #[inline]
    pub fn pop(&mut self) {
        if self.current == 0 {
            return;
        }

        self.current -= 1;
    }

    /// top returns an immutable reference to the top item
    ///
    /// @return: reference to the current item
    /// @requires: the stack is non-empty
    #[inline]
    pub fn top(&self) -> &T {
        debug_assert!(self.current > 0, "stack is empty");
        &self.items[self.current - 1]
    }

    /// top_mut returns a mutable reference to the top item
    ///
    /// @return: mutable reference to the current item
    /// @requires: the stack is non-empty
    #[inline]
    pub fn top_mut(&mut self) -> &mut T {
        debug_assert!(self.current > 0, "stack is empty");
        &mut self.items[self.current - 1]
    }

    /// size returns the number of items in the stack
    ///
    /// @return: the number of items in the stack
    #[inline]
    pub fn size(&self) -> usize {
        self.current
    }

    /// is_empty returns true if the stack is empty
    ///
    /// @return: true if the stack is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.current == 0
    }

    /// is_full returns true if the stack is full
    ///
    /// @return: true if the stack is full
    #[inline]
    pub fn is_full(&self) -> bool {
        self.current == N
    }

    /// clear resets the stack to an empty state
    ///
    /// @return: void
    /// @side-effects: sets the current index to 0
    #[inline]
    pub fn clear(&mut self) {
        self.current = 0;
    }

    /// as_slice returns the active stack items in insertion order.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.items[..self.current]
    }

    /// as_mut_slice returns the active stack items in insertion order.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.items[..self.current]
    }
}

/// StackIter is a double-ended iterator over the stack.
pub type StackIter<'a, T> = slice::Iter<'a, T>;

impl<T: Copyable, const N: usize> Stack<T, N> {
    pub fn iter(&self) -> StackIter<'_, T> {
        self.as_slice().iter()
    }
}

impl<T: Copyable, const CAP: usize> Default for Stack<T, CAP> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
    struct TestItem(u8);

    impl Copyable for TestItem {
        fn copy_from(&mut self, other: &Self) {
            *self = *other;
        }
    }

    #[test]
    fn push_uses_all_capacity() {
        let mut stack = Stack::<TestItem, 3>::new();

        stack.push(TestItem(1));
        stack.push(TestItem(2));
        stack.push(TestItem(3));

        assert!(stack.is_full());
        assert_eq!(stack.size(), 3);
        assert_eq!(stack.top(), &TestItem(3));
        assert_eq!(stack.as_slice(), &[TestItem(1), TestItem(2), TestItem(3)]);
    }

    #[test]
    fn push_next_copies_top_item_to_next_slot() {
        let mut stack = Stack::<TestItem, 3>::new();
        stack.push(TestItem(7));

        let next = stack.push_next();
        next.0 += 1;

        assert_eq!(stack.as_slice(), &[TestItem(7), TestItem(8)]);
    }

    #[test]
    fn pop_updates_top_and_empty_state() {
        let mut stack = Stack::<TestItem, 2>::new();
        stack.push(TestItem(1));
        stack.push(TestItem(2));

        stack.pop();
        assert_eq!(stack.top(), &TestItem(1));

        stack.pop();
        stack.pop();
        assert!(stack.is_empty());
    }

    #[test]
    fn iterator_supports_full_forward_and_reverse_iteration() {
        let mut stack = Stack::<TestItem, 4>::new();
        for value in 1..=4 {
            stack.push(TestItem(value));
        }

        let forward: Vec<_> = stack.iter().copied().collect();
        let reverse: Vec<_> = stack.iter().rev().copied().collect();

        assert_eq!(
            forward,
            vec![TestItem(1), TestItem(2), TestItem(3), TestItem(4)]
        );
        assert_eq!(
            reverse,
            vec![TestItem(4), TestItem(3), TestItem(2), TestItem(1)]
        );
    }
}
