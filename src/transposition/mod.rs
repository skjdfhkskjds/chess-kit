pub mod transposition_table;
pub(crate) mod entry;
pub(crate) mod bucket;

pub use transposition_table::TranspositionTable;

pub trait NodeData {
    // empty creates a new instance of a node with no data
    // 
    // @return: new instance of a node with no data
    fn empty() -> Self;

    // depth returns the depth of the node
    // 
    // @return: depth of the node
    fn depth(&self) -> i8;
}

