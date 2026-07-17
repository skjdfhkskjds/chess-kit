use std::fmt::{self, Display};

/// `EngineError` is the fallible error type returned by engine session operations
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EngineError(String);

impl EngineError {
    /// new creates an engine error from the given message
    ///
    /// @param: message - human-readable error description
    /// @return: new engine error
    pub fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for EngineError {}

impl From<String> for EngineError {
    fn from(message: String) -> Self {
        Self(message)
    }
}

impl From<&str> for EngineError {
    fn from(message: &str) -> Self {
        Self(message.to_owned())
    }
}
