use crate::primitives::Stack;
use crate::primitives::stack::Copyable;

// StackIter is a double-ended iterator over the stack
pub struct StackIter<'a, T: Copyable, const CAP: usize> {
    stack: &'a Stack<T, CAP>,
    front: usize,
    back: usize,
}

impl<'a, T: Copyable, const CAP: usize> Iterator for StackIter<'a, T, CAP> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }

        let item = &self.stack.items[self.front];
        self.front += 1;
        Some(item)
    }
}

impl<'a, T: Copyable, const CAP: usize> DoubleEndedIterator for StackIter<'a, T, CAP> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front > self.back {
            return None;
        }

        let item = &self.stack.items[self.back - 1];
        self.back -= 1;
        Some(item)
    }
}

impl<T: Copyable, const CAP: usize> Stack<T, CAP> {
    pub fn iter(&self) -> StackIter<'_, T, CAP> {
        StackIter {
            stack: self,
            front: 0,
            back: self.size(),
        }
    }
}
