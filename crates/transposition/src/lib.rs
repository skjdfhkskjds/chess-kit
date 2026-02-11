pub mod transposition_table;

pub use transposition_table::DefaultTranspositionTable;

use chess_kit_collections::Value;
use chess_kit_primitives::ZobristKey;

/// `NodeData` is a trait that defines the data accessors for a node in the
/// transposition table
///
/// @trait
pub trait NodeData: Value {
    /// empty creates a new instance of a node with no data
    ///
    /// @return: new instance of a node with no data
    fn empty() -> Self;

    /// depth returns the depth of the node
    ///
    /// @return: depth of the node
    fn depth(&self) -> i8;
}

/// `TranspositionTable` is a trait that defines the interface for a transposition
/// table
///
/// @trait
pub trait TranspositionTable<NodeT: NodeData> {
    /// new creates a new transposition table with the requested memory size
    ///
    /// @param: memory_size - the size of the transposition table in MBs
    /// @return: a new transposition table
    fn new(memory_size: usize) -> Self;

    /// insert inserts an entry into the transposition table
    ///
    /// @param: key - the key of the position to insert
    /// @param: data - the data to insert
    /// @return: void
    /// @side-effects: modifies the transposition table
    fn insert(&mut self, key: ZobristKey, data: NodeT);

    /// probe probes the transposition table for an entry with the given key
    ///
    /// @param: key - the key of the position to probe for
    /// @return: the data if the position is found, None otherwise
    fn probe(&self, key: ZobristKey) -> Option<&NodeT>;

    /// is_enabled checks if the transposition table is enabled
    ///
    /// @return: true if the transposition table is enabled, false otherwise
    fn is_enabled(&self) -> bool;

    /// capacity returns the maximum number of entries in the transposition table
    ///
    /// @return: maximum number of entries in the transposition table
    fn capacity(&self) -> usize;

    /// resize resizes the transposition table's underlying memory allocation to
    /// the requested size, clearing it of all data in the process
    ///
    /// @param: memory_size - the new size of the transposition table in MBs
    /// @return: void
    /// @side-effects: clears the transposition table
    fn resize(&mut self, memory_size: usize);

    /// clear clears the transposition table, removing all data from it
    ///
    /// @return: void
    /// @side-effects: clears the transposition table
    fn clear(&mut self);

    /// usage_permille returns the usage of the transposition table as a value
    /// between 0 and 1000 ('permille')
    ///
    /// @return: permille usage of the transposition table
    fn usage_permille(&self) -> u16;

    /// usage_percent returns the usage of the transposition table as a value
    /// between 0 and 100 ('percent')
    ///
    /// @return: percent usage of the transposition table
    fn usage_percent(&self) -> u16;
}
