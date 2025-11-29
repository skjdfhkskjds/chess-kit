use crate::primitives::Side;
use std::fmt;

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Side::White => write!(f, "White"),
            Side::Black => write!(f, "Black"),
        }
    }
}
