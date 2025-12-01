use crate::transposition::{NodeData, entry::Entry};

// TODO: make this configurable
pub const ENTRIES_PER_BUCKET: usize = 3;

#[derive(Clone)]
pub struct Bucket<T: NodeData> {
    entries: [Entry<T>; ENTRIES_PER_BUCKET],
}

impl<T> Bucket<T>
where
    T: NodeData + Copy,
{
    // new creates a new bucket with all entries initialized to empty
    // 
    // @return: new bucket
    #[inline(always)]
    pub(crate) fn new() -> Self {
        Self {
            entries: [Entry::new(); ENTRIES_PER_BUCKET],
        }
    }

    // get fetches the first entry with the given key
    // 
    // @param: key - key to fetch the data for
    // @return: data if the key is found, None otherwise
    pub(crate) fn get(&self, key: u32) -> Option<&T> {
        for entry in self.entries.iter() {
            if entry.key() == key {
                return Some(entry.data());
            }
        }
        None
    }

    // set sets the value of the entry with the lowest depth to the given key
    // data pair
    // 
    // @param: key - key to set the value to
    // @param: data - data to set the value to
    // @return: true if an entry was set for the first time, false otherwise
    pub(crate) fn set(&mut self, key: u32, data: T) -> bool {
        // find the index of the entry with the lowest depth
        let mut min_depth = i8::MAX;
        let mut min_depth_idx = 0;
        for i in 0..ENTRIES_PER_BUCKET {
            let depth = self.entries[i].data().depth();
            if depth < min_depth {
                min_depth = depth;
                min_depth_idx = i;
            }
        }

        // check if the entry was dirty before this operation
        let was_dirty = self.entries[min_depth_idx].is_dirty();

        // set the value of the entry
        // 
        // Note: we always replace the old value
        self.entries[min_depth_idx].set(key, data);

        !was_dirty
    }

    // clear clears the bucket to a clean state
    // 
    // @return: void
    // @side-effects: clears each entry in the bucket
    pub(crate) fn clear(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.clear();
        }
    }

    // size_of_mem returns the size of the memory occupied by the bucket
    // 
    // @return: size of the memory occupied by the bucket
    #[inline(always)]
    pub(crate) const fn size_of_mem() -> usize {
        std::mem::size_of::<Entry<T>>() * ENTRIES_PER_BUCKET
    }

    // capacity returns the maximum number of entries in the bucket
    // 
    // @return: maximum number of entries in the bucket
    #[inline(always)]
    pub(crate) const fn capacity() -> usize {
        ENTRIES_PER_BUCKET
    }
}

impl<T> Default for Bucket<T>
where
    T: NodeData + Copy,
{
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
