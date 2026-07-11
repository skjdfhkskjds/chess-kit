use core::{
    borrow::{Borrow, BorrowMut},
    fmt,
    hash::{Hash, Hasher},
    hint,
    iter::FusedIterator,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut, Index, IndexMut},
    ptr, slice,
    slice::SliceIndex,
};

/// Retain is implemented by collections that can remove items in place while
/// preserving the order of the items that remain.
pub trait Retain<T> {
    /// Removes every item for which `predicate` returns `false`.
    fn retain(&mut self, predicate: impl FnMut(&T) -> bool);
}

/// A fixed-capacity vector backed by inline storage.
///
/// `FixedArray` never allocates. Its capacity is part of the type and only the
/// initialized prefix is exposed to callers. Spare slots remain uninitialized,
/// so construction does not require `T: Default` or initialize `N` values.
pub struct FixedArray<T, const N: usize> {
    len: usize,
    items: [MaybeUninit<T>; N],
}

impl<T, const N: usize> FixedArray<T, N> {
    /// The number of elements available in the inline allocation.
    pub const CAPACITY: usize = N;

    /// Creates an empty fixed-capacity vector.
    #[inline]
    pub const fn new() -> Self {
        Self {
            len: 0,
            items: [const { MaybeUninit::uninit() }; N],
        }
    }

    /// Returns the number of initialized elements.
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns the fixed capacity.
    #[inline]
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns the number of elements that can still be inserted.
    #[inline]
    pub const fn remaining_capacity(&self) -> usize {
        N - self.len
    }

    /// Returns whether the vector contains no elements.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns whether the vector has reached its capacity.
    #[inline]
    pub const fn is_full(&self) -> bool {
        self.len == N
    }

    /// Appends an element.
    ///
    /// # Panics
    ///
    /// Panics if the vector is full.
    #[inline]
    pub fn push(&mut self, item: T) {
        assert!(!self.is_full(), "fixed array is full");

        // SAFETY: capacity was checked above.
        unsafe { self.push_unchecked(item) };
    }

    /// Attempts to append an element, returning it when the vector is full.
    #[inline]
    pub fn try_push(&mut self, item: T) -> Result<(), T> {
        if self.is_full() {
            return Err(item);
        }

        // SAFETY: capacity was checked above.
        unsafe { self.push_unchecked(item) };
        Ok(())
    }

    /// Appends an element without checking capacity.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `self.len() < self.capacity()`.
    #[inline]
    pub unsafe fn push_unchecked(&mut self, item: T) {
        debug_assert!(!self.is_full(), "fixed array is full");

        // SAFETY: this is the method's caller contract. Encoding it as an
        // optimizer assumption preserves the useful range information from
        // the checked `push` path without retaining its runtime branch.
        unsafe { hint::assert_unchecked(!self.is_full()) };

        // SAFETY: upheld by the caller. The slot at `len` is outside the
        // initialized prefix and can be written without dropping a value.
        unsafe { self.items.get_unchecked_mut(self.len).write(item) };
        self.len += 1;
    }

    /// Inserts an element at `index`, shifting later elements to the right.
    ///
    /// # Panics
    ///
    /// Panics if `index > len` or the vector is full.
    #[inline]
    pub fn insert(&mut self, index: usize, item: T) {
        assert!(index <= self.len, "fixed array index out of bounds");
        assert!(!self.is_full(), "fixed array is full");

        // SAFETY: the index and capacity were checked above.
        unsafe { self.insert_unchecked(index, item) };
    }

    /// Inserts an element without checking the index or capacity.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `index <= self.len()` and that the vector
    /// is not full.
    #[inline]
    pub unsafe fn insert_unchecked(&mut self, index: usize, item: T) {
        debug_assert!(index <= self.len, "fixed array index out of bounds");
        debug_assert!(!self.is_full(), "fixed array is full");

        // SAFETY: these are the method's caller contracts.
        unsafe { hint::assert_unchecked(index <= self.len && !self.is_full()) };

        // SAFETY: upheld by the caller. `copy` permits the overlapping ranges
        // created while shifting the initialized suffix one slot to the right.
        unsafe {
            let base = self.items.as_mut_ptr();
            ptr::copy(base.add(index), base.add(index + 1), self.len - index);
            base.add(index).write(MaybeUninit::new(item));
        }
        self.len += 1;
    }

    /// Removes and returns the last element, if any.
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.len -= 1;
        // SAFETY: the old last slot was initialized and reducing `len` makes
        // ownership of it available to the caller.
        Some(unsafe { self.items.get_unchecked(self.len).assume_init_read() })
    }

    /// Removes and returns the element at `index`, shifting later elements to
    /// the left.
    ///
    /// # Panics
    ///
    /// Panics if `index >= len`.
    #[inline]
    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "fixed array index out of bounds");

        // SAFETY: the index was checked above.
        unsafe { self.remove_unchecked(index) }
    }

    /// Removes an element without checking the index.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `index < self.len()`.
    #[inline]
    pub unsafe fn remove_unchecked(&mut self, index: usize) -> T {
        debug_assert!(index < self.len, "fixed array index out of bounds");

        // SAFETY: this is the method's caller contract.
        unsafe { hint::assert_unchecked(index < self.len) };

        // SAFETY: upheld by the caller. The removed value is read before the
        // overlapping initialized suffix is shifted left.
        unsafe {
            let base = self.items.as_mut_ptr();
            let removed = base.add(index).read().assume_init();
            ptr::copy(base.add(index + 1), base.add(index), self.len - index - 1);
            self.len -= 1;
            removed
        }
    }

    /// Removes an element by replacing it with the last element.
    ///
    /// # Panics
    ///
    /// Panics if `index >= len`.
    #[inline]
    pub fn swap_remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "fixed array index out of bounds");

        let last = self.len - 1;
        // SAFETY: `index` and `last` are within the initialized prefix.
        unsafe {
            let removed = self.items.get_unchecked(index).assume_init_read();
            if index != last {
                let replacement = self.items.get_unchecked(last).assume_init_read();
                self.items.get_unchecked_mut(index).write(replacement);
            }
            self.len = last;
            removed
        }
    }

    /// Shortens the vector to `new_len`, dropping removed elements.
    #[inline]
    pub fn truncate(&mut self, new_len: usize) {
        if new_len >= self.len {
            return;
        }

        let removed_len = self.len - new_len;
        self.len = new_len;

        // SAFETY: the removed range was initialized before `len` was reduced.
        // Updating `len` first prevents double drops if an element destructor
        // panics.
        unsafe {
            let removed = slice::from_raw_parts_mut(
                self.items.as_mut_ptr().add(new_len).cast::<T>(),
                removed_len,
            );
            ptr::drop_in_place(removed);
        }
    }

    /// Removes all elements.
    #[inline]
    pub fn clear(&mut self) {
        self.truncate(0);
    }

    /// Returns the initialized elements as a slice.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        // SAFETY: `0..len` is always the initialized prefix.
        unsafe { slice::from_raw_parts(self.items.as_ptr().cast::<T>(), self.len) }
    }

    /// Returns the initialized elements as a mutable slice.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        // SAFETY: `0..len` is always the initialized prefix and `&mut self`
        // guarantees exclusive access.
        unsafe { slice::from_raw_parts_mut(self.items.as_mut_ptr().cast::<T>(), self.len) }
    }

    /// Returns a pointer to the first element.
    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        self.items.as_ptr().cast::<T>()
    }

    /// Returns a mutable pointer to the first element.
    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        self.items.as_mut_ptr().cast::<T>()
    }

    /// Returns the uninitialized spare capacity.
    #[inline]
    pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>] {
        &mut self.items[self.len..]
    }

    /// Sets the initialized length without modifying storage.
    ///
    /// # Safety
    ///
    /// `new_len` must not exceed capacity. Every slot in the new initialized
    /// prefix must contain a valid `T`, and values removed by reducing the
    /// length must already have been dropped.
    #[inline]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        debug_assert!(new_len <= N, "fixed array length exceeds capacity");
        // SAFETY: this is part of the method's caller contract.
        unsafe { hint::assert_unchecked(new_len <= N) };
        self.len = new_len;
    }
}

impl<T, const N: usize> Retain<T> for FixedArray<T, N> {
    #[inline]
    fn retain(&mut self, mut predicate: impl FnMut(&T) -> bool) {
        let original_len = self.len;
        let mut first_removed = 0;

        // Fast path: no bookkeeping or writes are needed when every element
        // is retained, which is the common case for move filtering.
        while first_removed < original_len {
            // SAFETY: `first_removed` is within the initialized prefix.
            let item = unsafe { self.items.get_unchecked(first_removed).assume_init_ref() };
            if !predicate(item) {
                break;
            }
            first_removed += 1;
        }

        if first_removed == original_len {
            return;
        }

        // Hide every element from `Drop` while compaction is in progress. The
        // guard restores a valid initialized prefix on success or unwinding.
        self.len = 0;
        let mut guard = BackshiftOnDrop::new(self, original_len, first_removed);

        // The first rejected element starts the first hole.
        guard.processed += 1;
        guard.deleted += 1;
        // SAFETY: `first_removed` is initialized and was rejected.
        unsafe {
            ptr::drop_in_place(
                guard
                    .array
                    .as_mut()
                    .unwrap_unchecked()
                    .items
                    .get_unchecked_mut(first_removed)
                    .as_mut_ptr(),
            )
        };

        while guard.processed < original_len {
            let read = guard.processed;
            // SAFETY: `read` is in the unprocessed initialized suffix.
            let keep = unsafe {
                let array = guard.array.as_ref().unwrap_unchecked();
                predicate(array.items.get_unchecked(read).assume_init_ref())
            };

            if keep {
                // SAFETY: `deleted > 0`, so source and destination are
                // distinct initialized/uninitialized slots.
                unsafe {
                    let array = guard.array.as_mut().unwrap_unchecked();
                    let base = array.items.as_mut_ptr();
                    ptr::copy_nonoverlapping(base.add(read), base.add(read - guard.deleted), 1);
                }
                guard.processed += 1;
            } else {
                guard.processed += 1;
                guard.deleted += 1;
                // SAFETY: the rejected slot is initialized and excluded from
                // both the compacted prefix and unprocessed suffix.
                unsafe {
                    let array = guard.array.as_mut().unwrap_unchecked();
                    ptr::drop_in_place(array.items.get_unchecked_mut(read).as_mut_ptr());
                }
            }
        }
    }
}

impl<T, const N: usize> Default for FixedArray<T, N> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone, const N: usize> Clone for FixedArray<T, N> {
    #[inline]
    fn clone(&self) -> Self {
        let mut clone = Self::new();
        for item in self {
            // SAFETY: the source contains at most `N` initialized elements.
            unsafe { clone.push_unchecked(item.clone()) };
        }
        clone
    }
}

impl<T: fmt::Debug, const N: usize> fmt::Debug for FixedArray<T, N> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(self).finish()
    }
}

impl<T: Hash, const N: usize> Hash for FixedArray<T, N> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl<T, U, const N: usize, const M: usize> PartialEq<FixedArray<U, M>> for FixedArray<T, N>
where
    T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &FixedArray<U, M>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: Eq, const N: usize> Eq for FixedArray<T, N> {}

impl<T: PartialOrd, const N: usize> PartialOrd for FixedArray<T, N> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl<T: Ord, const N: usize> Ord for FixedArray<T, N> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl<T, const N: usize> Deref for FixedArray<T, N> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, const N: usize> DerefMut for FixedArray<T, N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T, const N: usize> AsRef<[T]> for FixedArray<T, N> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, const N: usize> AsMut<[T]> for FixedArray<T, N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T, const N: usize> Borrow<[T]> for FixedArray<T, N> {
    #[inline]
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, const N: usize> BorrowMut<[T]> for FixedArray<T, N> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T, I, const N: usize> Index<I> for FixedArray<T, N>
where
    I: SliceIndex<[T]>,
{
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<T, I, const N: usize> IndexMut<I> for FixedArray<T, N>
where
    I: SliceIndex<[T]>,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.as_mut_slice()[index]
    }
}

impl<T, const N: usize> Extend<T> for FixedArray<T, N> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

impl<'a, T: Copy + 'a, const N: usize> Extend<&'a T> for FixedArray<T, N> {
    #[inline]
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().copied());
    }
}

impl<T, const N: usize> FromIterator<T> for FixedArray<T, N> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut array = Self::new();
        array.extend(iter);
        array
    }
}

impl<T, const N: usize> From<[T; N]> for FixedArray<T, N> {
    #[inline]
    fn from(items: [T; N]) -> Self {
        Self {
            len: N,
            items: items.map(MaybeUninit::new),
        }
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a FixedArray<T, N> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut FixedArray<T, N> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.as_mut_slice().iter_mut()
    }
}

impl<T, const N: usize> IntoIterator for FixedArray<T, N> {
    type Item = T;
    type IntoIter = FixedArrayIntoIter<T, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        let this = ManuallyDrop::new(self);
        // SAFETY: `this` will not be dropped, so ownership of the backing
        // array can be transferred to the iterator.
        let items = unsafe { ptr::read(&this.items) };

        FixedArrayIntoIter {
            items,
            front: 0,
            back: this.len,
        }
    }
}

impl<T, const N: usize> Drop for FixedArray<T, N> {
    #[inline]
    fn drop(&mut self) {
        self.clear();
    }
}

/// An owning iterator over a [`FixedArray`].
pub struct FixedArrayIntoIter<T, const N: usize> {
    items: [MaybeUninit<T>; N],
    front: usize,
    back: usize,
}

impl<T, const N: usize> FixedArrayIntoIter<T, N> {
    /// Returns the elements that have not yet been yielded.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        // SAFETY: `front..back` is the remaining initialized range.
        unsafe {
            slice::from_raw_parts(
                self.items.as_ptr().add(self.front).cast::<T>(),
                self.back - self.front,
            )
        }
    }

    /// Returns the elements that have not yet been yielded mutably.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        // SAFETY: `front..back` is the remaining initialized range.
        unsafe {
            slice::from_raw_parts_mut(
                self.items.as_mut_ptr().add(self.front).cast::<T>(),
                self.back - self.front,
            )
        }
    }
}

impl<T, const N: usize> Iterator for FixedArrayIntoIter<T, N> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }

        let index = self.front;
        self.front += 1;
        // SAFETY: `index` was within the remaining initialized range and is
        // removed from that range before ownership is returned.
        Some(unsafe { self.items.get_unchecked(index).assume_init_read() })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }
}

impl<T, const N: usize> DoubleEndedIterator for FixedArrayIntoIter<T, N> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }

        self.back -= 1;
        // SAFETY: `back` now identifies the last remaining initialized item.
        Some(unsafe { self.items.get_unchecked(self.back).assume_init_read() })
    }
}

impl<T, const N: usize> ExactSizeIterator for FixedArrayIntoIter<T, N> {
    #[inline]
    fn len(&self) -> usize {
        self.back - self.front
    }
}

impl<T, const N: usize> FusedIterator for FixedArrayIntoIter<T, N> {}

impl<T: fmt::Debug, const N: usize> fmt::Debug for FixedArrayIntoIter<T, N> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("FixedArrayIntoIter")
            .field(&self.as_slice())
            .finish()
    }
}

impl<T, const N: usize> Drop for FixedArrayIntoIter<T, N> {
    fn drop(&mut self) {
        // SAFETY: only `front..back` remains initialized and owned by the
        // iterator.
        unsafe { ptr::drop_in_place(self.as_mut_slice()) };
    }
}

struct BackshiftOnDrop<T, const N: usize> {
    array: *mut FixedArray<T, N>,
    original_len: usize,
    processed: usize,
    deleted: usize,
}

impl<T, const N: usize> BackshiftOnDrop<T, N> {
    #[inline]
    fn new(array: &mut FixedArray<T, N>, original_len: usize, processed: usize) -> Self {
        Self {
            array,
            original_len,
            processed,
            deleted: 0,
        }
    }
}

impl<T, const N: usize> Drop for BackshiftOnDrop<T, N> {
    fn drop(&mut self) {
        // SAFETY: `processed..original_len` is the unprocessed initialized
        // suffix and `processed - deleted` is the next hole. Moving that suffix
        // closes every hole, leaving one initialized prefix.
        unsafe {
            let array = &mut *self.array;
            if self.deleted != 0 {
                let base = array.items.as_mut_ptr();
                ptr::copy(
                    base.add(self.processed),
                    base.add(self.processed - self.deleted),
                    self.original_len - self.processed,
                );
            }
            array.len = self.original_len - self.deleted;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::cell::Cell;
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::rc::Rc;

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct TestItem(u8);

    #[test]
    fn supports_vector_operations() {
        let mut items = FixedArray::<TestItem, 4>::new();

        assert_eq!(items.capacity(), 4);
        assert_eq!(items.remaining_capacity(), 4);
        assert!(items.is_empty());

        items.push(TestItem(1));
        items.try_push(TestItem(3)).unwrap();
        items.insert(1, TestItem(2));
        items.push(TestItem(4));

        assert!(items.is_full());
        assert_eq!(
            &*items,
            &[TestItem(1), TestItem(2), TestItem(3), TestItem(4)]
        );
        assert_eq!(items.remove(1), TestItem(2));
        assert_eq!(items.swap_remove(0), TestItem(1));
        assert_eq!(items.pop(), Some(TestItem(3)));
        assert_eq!(items.as_slice(), &[TestItem(4)]);
    }

    #[test]
    fn unchecked_and_spare_capacity_apis_preserve_the_initialized_prefix() {
        let mut items = FixedArray::<u8, 3>::new();

        unsafe { items.push_unchecked(1) };
        items.spare_capacity_mut()[0].write(2);
        unsafe { items.set_len(2) };

        assert_eq!(items.as_slice(), &[1, 2]);
        assert_eq!(unsafe { *items.get_unchecked(1) }, 2);
    }

    #[test]
    fn slice_and_collection_traits_are_available() {
        let mut items: FixedArray<u8, 4> = [3, 1, 2].into_iter().collect();
        items.sort_unstable();
        items[1] = 4;

        let clone = items.clone();
        assert_eq!(items.as_ref(), &[1, 4, 3]);
        assert_eq!(clone, items);
        assert_eq!(format!("{items:?}"), "[1, 4, 3]");
    }

    #[test]
    fn borrowed_iterators_have_slice_iterator_hints() {
        fn assert_hints<I>(iter: I)
        where
            I: DoubleEndedIterator + ExactSizeIterator + FusedIterator,
        {
            assert_eq!(iter.len(), 3);
        }

        let items: FixedArray<u8, 3> = [1, 2, 3].into_iter().collect();
        assert_hints(items.iter());
        assert_eq!(
            (&items).into_iter().rev().copied().collect::<Vec<_>>(),
            [3, 2, 1]
        );
    }

    #[test]
    fn owning_iterator_is_exact_size_double_ended_and_drops_the_remainder() {
        struct DropItem(Rc<Cell<usize>>, u8);

        impl Drop for DropItem {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }

        let drops = Rc::new(Cell::new(0));
        let mut items = FixedArray::<DropItem, 3>::new();
        for value in 1..=3 {
            items.push(DropItem(drops.clone(), value));
        }

        let mut iter = items.into_iter();
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next().unwrap().1, 1);
        assert_eq!(iter.next_back().unwrap().1, 3);
        assert_eq!(iter.as_slice()[0].1, 2);
        assert_eq!(drops.get(), 2);

        drop(iter);
        assert_eq!(drops.get(), 3);
    }

    #[test]
    fn retain_preserves_order_and_uses_the_all_kept_fast_path() {
        let mut items: FixedArray<TestItem, 5> = (0..5).map(TestItem).collect();

        items.retain(|_| true);
        assert_eq!(
            items.as_slice(),
            &[
                TestItem(0),
                TestItem(1),
                TestItem(2),
                TestItem(3),
                TestItem(4)
            ]
        );

        items.retain(|item| item.0 % 2 == 0);
        assert_eq!(items.as_slice(), &[TestItem(0), TestItem(2), TestItem(4)]);
    }

    #[test]
    fn clear_remove_and_drop_drop_each_element_once() {
        struct DropItem(Rc<Cell<usize>>);

        impl Drop for DropItem {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }

        let drops = Rc::new(Cell::new(0));
        let mut items = FixedArray::<DropItem, 3>::new();
        for _ in 0..3 {
            items.push(DropItem(drops.clone()));
        }

        drop(items.remove(1));
        items.clear();
        assert_eq!(drops.get(), 3);
    }

    #[test]
    fn retain_restores_a_valid_collection_when_the_predicate_panics() {
        let drops = Rc::new(Cell::new(0));

        struct DropItem(Rc<Cell<usize>>, u8);

        impl Drop for DropItem {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }

        let mut items = FixedArray::<DropItem, 4>::new();
        for value in 0..4 {
            items.push(DropItem(drops.clone(), value));
        }

        let result = catch_unwind(AssertUnwindSafe(|| {
            items.retain(|item| {
                assert!(item.1 != 2);
                item.1 != 0
            });
        }));

        assert!(result.is_err());
        assert_eq!(
            items.iter().map(|item| item.1).collect::<Vec<_>>(),
            [1, 2, 3]
        );
        assert_eq!(drops.get(), 1);

        drop(items);
        assert_eq!(drops.get(), 4);
    }
}
