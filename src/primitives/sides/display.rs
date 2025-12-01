use crate::primitives::Sides;
use std::fmt;

impl fmt::Display for Sides {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sides::White => write!(f, "White"),
            Sides::Black => write!(f, "Black"),
        }
    }
}
