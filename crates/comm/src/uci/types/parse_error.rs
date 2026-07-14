use std::fmt::{self, Display};

/// `ParseError` is an enum that represents errors encountered while parsing a
/// known UCI command or move
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    MissingArgument(&'static str),
    InvalidArgument(&'static str),
    InvalidMove(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingArgument(name) => write!(f, "missing value for {name}"),
            Self::InvalidArgument(name) => write!(f, "invalid value for {name}"),
            Self::InvalidMove(mv) => write!(f, "invalid UCI move: {mv}"),
        }
    }
}

impl std::error::Error for ParseError {}
