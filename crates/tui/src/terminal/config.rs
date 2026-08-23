use std::error::Error;
use std::ffi::OsString;
use std::fmt::{self, Display};
use std::path::{Path, PathBuf};

/// DEFAULT_ENGINE_PATH is the workspace debug engine used when none is given.
const DEFAULT_ENGINE_PATH: &str = "target/debug/chess-kit";

/// USAGE describes the supported first-pass command-line interface.
pub const USAGE: &str =
    "Usage: chess-kit-tui [--engine <path> | <path>] [-- <engine arguments...>]";

/// `TerminalConfig` describes the local UCI process to launch.
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TerminalConfig {
    program: PathBuf,
    arguments: Vec<OsString>,
}

impl TerminalConfig {
    /// from_args parses terminal-client arguments.
    ///
    /// A positional engine path and `--engine <path>` are equivalent. Values
    /// after `--` are passed to the engine unchanged.
    ///
    /// @marker: ArgsT - argument collection type
    /// @marker: ArgumentT - argument value type
    /// @param: arguments - process arguments excluding the executable name
    /// @return: parsed configuration or usage error
    pub fn from_args<ArgsT, ArgumentT>(arguments: ArgsT) -> Result<Self, ConfigError>
    where
        ArgsT: IntoIterator<Item = ArgumentT>,
        ArgumentT: Into<OsString>,
    {
        let arguments = arguments.into_iter().map(Into::into).collect::<Vec<_>>();
        let mut program = None;
        let mut engine_arguments = Vec::new();
        let mut index = 0;
        while index < arguments.len() {
            match arguments[index].to_str() {
                Some("-h" | "--help") => return Err(ConfigError::Help),
                Some("--engine") => {
                    let value = arguments
                        .get(index + 1)
                        .ok_or(ConfigError::MissingEnginePath)?;
                    if program.replace(PathBuf::from(value)).is_some() {
                        return Err(ConfigError::DuplicateEnginePath);
                    }
                    index += 2;
                }
                Some("--") => {
                    engine_arguments.extend(arguments[index + 1..].iter().cloned());
                    break;
                }
                Some(value) if value.starts_with('-') => {
                    return Err(ConfigError::UnexpectedArgument(value.to_owned()));
                }
                _ => {
                    if program.replace(PathBuf::from(&arguments[index])).is_some() {
                        return Err(ConfigError::DuplicateEnginePath);
                    }
                    index += 1;
                }
            }
        }

        Ok(Self {
            program: program.unwrap_or_else(|| PathBuf::from(DEFAULT_ENGINE_PATH)),
            arguments: engine_arguments,
        })
    }

    /// program returns the configured engine executable.
    ///
    /// @return: engine executable path
    pub fn program(&self) -> &Path {
        &self.program
    }

    /// arguments returns arguments passed to the engine.
    ///
    /// @return: engine process arguments
    pub fn arguments(&self) -> &[OsString] {
        &self.arguments
    }
}

/// `ConfigError` describes invalid terminal-client arguments.
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConfigError {
    Help,
    MissingEnginePath,
    DuplicateEnginePath,
    UnexpectedArgument(String),
}

impl ConfigError {
    /// is_help reports whether normal usage output was requested.
    ///
    /// @return: true for the help sentinel
    pub const fn is_help(&self) -> bool {
        matches!(self, Self::Help)
    }
}

impl Display for ConfigError {
    /// fmt describes the argument error and includes usage.
    ///
    /// @impl: Display::fmt
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Help => formatter.write_str(USAGE),
            Self::MissingEnginePath => {
                write!(formatter, "missing path after --engine\n{USAGE}")
            }
            Self::DuplicateEnginePath => {
                write!(
                    formatter,
                    "engine path was provided more than once\n{USAGE}"
                )
            }
            Self::UnexpectedArgument(argument) => {
                write!(formatter, "unexpected argument: {argument}\n{USAGE}")
            }
        }
    }
}

impl Error for ConfigError {}
