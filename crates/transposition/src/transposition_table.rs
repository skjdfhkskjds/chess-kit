use crate::{NodeData, TranspositionTable};
use chess_kit_collections::{HashFn, HashKey, Map};
use chess_kit_primitives::ZobristKey;

struct ZobristKeyHashFn;

impl HashFn<ZobristKey> for ZobristKeyHashFn {
    #[inline]
    fn hash(key: &ZobristKey) -> HashKey {
        // split the zobrist key into an index and a key
        //
        // index: right-shifted by 32 bits and then truncated
        // key:   truncated to the least-significant 32 bits
        //
        // Note: this is an optimization that tightly enforces that the zobrist
        //       key is an alias/wrapper around a u64 and relies on the fact that
        //       the `as` cast truncates the superfluous upper bits
        // TODO: make a choice as to whether or not that invariant is reasonable
        let index = u32::from(*key >> 32u64) as usize;
        let tag = u32::from(*key);
        HashKey { index, tag }
    }
}

/// `DefaultTranspositionTable` is the default implementation of the
/// `TranspositionTable` trait
pub struct DefaultTranspositionTable<NodeT: NodeData> {
    map: Map<ZobristKey, NodeT, ZobristKeyHashFn>,
}

impl<NodeT: NodeData> TranspositionTable<NodeT> for DefaultTranspositionTable<NodeT> {
    /// new creates a new transposition table with the requested memory size
    ///
    /// @impl: TranspositionTable::new
    fn new(memory_size: usize) -> Self {
        Self {
            map: Map::new(memory_size),
        }
    }

    /// insert inserts an entry into the transposition table
    ///
    /// @impl: TranspositionTable::insert
    #[inline]
    fn insert(&mut self, zobrist_key: ZobristKey, data: NodeT) {
        self.map.set(&zobrist_key, data);
    }

    /// probe probes the transposition table for an entry with the given zobrist
    /// key
    ///
    /// @impl: TranspositionTable::probe
    #[inline]
    fn probe(&self, zobrist_key: ZobristKey) -> Option<&NodeT> {
        self.map.get(&zobrist_key)
    }

    /// is_enabled checks if the transposition table is enabled
    ///
    /// @impl: TranspositionTable::is_enabled
    #[inline]
    fn is_enabled(&self) -> bool {
        self.map.is_enabled()
    }

    /// capacity returns the maximum number of entries in the transposition table
    ///
    /// @impl: TranspositionTable::capacity
    #[inline]
    fn capacity(&self) -> usize {
        self.map.capacity()
    }

    /// resize resizes the transposition table's underlying memory allocation to
    /// the requested size
    ///
    /// @impl: TranspositionTable::resize
    fn resize(&mut self, memory_size: usize) {
        self.map.resize(memory_size);
    }

    /// clear clears the transposition table
    ///
    /// @impl: TranspositionTable::clear
    #[inline]
    fn clear(&mut self) {
        self.map.clear();
    }

    /// usage_permille returns the usage of the transposition table as a value
    /// between 0 and 1000 ('permille')
    ///
    /// @impl: TranspositionTable::usage_permille
    #[inline]
    fn usage_permille(&self) -> u16 {
        self.map.usage(1000)
    }

    /// usage_percent returns the usage of the transposition table as a value
    /// between 0 and 100 ('percent')
    ///
    /// @impl: TranspositionTable::usage_percent
    #[inline]
    fn usage_percent(&self) -> u16 {
        self.map.usage(100)
    }
}
