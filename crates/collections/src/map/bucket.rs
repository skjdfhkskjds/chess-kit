use super::{Entry, EvictionPolicy, Value, ValuePriority};
use std::marker::PhantomData;

/// DEFAULT_CAPACITY is the default capacity of a bucket
pub const DEFAULT_CAPACITY: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SetResult {
    Inserted,
    Updated,
    Evicted,
}

pub(crate) struct Bucket<
    T: Value,
    P: EvictionPolicy<T> = ValuePriority,
    const N: usize = DEFAULT_CAPACITY,
> {
    entries: [Option<Entry<T>>; N],
    _policy: PhantomData<P>,
}

impl<T: Value, P: EvictionPolicy<T>, const N: usize> Clone for Bucket<T, P, N> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            entries: self.entries,
            _policy: PhantomData,
        }
    }
}

impl<T: Value, P: EvictionPolicy<T>, const N: usize> Bucket<T, P, N> {
    #[inline]
    pub fn new() -> Self {
        Self {
            entries: [None; N],
            _policy: PhantomData,
        }
    }

    #[inline]
    pub fn get(&self, key: u32) -> Option<&T> {
        for entry in self.entries.iter() {
            let Some(entry) = entry else {
                continue;
            };

            if entry.key() == key {
                return Some(entry.data());
            }
        }

        None
    }

    /// Inserts or updates a key/value pair, evicting the lowest-priority entry
    /// only when the bucket is already full.
    #[inline]
    pub fn set(&mut self, key: u32, data: T) -> SetResult {
        for slot in self.entries.iter_mut() {
            match slot {
                Some(entry) if entry.key() == key => {
                    entry.set(key, data);
                    return SetResult::Updated;
                }
                Some(_) => {}
                None => {
                    // Buckets maintain an occupied prefix, so the key cannot
                    // occur after this slot and no second scan is needed.
                    *slot = Some(Entry::new(key, data));
                    return SetResult::Inserted;
                }
            }
        }

        let mut min_priority = i16::MAX;
        let mut min_priority_idx = 0;
        for (index, entry) in self.entries.iter().enumerate() {
            let entry = entry.expect("full bucket contains only occupied entries");
            let priority = P::priority(entry.data());
            if priority < min_priority {
                min_priority = priority;
                min_priority_idx = index;
            }
        }

        self.entries[min_priority_idx] = Some(Entry::new(key, data));
        SetResult::Evicted
    }

    #[inline]
    pub fn clear(&mut self) {
        for entry in self.entries.iter_mut() {
            *entry = None;
        }
    }

    #[inline]
    pub const fn size_of_mem() -> usize {
        std::mem::size_of::<Option<Entry<T>>>() * N
    }

    #[inline]
    pub const fn capacity() -> usize {
        N
    }
}

impl<T: Value, P: EvictionPolicy<T>, const N: usize> Default for Bucket<T, P, N> {
    #[inline]
    fn default() -> Self {
        Self::new()
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

    #[test]
    fn get_ignores_empty_entries_with_matching_default_key() {
        let bucket = Bucket::<TestValue>::new();

        assert_eq!(bucket.get(0), None);
    }

    #[test]
    fn set_updates_existing_key_before_eviction() {
        let mut bucket = Bucket::<TestValue>::new();

        assert_eq!(bucket.set(1, TestValue(1)), SetResult::Inserted);
        assert_eq!(bucket.set(2, TestValue(2)), SetResult::Inserted);
        assert_eq!(bucket.set(3, TestValue(3)), SetResult::Inserted);
        assert_eq!(bucket.set(1, TestValue(9)), SetResult::Updated);

        assert_eq!(bucket.get(1), Some(&TestValue(9)));
        assert_eq!(bucket.get(2), Some(&TestValue(2)));
        assert_eq!(bucket.get(3), Some(&TestValue(3)));
    }

    #[test]
    fn set_uses_empty_slots_before_comparing_priorities() {
        let mut bucket = Bucket::<TestValue>::new();

        assert_eq!(bucket.set(1, TestValue(9)), SetResult::Inserted);
        assert_eq!(bucket.set(2, TestValue(8)), SetResult::Inserted);

        assert_eq!(bucket.get(1), Some(&TestValue(9)));
        assert_eq!(bucket.get(2), Some(&TestValue(8)));
    }

    #[test]
    fn set_evicts_lowest_priority_when_full() {
        let mut bucket = Bucket::<TestValue>::new();

        bucket.set(1, TestValue(9));
        bucket.set(2, TestValue(1));
        bucket.set(3, TestValue(5));
        assert_eq!(bucket.set(4, TestValue(7)), SetResult::Evicted);

        assert_eq!(bucket.get(1), Some(&TestValue(9)));
        assert_eq!(bucket.get(2), None);
        assert_eq!(bucket.get(3), Some(&TestValue(5)));
        assert_eq!(bucket.get(4), Some(&TestValue(7)));
    }
}
