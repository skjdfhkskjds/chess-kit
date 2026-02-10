use chess_kit_transposition::NodeData;
use chess_kit_collections::Value;
use std::fmt::{self, Display};

pub type Depth = i8;
pub type NodeCount = usize;

#[derive(Debug, Clone, Copy)]
pub struct PerftData(Depth, NodeCount);

impl PerftData {
    // new creates a new perft data
    //
    // @param: depth - depth of the perft data
    // @param: node_count - number of nodes at the given depth
    // @return: new perft data
    #[inline(always)]
    pub fn new(depth: Depth, node_count: NodeCount) -> Self {
        Self(depth, node_count)
    }

    // depth returns the depth of the perft data
    //
    // @return: depth of the perft data
    // TODO: method overloading via traits is bad style i think
    #[inline(always)]
    pub fn depth(&self) -> Depth {
        self.0
    }

    // node_count returns the number of nodes at the given depth
    //
    // @return: number of nodes at the given depth
    #[inline(always)]
    pub fn node_count(&self) -> NodeCount {
        self.1
    }
}

impl NodeData for PerftData {
    // empty creates a new instance of a node with no data
    //
    // @return: new instance of a node with no data
    #[inline(always)]
    fn empty() -> Self {
        Self(0, 0)
    }

    // depth returns the depth of the node
    //
    // @return: depth of the node
    #[inline(always)]
    fn depth(&self) -> i8 {
        self.0
    }
}

impl Value for PerftData {
    // priority returns the priority of the value
    //
    // @return: the priority of the value
    #[inline(always)]
    fn priority(&self) -> i8 {
        self.0
    }
}

impl Default for PerftData {
    // default creates a new instance of a node with no data
    //
    // @return: new instance of a node with no data
    #[inline(always)]
    fn default() -> Self {
        Self::empty()
    }
}

impl Display for PerftData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.depth(), self.node_count())
    }
}
