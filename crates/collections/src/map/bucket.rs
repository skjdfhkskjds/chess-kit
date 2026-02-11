use super::{Entry, Value};

/// DEFAULT_SIZE is the default number of entries per bucket
pub const DEFAULT_SIZE: usize = 3;

/// Bucket is a collection of entries that are used to store data in a map
///
/// @type
#[derive(Clone)]
pub struct Bucket<T: Value, const SIZE: usize = DEFAULT_SIZE> {
    entries: [Entry<T>; SIZE],
}

impl<T: Value, const SIZE: usize> Bucket<T, SIZE> {
    /// new creates a new bucket with all entries initialized to empty
    ///
    /// @return: new bucket
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            entries: [Entry::new(); SIZE],
        }
    }

    /// get fetches the first entry with the given key
    ///
    /// @param: key - key to fetch the data for
    /// @return: data if the key is found, None otherwise
    #[inline(always)]
    pub fn get(&self, key: u32) -> Option<&T> {
        for entry in self.entries.iter() {
            if entry.key() == key {
                return Some(entry.data());
            }
        }
        None
    }

    /// set sets the value of the entry with the lowest priority to the given key
    /// data pair
    ///
    /// @param: key - key to set the value to
    /// @param: data - data to set the value to
    /// @return: true if an entry was set for the first time, false otherwise
    #[inline(always)]
    pub fn set(&mut self, key: u32, data: T) -> bool {
        // find the index of the entry with the lowest priority
        let mut min_priority = i8::MAX;
        let mut min_priority_idx = 0;
        for i in 0..SIZE {
            let priority = self.entries[i].data().priority();
            if priority < min_priority {
                min_priority = priority;
                min_priority_idx = i;
            }
        }

        // check if the entry was dirty before this operation
        let was_dirty = self.entries[min_priority_idx].is_dirty();

        // set the value of the entry
        //
        // note: we always replace the old value
        self.entries[min_priority_idx].set(key, data);

        !was_dirty
    }

    /// clear clears the bucket to a clean state
    ///
    /// @return: void
    /// @side-effects: clears each entry in the bucket
    #[inline(always)]
    pub fn clear(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.clear();
        }
    }

    /// size_of_mem returns the size of the memory occupied by the bucket
    ///
    /// @return: size of the memory occupied by the bucket
    #[inline(always)]
    pub const fn size_of_mem() -> usize {
        std::mem::size_of::<Entry<T>>() * DEFAULT_SIZE
    }

    /// capacity returns the maximum number of entries in the bucket
    ///
    /// @return: maximum number of entries in the bucket
    #[inline(always)]
    pub const fn capacity() -> usize {
        DEFAULT_SIZE
    }
}

impl<T: Value, const SIZE: usize> Default for Bucket<T, SIZE> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
