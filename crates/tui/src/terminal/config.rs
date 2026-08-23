use std::error::Error;
use std::ffi::OsString;
use std::fmt::{self, Display};
use std::path::{Path, PathBuf};

use crate::PieceSet;

/// USAGE describes the supported first-pass command-line interface.
pub const USAGE: &str = "Usage: chess-kit-tui [--engine <path> | <path>] [--pieces <set>] \
                         [-- <engine arguments...>]";

/// `TerminalConfig` describes the local UCI process to launch.
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TerminalConfig {
    program: PathBuf,
    arguments: Vec<OsString>,
    piece_set: PieceSet,
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
        let mut piece_set = None;
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
                Some("--pieces") => {
                    let value = arguments
                        .get(index + 1)
                        .ok_or(ConfigError::MissingPieceSet)?;
                    let name = value.to_string_lossy();
                    let parsed = name
                        .parse()
                        .map_err(|_| ConfigError::InvalidPieceSet(name.to_string()))?;
                    if piece_set.replace(parsed).is_some() {
                        return Err(ConfigError::DuplicatePieceSet);
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
            program: program.unwrap_or_else(default_engine_path),
            arguments: engine_arguments,
            piece_set: piece_set.unwrap_or_default(),
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

    /// piece_set returns the configured terminal piece assets.
    ///
    /// @return: selected piece set
    pub const fn piece_set(&self) -> PieceSet {
        self.piece_set
    }
}

/// default_engine_path returns the platform-specific workspace debug engine.
///
/// @return: default engine executable path
fn default_engine_path() -> PathBuf {
    Path::new("target")
        .join("debug")
        .join(format!("chess-kit{}", std::env::consts::EXE_SUFFIX))
}

/// `ConfigError` describes invalid terminal-client arguments.
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConfigError {
    Help,
    MissingEnginePath,
    MissingPieceSet,
    DuplicateEnginePath,
    DuplicatePieceSet,
    InvalidPieceSet(String),
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
            Self::Help => {}
            Self::MissingEnginePath => formatter.write_str("missing path after --engine\n")?,
            Self::MissingPieceSet => formatter.write_str("missing set name after --pieces\n")?,
            Self::DuplicateEnginePath => {
                formatter.write_str("engine path was provided more than once\n")?;
            }
            Self::DuplicatePieceSet => {
                formatter.write_str("piece set was provided more than once\n")?;
            }
            Self::InvalidPieceSet(name) => writeln!(formatter, "unknown piece set: {name}")?,
            Self::UnexpectedArgument(argument) => {
                writeln!(formatter, "unexpected argument: {argument}")?;
            }
        }
        write!(formatter, "{USAGE}\nPiece sets: {}", PieceSet::NAMES)
    }
}

impl Error for ConfigError {}
