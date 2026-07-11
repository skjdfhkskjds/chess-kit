mod bucket;
mod entry;

use bucket::{Bucket, SetResult};
use entry::Entry;
use std::marker::PhantomData;

/// HashKey is the bucket index and verification tag produced by a map hasher.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HashKey {
    /// Raw bucket index before modulo reduction by the map's bucket count.
    pub index: usize,
    /// Compact key tag stored in the bucket entry.
    pub tag: u32,
}

/// HashFn is a trait that defines the contract for a function that can hash a
/// key into an index and a key id
///
/// @trait
pub trait HashFn<K> {
    /// hash hashes the given key and returns the raw index and the key id
    ///
    /// note: the raw index is the index of the bucket in the map prior to any
    ///       modulo operations
    ///
    /// @param: key - the key to hash
    /// @return: the raw index and the key id
    fn hash(key: &K) -> HashKey;
}

/// Value is a trait that defines the contract for a type that can be used as a
/// value in a map
///
/// @trait
pub trait Value: Copy + Default {
    /// priority returns the priority of the value
    ///
    /// @return: the priority of the value
    fn priority(&self) -> i8;
}

/// EvictionPolicy chooses which occupied entry should be evicted when a
/// bucket is full.
pub trait EvictionPolicy<V> {
    /// priority returns the eviction priority for the value. Lower priority
    /// entries are evicted before higher priority entries.
    fn priority(value: &V) -> i16;
}

/// ValuePriority is the default eviction policy that delegates priority to
/// the stored value.
#[derive(Debug, Clone, Copy, Default)]
pub struct ValuePriority;

impl<V: Value> EvictionPolicy<V> for ValuePriority {
    #[inline]
    fn priority(value: &V) -> i16 {
        value.priority() as i16
    }
}

const MB_TO_BYTES: usize = 1024 * 1024;

/// Map is a generic hash map implementation that uses maximum length bucket-
/// chaining and a priority-based eviction policy
pub struct Map<K, V: Value, Hasher: HashFn<K>, Policy: EvictionPolicy<V> = ValuePriority> {
    /// number of entries in the map
    len: usize,
    /// maximum number of entries in the map
    capacity: usize,
    /// amount of memory allocated in MBs
    memory_size: usize,
    /// buckets of the map
    buckets: Vec<Bucket<V, Policy>>,

    _key: PhantomData<K>,
    _hasher: PhantomData<Hasher>,
    _policy: PhantomData<Policy>,
}

impl<K, V: Value, Hasher: HashFn<K>, Policy: EvictionPolicy<V>> Map<K, V, Hasher, Policy> {
    /// new creates a new map with the requested memory size
    ///
    /// @param: memory_size - the size of the map in MBs
    /// @return: a new map
    pub fn new(memory_size: usize) -> Self {
        let (bucket_count, capacity) = Self::calculate_sizes(memory_size);
        let buckets = if bucket_count == 0 {
            Vec::new()
        } else {
            vec![Bucket::<V, Policy>::new(); bucket_count]
        };

        Self {
            len: 0,
            capacity,
            memory_size,
            buckets,
            _key: PhantomData,
            _hasher: PhantomData,
            _policy: PhantomData,
        }
    }

    /// set sets an entry in the map
    ///
    /// @param: key - the key of the entry to set
    /// @param: data - the data to insert
    /// @return: void
    /// @side-effects: modifies the transposition table
    #[inline]
    pub fn set(&mut self, key: &K, data: V) {
        let hash_key = Hasher::hash(key);
        let Some(bucket_idx) = self.bucket_index(hash_key.index) else {
            return;
        };

        if self.buckets[bucket_idx].set(hash_key.tag, data) == SetResult::Inserted {
            self.len += 1;
        }
    }

    /// get gets an entry from the map with the given key
    ///
    /// @param: key - the key of the entry to get
    /// @return: the data if the entry is found, None otherwise
    #[inline]
    pub fn get(&self, key: &K) -> Option<&V> {
        let hash_key = Hasher::hash(key);
        let bucket_idx = self.bucket_index(hash_key.index)?;
        self.buckets[bucket_idx].get(hash_key.tag)
    }

    /// is_enabled checks if the map is enabled
    ///
    /// @return: true if the map is enabled, false otherwise
    #[inline]
    pub fn is_enabled(&self) -> bool {
        !self.buckets.is_empty()
    }

    /// capacity returns the maximum number of entries in the map
    ///
    /// @return: maximum number of entries in the map
    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// len returns the number of occupied entries in the map.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// is_empty checks if the map contains no occupied entries.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// resize resizes the map's underlying memory allocation to the requested
    /// size
    ///
    /// @param: memory_size - the new size of the map in MBs
    /// @return: void
    /// @side-effects: clears the map
    #[inline]
    pub fn resize(&mut self, memory_size: usize) {
        // if the memory size is unchanged, just clear the map
        if self.memory_size == memory_size {
            self.clear();
            return;
        }

        // reassign the map to a new one with the requested memory size
        *self = Self::new(memory_size);
    }

    /// clear clears the map
    ///
    /// @return: void
    /// @side-effects: clears the map
    #[inline]
    pub fn clear(&mut self) {
        for entry in self.buckets.iter_mut() {
            entry.clear();
        }
        self.len = 0;
    }

    /// buckets returns the number of buckets allocated in the map
    ///
    /// @return: number of buckets allocated in the map
    #[inline]
    pub fn buckets(&self) -> usize {
        self.buckets.len()
    }

    /// usage is a function to calculate the usage of the map as a ratio between
    /// 0 and the given base
    ///
    /// @param: base - the base to use for the calculation
    /// @return: usage ratio of the map
    #[inline]
    pub fn usage(&self, base: u16) -> u16 {
        if !self.is_enabled() || self.capacity == 0 {
            return 0;
        }

        (self.len * base as usize / self.capacity) as u16
    }

    /// calculate_sizes calculates the number of entries and buckets that can fit
    /// into the requested amount of memory
    ///
    /// @param: memory_size - the amount of memory in MBs
    /// @return: number of buckets and capacity that fit in memory_size
    #[inline]
    const fn calculate_sizes(memory_size: usize) -> (usize, usize) {
        let buckets_per_mb = MB_TO_BYTES / Bucket::<V, Policy>::size_of_mem();
        let buckets = buckets_per_mb.saturating_mul(memory_size);
        let capacity = buckets.saturating_mul(Bucket::<V, Policy>::capacity());

        (buckets, capacity)
    }

    #[inline]
    fn bucket_index(&self, index: usize) -> Option<usize> {
        if self.buckets.is_empty() {
            return None;
        }

        Some(index % self.buckets.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
    struct TestValue(i8);

    impl Value for TestValue {
        fn priority(&self) -> i8 {
            self.0
        }
    }

    struct TestHasher;

    impl HashFn<u64> for TestHasher {
        fn hash(key: &u64) -> HashKey {
            HashKey {
                index: (*key >> 32) as usize,
                tag: *key as u32,
            }
        }
    }

    #[test]
    fn disabled_map_ignores_set_and_get() {
        let mut map = Map::<u64, TestValue, TestHasher>::new(0);

        map.set(&0, TestValue(1));

        assert!(!map.is_enabled());
        assert!(map.is_empty());
        assert_eq!(map.capacity(), 0);
        assert_eq!(map.get(&0), None);
        assert_eq!(map.usage(1000), 0);
    }

    #[test]
    fn get_does_not_return_default_value_for_zero_tag() {
        let map = Map::<u64, TestValue, TestHasher>::new(1);

        assert_eq!(map.get(&0), None);
    }

    #[test]
    fn len_tracks_insert_update_and_eviction() {
        let mut map = Map::<u64, TestValue, TestHasher>::new(1);

        map.set(&1, TestValue(1));
        map.set(&2, TestValue(2));
        map.set(&3, TestValue(3));
        assert_eq!(map.len(), 3);

        map.set(&1, TestValue(9));
        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&1), Some(&TestValue(9)));

        map.set(&4, TestValue(4));
        assert_eq!(map.len(), 3);
    }
}
