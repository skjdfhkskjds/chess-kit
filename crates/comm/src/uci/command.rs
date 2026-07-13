use std::str::FromStr;

use super::types::{ParseError, PositionCommand, SearchLimits};

/// `Command` is an enum that represents a GUI-to-engine UCI command understood
/// by the protocol loop
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    Uci,
    Debug(bool),
    IsReady,
    UciNewGame,
    Position(PositionCommand),
    Go(SearchLimits),
    Stop,
    PonderHit,
    Quit,
    Unknown,
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // split the input into the command name and its remaining arguments
        let mut tokens = line.split_whitespace();
        let Some(command) = tokens.next() else {
            return Ok(Self::Unknown);
        };

        // dispatch commands with structured arguments to their type-specific
        // parsers and handle argument-free commands directly
        match command {
            "uci" => Ok(Self::Uci),
            "debug" => match tokens.next() {
                Some("on") => Ok(Self::Debug(true)),
                Some("off") => Ok(Self::Debug(false)),
                _ => Err(ParseError::InvalidArgument("debug")),
            },
            "isready" => Ok(Self::IsReady),
            "ucinewgame" => Ok(Self::UciNewGame),
            "position" => PositionCommand::from_tokens(tokens).map(Self::Position),
            "go" => SearchLimits::from_tokens(tokens).map(Self::Go),
            "stop" => Ok(Self::Stop),
            "ponderhit" => Ok(Self::PonderHit),
            "quit" => Ok(Self::Quit),
            _ => Ok(Self::Unknown),
        }
    }
}
