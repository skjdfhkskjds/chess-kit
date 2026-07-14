use chess_kit_collections::Value;
use chess_kit_eval::Score;
use chess_kit_primitives::{Depth, Move};
use chess_kit_transposition::NodeData;

/// Bound describes how a cached score relates to the alpha-beta window that
/// produced it
///
/// @type
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum Bound {
    #[default]
    Exact,
    Lower,
    Upper,
}

/// SearchNode is the search information cached for one position
///
/// @type
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct SearchNode {
    depth: Depth,
    score: Score,
    bound: Bound,
    best_move: Option<Move>,
}

impl SearchNode {
    /// new creates a transposition-table entry for a searched position
    ///
    /// @param: depth - remaining search depth represented by the entry
    /// @param: score - score produced by the search
    /// @param: bound - relationship between the score and its alpha-beta window
    /// @param: best_move - best move found while producing the entry
    /// @return: new search node
    pub const fn new(depth: Depth, score: Score, bound: Bound, best_move: Option<Move>) -> Self {
        Self {
            depth,
            score,
            bound,
            best_move,
        }
    }

    /// depth returns the remaining search depth represented by this entry
    ///
    /// @return: remaining search depth represented by the entry
    #[inline]
    pub const fn depth(&self) -> Depth {
        self.depth
    }

    /// score returns the cached score
    ///
    /// @return: cached score
    #[inline]
    pub const fn score(&self) -> Score {
        self.score
    }

    /// bound returns how the score relates to its original alpha-beta window
    ///
    /// @return: bound associated with the cached score
    #[inline]
    pub const fn bound(&self) -> Bound {
        self.bound
    }

    /// best_move returns the best move found while producing this entry
    ///
    /// @return: cached best move, or None if no move was found
    #[inline]
    pub const fn best_move(&self) -> Option<Move> {
        self.best_move
    }
}

impl NodeData for SearchNode {
    /// empty creates a new search node with no data
    ///
    /// @impl: NodeData::empty
    #[inline]
    fn empty() -> Self {
        Self::default()
    }

    /// depth returns the remaining search depth represented by this entry
    ///
    /// @impl: NodeData::depth
    #[inline]
    fn depth(&self) -> i8 {
        self.depth
    }
}

impl Value for SearchNode {
    /// priority returns the replacement priority of this entry
    ///
    /// @impl: Value::priority
    #[inline]
    fn priority(&self) -> i8 {
        self.depth
    }
}
