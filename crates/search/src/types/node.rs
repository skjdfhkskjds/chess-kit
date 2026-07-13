use chess_kit_primitives::Depth;

#[derive(Copy, Clone)]
pub enum Bound {
    Exact,
    Lower,
    Upper,
}

#[derive(Copy, Clone)]
pub struct SearchNode {
    pub depth: Depth
}
