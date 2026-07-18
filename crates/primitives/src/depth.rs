use std::error::Error;
use std::fmt::{self, Display};

/// `Depth` is the signed numeric representation used by recursive search and
/// perft code, where zero is a meaningful leaf depth.
pub type Depth = i8;

/// `SearchDepth` is a validated, positive depth requested at an engine boundary.
///
/// Unlike [`Depth`], this type cannot represent the zero-depth leaf searches
/// used internally by negamax and quiescence search.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct SearchDepth(Depth);

impl SearchDepth {
    /// new validates a requested engine search depth.
    pub const fn new(depth: Depth) -> Result<Self, InvalidSearchDepth> {
        if depth > 0 {
            Ok(Self(depth))
        } else {
            Err(InvalidSearchDepth(depth))
        }
    }

    /// get returns the underlying recursive depth value.
    pub const fn get(self) -> Depth {
        self.0
    }
}

impl TryFrom<Depth> for SearchDepth {
    type Error = InvalidSearchDepth;

    fn try_from(depth: Depth) -> Result<Self, Self::Error> {
        Self::new(depth)
    }
}

impl From<SearchDepth> for Depth {
    fn from(depth: SearchDepth) -> Self {
        depth.get()
    }
}

impl Display for SearchDepth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// `InvalidSearchDepth` reports a non-positive engine search request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidSearchDepth(Depth);

impl InvalidSearchDepth {
    /// depth returns the rejected depth value.
    pub const fn depth(self) -> Depth {
        self.0
    }
}

impl Display for InvalidSearchDepth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "search depth must be positive (got {})", self.0)
    }
}

impl Error for InvalidSearchDepth {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_depth_accepts_only_positive_depths() {
        assert_eq!(SearchDepth::new(1).unwrap().get(), 1);
        assert_eq!(SearchDepth::new(127).unwrap().get(), 127);
        assert!(SearchDepth::new(0).is_err());
        assert!(SearchDepth::new(-1).is_err());
    }
}
