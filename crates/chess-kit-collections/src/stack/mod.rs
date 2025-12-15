mod iterator;

// Copyable is a trait that defines the behavior of a copyable item
//
// note: this trait is different from `Copy`, with the explicit intention of
//       defining custom copy behaviour without sacrificing support from the
//       standard `Copy` behaviour
//
// @trait
pub trait Copyable: Default + Copy + Clone {
    fn copy_from(&mut self, other: &Self);
}

// DEFAULT_CAPACITY is the default capacity of the stack
const DEFAULT_CAPACITY: usize = u8::MAX as usize;

// Stack is a non-opinionated stack data structure that can be used to store any
// type of item that is copyable
//
// note: we do a custom definition of stack so that we can use the `push_next`
//       pattern to do lightweight derivation of new items from existing ones
pub struct Stack<T: Copyable, const CAP: usize = DEFAULT_CAPACITY> {
    pub(super) current: usize,  // index of the current state
    pub(super) items: [T; CAP], // stack of previous states
}

impl<T: Copyable, const CAP: usize> Stack<T, CAP> {
    // new creates a new stack with all items initialized to the default
    //
    // @return: new stack
    pub fn new() -> Self {
        Self {
            current: 0,
            items: [T::default(); CAP],
        }
    }

    // push adds a new item to the stack
    //
    // @param: item - item to add to the stack
    // @return: void
    // @side-effects: modifies the stack, increments the current index
    // @requires: the current index is less than the stack capacity
    #[inline(always)]
    pub fn push(&mut self, item: T) {
        debug_assert!(self.current < CAP, "stack is full");
        self.current += 1;
        self.items[self.current] = item;
    }

    // push_next adds a new item to the stack by deriving it from the copy of
    // the current item
    //
    // @return: mutable reference to the newly pushed item
    // @side-effects: modifies the stack, increments the current index
    // @requires: the stack is non-empty and not full
    #[inline(always)]
    pub fn push_next(&mut self) -> &mut T {
        debug_assert!(self.current > 0, "cannot clone from an empty stack");
        debug_assert!(self.current < CAP, "stack is full");

        let src_item = self.items[self.current];
        self.current += 1;
        self.items[self.current].copy_from(&src_item);
        &mut self.items[self.current]
    }

    // pop removes the last item from the stack and returns it
    //
    // @side-effects: modifies the stack, decrements the current index
    // @requires: the current index is greater than 1
    #[inline(always)]
    pub fn pop(&mut self) {
        if self.current == 0 {
            return;
        }

        self.current -= 1;
    }

    // top returns an immutable reference to the top item
    //
    // @return: reference to the current item
    // @requires: the stack is non-empty
    #[inline(always)]
    pub fn top(&self) -> &T {
        debug_assert!(self.current > 0, "stack is empty");
        &self.items[self.current]
    }

    // top_mut returns a mutable reference to the top item
    //
    // @return: mutable reference to the current item
    // @requires: the stack is non-empty
    #[inline(always)]
    pub fn top_mut(&mut self) -> &mut T {
        debug_assert!(self.current > 0, "stack is empty");
        &mut self.items[self.current]
    }

    // size returns the number of items in the stack
    //
    // @return: the number of items in the stack
    #[inline(always)]
    pub fn size(&self) -> usize {
        self.current
    }

    // is_empty returns true if the stack is empty
    //
    // @return: true if the stack is empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.current == 0
    }

    // is_full returns true if the stack is full
    //
    // @return: true if the stack is full
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        self.current == CAP
    }

    // clear resets the stack to an empty state
    //
    // @return: void
    // @side-effects: sets the current index to 0
    #[inline(always)]
    pub fn clear(&mut self) {
        self.current = 0;
    }
}

impl<T: Copyable, const CAP: usize> Default for Stack<T, CAP> {
    fn default() -> Self {
        Self::new()
    }
}
