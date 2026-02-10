mod bucket;
mod entry;

pub use bucket::Bucket;
pub use entry::Entry;

use std::marker::PhantomData;

// HashFn is a trait that defines the contract for a function that can hash a
// key into an index and a key id
//
// @trait
pub trait HashFn<K> {
    // hash hashes the given key and returns the raw index and the key id
    //
    // note: the raw index is the index of the bucket in the map prior to any
    //       modulo operations
    //
    // @param: key - the key to hash
    // @return: the raw index and the key id
    fn hash(key: K) -> (usize, u32);
}

// Value is a trait that defines the contract for a type that can be used as a
// value in a map
//
// @trait
pub trait Value: Copy + Default {
    // priority returns the priority of the value
    //
    // @return: the priority of the value
    fn priority(&self) -> i8;
}

const MB_TO_BYTES: usize = 1024 * 1024;

// Map is a generic hash map implementation that uses maximum length bucket-
// chaining and a priority-based eviction policy
pub struct Map<K, V: Value, Hasher: HashFn<K>> {
    size: usize,        // number of entries in the map
    capacity: usize,    // maximum number of entries in the map
    max_buckets: usize, // maximum number of buckets in the map
    memory_size: usize, // amount of memory allocated in MBs

    buckets: Vec<Bucket<V>>, // buckets of the map

    _key: PhantomData<K>,
    _hasher: PhantomData<Hasher>,
}

impl<K, V: Value, Hasher: HashFn<K>> Map<K, V, Hasher> {
    // new creates a new map with the requested memory size
    //
    // @param: memory_size - the size of the map in MBs
    // @return: a new map
    pub fn new(memory_size: usize) -> Self {
        let (buckets, capacity) = Self::calculate_sizes(memory_size);

        Self {
            size: 0,
            capacity,
            max_buckets: buckets,
            memory_size: memory_size,
            buckets: vec![Bucket::<V>::new(); buckets],
            _key: PhantomData,
            _hasher: PhantomData,
        }
    }

    // set sets an entry in the map
    //
    // @param: key - the key of the entry to set
    // @param: data - the data to insert
    // @return: void
    // @side-effects: modifies the transposition table
    #[inline(always)]
    pub fn set(&mut self, key: K, data: V) {
        if !self.is_enabled() {
            return;
        }

        let (idx, key_id) = Hasher::hash(key);
        let bucket_idx = idx % self.max_buckets;
        self.size += self.buckets[bucket_idx].set(key_id, data) as usize;
    }

    // get gets an entry from the map with the given key
    //
    // @param: key - the key of the entry to get
    // @return: the data if the entry is found, None otherwise
    #[inline(always)]
    pub fn get(&self, key: K) -> Option<&V> {
        if !self.is_enabled() {
            return None;
        }

        let (idx, key_id) = Hasher::hash(key);
        let bucket_idx = idx % self.max_buckets;
        self.buckets[bucket_idx].get(key_id)
    }

    // is_enabled checks if the map is enabled
    //
    // @return: true if the map is enabled, false otherwise
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        self.memory_size > 0
    }

    // capacity returns the maximum number of entries in the map
    //
    // @return: maximum number of entries in the map
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    // resize resizes the map's underlying memory allocation to the requested
    // size
    //
    // @param: memory_size - the new size of the map in MBs
    // @return: void
    // @side-effects: clears the map
    #[inline(always)]
    pub fn resize(&mut self, memory_size: usize) {
        // if the memory size is unchanged, just clear the map
        if self.memory_size == memory_size {
            self.clear();
            return;
        }

        // reassign the map to a new one with the requested memory size
        *self = Self::new(memory_size);
    }

    // clear clears the map
    //
    // @return: void
    // @side-effects: clears the map
    #[inline(always)]
    pub fn clear(&mut self) {
        for entry in self.buckets.iter_mut() {
            entry.clear();
        }
        self.size = 0;
    }

    // buckets returns the number of buckets allocated in the map
    //
    // @return: number of buckets allocated in the map
    #[inline(always)]
    pub fn buckets(&self) -> usize {
        self.max_buckets
    }

    // usage is a function to calculate the usage of the map as a ratio between
    // 0 and the given base
    //
    // @param: base - the base to use for the calculation
    // @return: usage ratio of the map
    #[inline(always)]
    pub fn usage(&self, base: f64) -> u16 {
        if !self.is_enabled() {
            return 0;
        }

        let fraction = self.size as f64 / self.capacity as f64;
        (fraction * base).floor() as u16
    }

    // calculate_sizes calculates the number of entries and buckets that can fit
    // into the requested amount of memory
    //
    // @param: memory_size - the amount of memory in MBs
    // @return: number of buckets and capacity that fit in memory_size
    #[inline(always)]
    const fn calculate_sizes(memory_size: usize) -> (usize, usize) {
        let buckets = MB_TO_BYTES / Bucket::<V>::size_of_mem() * memory_size;
        let capacity = buckets * Bucket::<V>::capacity();

        (buckets, capacity)
    }
}
