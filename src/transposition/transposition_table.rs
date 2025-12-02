use crate::primitives::ZobristKey;
use crate::transposition::{NodeData, TranspositionTable, bucket::Bucket};

const MB_TO_BYTES: usize = 1024 * 1024;

pub struct DefaultTranspositionTable<T: NodeData> {
    size: usize,        // number of entries in the table
    capacity: usize,    // maximum number of entries in the table
    max_buckets: usize, // maximum number of buckets in the table
    memory_size: usize, // amount of memory allocated in MBs

    buckets: Vec<Bucket<T>>,
}

impl<T: NodeData> TranspositionTable<T> for DefaultTranspositionTable<T> {
    // new creates a new transposition table with the requested memory size
    //
    // @param: memory_size - the size of the transposition table in MBs
    // @return: a new transposition table
    fn new(memory_size: usize) -> Self {
        let (buckets, capacity) = Self::calculate_sizes(memory_size);

        Self {
            size: 0,
            capacity,
            max_buckets: buckets,
            memory_size: memory_size,
            buckets: vec![Bucket::<T>::new(); buckets],
        }
    }

    // insert inserts an entry into the transposition table
    //
    // @param: zobrist_key - the zobrist key of the position to insert
    // @param: data - the data to insert
    // @return: void
    // @side-effects: modifies the transposition table
    #[inline(always)]
    fn insert(&mut self, zobrist_key: ZobristKey, data: T) {
        if !self.is_enabled() {
            return;
        }

        let (idx, key) = self.parse_zobrist_key(zobrist_key);
        self.size += self.buckets[idx].set(key, data) as usize;
    }

    // probe probes the transposition table for an entry with the given zobrist
    // key
    //
    // @param: zobrist_key - the zobrist key of the position to probe
    // @return: the data if the position is found, None otherwise
    #[inline(always)]
    fn probe(&self, zobrist_key: ZobristKey) -> Option<&T> {
        if !self.is_enabled() {
            return None;
        }

        let (idx, key) = self.parse_zobrist_key(zobrist_key);
        self.buckets[idx].get(key)
    }

    // is_enabled checks if the transposition table is enabled
    //
    // @return: true if the transposition table is enabled, false otherwise
    #[inline(always)]
    fn is_enabled(&self) -> bool {
        self.memory_size > 0
    }

    // capacity returns the maximum number of entries in the transposition table
    //
    // @return: maximum number of entries in the transposition table
    #[inline(always)]
    fn capacity(&self) -> usize {
        self.capacity
    }

    // resize resizes the transposition table's underlying memory allocation to
    // the requested size
    //
    // @param: memory_size - the new size of the transposition table in MBs
    // @return: void
    // @side-effects: clears the transposition table
    fn resize(&mut self, memory_size: usize) {
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
    // @side-effects: clears the transposition table
    #[inline(always)]
    fn clear(&mut self) {
        for entry in self.buckets.iter_mut() {
            entry.clear();
        }
        self.size = 0;
    }

    // usage_permille returns the usage of the transposition table as a value
    // between 0 and 1000 ('permille')
    //
    // @return: permille usage of the transposition table
    #[inline(always)]
    fn usage_permille(&self) -> u16 {
        self.usage(1000f64)
    }

    // usage_percent returns the usage of the transposition table as a value
    // between 0 and 100 ('percent')
    //
    // @return: percent usage of the transposition table
    #[inline(always)]
    fn usage_percent(&self) -> u16 {
        self.usage(100f64)
    }
}

impl<T: NodeData> DefaultTranspositionTable<T> {
    // buckets returns the number of buckets allocated in the transposition table
    //
    // @return: number of buckets allocated in the transposition table
    #[inline(always)]
    pub fn buckets(&self) -> usize {
        self.max_buckets
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
        let index = u32::from(zobrist_key >> 32u64) as usize % self.max_buckets;
        let key = u32::from(zobrist_key);
        (index, key)
    }

    // usage is a helper routine to calculate the usage of the transposition
    // table as a value between 0 and base
    //
    // @param: base - the base to use for the calculation
    // @return: usage of the transposition table
    #[inline(always)]
    fn usage(&self, base: f64) -> u16 {
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
        let buckets = MB_TO_BYTES / Bucket::<T>::size_of_mem() * memory_size;
        let capacity = buckets * Bucket::<T>::capacity();

        (buckets, capacity)
    }
}
