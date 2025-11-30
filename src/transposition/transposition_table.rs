use crate::board::ZobristKey;
use crate::transposition::{NodeData, entry::Entry};

const MB_TO_BYTES: usize = 1024 * 1024;

pub struct TranspositionTable<T: NodeData> {
    memory_size: usize, // amount of memory allocated in MBs
    size: usize,        // number of buckets used in the table
    max_entries: usize, // maximum number of entries in the table
    max_buckets: usize, // maximum number of buckets in the table

    entries: Vec<Entry<T>>,
}

impl<T> TranspositionTable<T>
where
    T: NodeData + Copy + Clone,
{
    // Create a new TT of the requested size, able to hold the data
    // of type D, where D has to implement HashData, and must be cloneable
    // and copyable.
    pub fn new(memory_size: usize) -> Self {
        let (max_entries, max_buckets) = Self::calculate_sizes(memory_size);

        Self {
            memory_size: memory_size,
            size: 0,
            max_entries: max_entries,
            max_buckets: max_buckets,
            entries: vec![Entry::<T>::new(); max_entries],
        }
    }

    // insert inserts an entry into the transposition table
    //
    // @param: zobrist_key - the zobrist key of the position to insert
    // @param: data - the data to insert
    // @return: void
    // @side-effects: modifies the transposition table
    pub fn insert(&mut self, zobrist_key: ZobristKey, data: T) {
        if !self.is_enabled() {
            return;
        }

        let (idx, key) = self.parse_zobrist_key(zobrist_key);
        self.size += self.entries[idx].set(key, data) as usize;
    }

    // probe probes the transposition table for an entry with the given zobrist
    // key
    //
    // @param: zobrist_key - the zobrist key of the position to probe
    // @return: the data if the position is found, None otherwise
    pub fn probe(&self, zobrist_key: ZobristKey) -> Option<&T> {
        if !self.is_enabled() {
            return None;
        }

        let (idx, key) = self.parse_zobrist_key(zobrist_key);
        self.entries[idx].get(key)
    }

    // is_enabled checks if the transposition table is enabled
    //
    // @return: true if the transposition table is enabled, false otherwise
    #[inline(always)]
    pub const fn is_enabled(&self) -> bool {
        self.memory_size > 0
    }

    // max_buckets returns the maximum number of buckets in the transposition table
    //
    // @return: maximum number of buckets in the transposition table
    #[inline(always)]
    pub const fn max_buckets(&self) -> usize {
        self.max_buckets
    }

    // max_entries returns the maximum number of entries in the transposition table
    //
    // @return: maximum number of entries in the transposition table
    #[inline(always)]
    pub const fn max_entries(&self) -> usize {
        self.max_entries
    }

    // resize resizes the transposition table's underlying memory allocation to
    // the requested size
    //
    // @param: memory_size - the new size of the transposition table in MBs
    // @return: void
    // @side-effects: zeroes the transposition table
    pub fn resize(&mut self, memory_size: usize) {
        // if the memory size is unchanged, just clear the table
        if self.memory_size == memory_size {
            self.clear();
            return;
        }

        // reassign the transposition table to a new one with the requested
        // memory size
        *self = Self::new(memory_size);
    }

    // clear clears the transposition table
    //
    // @return: void
    // @side-effects: modifies the transposition table
    #[inline(always)]
    pub fn clear(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.clear();
        }
        self.size = 0;
    }

    // usage_permille returns the usage of the transposition table as a value
    // between 0 and 1000 ('permille')
    //
    // @return: permille usage of the transposition table
    pub fn usage_permille(&self) -> u16 {
        self.usage(1000f64)
    }

    // usage_percent returns the usage of the transposition table as a value
    // between 0 and 100 ('percent')
    //
    // @return: percent usage of the transposition table
    pub fn usage_percent(&self) -> u16 {
        self.usage(100f64)
    }

    // parse_zobrist_key builds the index and key from the zobrist key
    //
    // @param: zobrist_key - the zobrist key to parse
    // @return: the index and key
    #[inline(always)]
    fn parse_zobrist_key(&self, zobrist_key: ZobristKey) -> (usize, u32) {
        // split the zobrist key into an index and a key
        //
        // index: right-shifted by 32 bits and then truncated
        // key:   truncated to the least-significant 32 bits
        //
        // Note: this is an optimization that tightly enforces that the zobrist
        //       key is an alias/wrapper around a u64 and relies on the fact that
        //       the `as` cast truncates the superfluous upper bits
        // TODO: make a choice as to whether or not that invariant is reasonable
        let index = ((zobrist_key >> 32) as u32) as usize % self.max_entries;
        let key = zobrist_key as u32;
        (index, key)
    }

    // usage is a helper routine to calculate the usage of the transposition
    // table as a value between 0 and base
    //
    // @param: base - the base to use for the calculation
    // @return: usage of the transposition table
    #[inline(always)]
    const fn usage(&self, base: f64) -> u16 {
        if !self.is_enabled() {
            return 0;
        }

        let fraction = self.size as f64 / self.max_buckets as f64;
        (fraction * base).floor() as u16
    }

    // calculate_sizes calculates the number of entries and buckets that can fit
    // into the requested amount of memory
    //
    // @param: memory_size - the amount of memory in MBs
    // @return: total number of entries and buckets that fit in memory_size
    #[inline(always)]
    const fn calculate_sizes(memory_size: usize) -> (usize, usize) {
        let entries = MB_TO_BYTES / Entry::<T>::size_of_mem() * memory_size;
        let buckets = entries * Entry::<T>::num_buckets();

        (entries, buckets)
    }
}
