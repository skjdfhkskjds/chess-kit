use crate::transposition::{NodeData, bucket::Bucket};

// TODO: make this configurable
pub const BUCKETS_PER_ENTRY: usize = 3;

#[derive(Clone)]
pub struct Entry<T: NodeData> {
    buckets: [Bucket<T>; BUCKETS_PER_ENTRY],
}

impl<T> Entry<T>
where
    T: NodeData + Copy,
{
    // new creates a new entry with all buckets initialized to empty
    // 
    // @return: new entry
    #[inline(always)]
    pub(crate) fn new() -> Self {
        Self {
            buckets: [Bucket::new(); BUCKETS_PER_ENTRY],
        }
    }

    // get fetches the first bucket containing the given key
    // 
    // @param: key - key to fetch the data for
    // @return: data if the key is found, None otherwise
    pub(crate) fn get(&self, key: u32) -> Option<&T> {
        for bucket in self.buckets.iter() {
            if bucket.key() == key {
                return Some(bucket.data());
            }
        }
        None
    }

    // set sets the value of the bucket with the lowest depth to the given key
    // data pair
    // 
    // @param: key - key to set the value to
    // @param: data - data to set the value to
    // @return: true if a bucket was set for the first time, false otherwise
    pub(crate) fn set(&mut self, key: u32, data: T) -> bool {
        // find the index of the entry with the lowest depth
        let mut min_depth = 0;
        let mut min_depth_idx = 0;
        for i in 0..BUCKETS_PER_ENTRY {
            let depth = self.buckets[i].data().depth();
            if depth < min_depth {
                min_depth = depth;
                min_depth_idx = i;
            }
        }

        // check if the bucket was dirty before this operation
        let was_dirty = self.buckets[min_depth_idx].is_dirty();

        // set the value of the bucket
        // 
        // Note: we always replace the old value
        self.buckets[min_depth_idx].set(key, data);

        !was_dirty
    }

    // clear clears the entry to an empty state
    // 
    // @return: void
    // @side-effects: modifies the entry
    pub(crate) fn clear(&mut self) {
        for bucket in self.buckets.iter_mut() {
            bucket.clear();
        }
    }

    // size_of_mem returns the size of the memory occupied by the entry
    // 
    // @return: size of the memory occupied by the entry
    #[inline(always)]
    pub(crate) const fn size_of_mem() -> usize {
        std::mem::size_of::<Bucket<T>>() * BUCKETS_PER_ENTRY
    }

    // num_buckets returns the number of buckets in the entry
    // 
    // @return: number of buckets in the entry
    #[inline(always)]
    pub(crate) const fn num_buckets() -> usize {
        BUCKETS_PER_ENTRY
    }
}

impl<T> Default for Entry<T>
where
    T: NodeData + Copy,
{
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
