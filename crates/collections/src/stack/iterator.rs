use super::{Copyable, Stack};
use std::slice;

/// StackIter is a double-ended iterator over the stack.
pub type StackIter<'a, T> = slice::Iter<'a, T>;

impl<T: Copyable, const CAP: usize> Stack<T, CAP> {
    pub fn iter(&self) -> StackIter<'_, T> {
        self.as_slice().iter()
    }
}
