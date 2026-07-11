use chess_kit_collections::Value;

/// Matches the size and alignment profile of the current perft TT payload.
#[derive(Clone, Copy, Default)]
pub struct NodeValue {
    depth: i8,
    nodes: usize,
}

impl NodeValue {
    #[inline]
    pub fn new(seed: u64) -> Self {
        Self {
            depth: (seed & 0x7f) as i8,
            nodes: seed as usize,
        }
    }

    #[inline]
    pub fn payload(&self) -> usize {
        self.nodes ^ self.depth as usize
    }
}

impl Value for NodeValue {
    #[inline]
    fn priority(&self) -> i8 {
        self.depth
    }
}
