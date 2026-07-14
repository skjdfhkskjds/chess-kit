use std::str::FromStr;

use super::{ParseError, UciMove};

/// `BasePosition` is an enum that represents the base position supplied by a UCI
/// `position` command
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BasePosition {
    StartPos,
    Fen(String),
}

/// `PositionCommand` is a type that represents a parsed UCI `position` command
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PositionCommand {
    pub base: BasePosition,  // position from which the move history begins
    pub moves: Vec<UciMove>, // ordered moves applied to the base position
}

impl PositionCommand {
    /// from_tokens parses the arguments following a UCI `position` command
    ///
    /// @param: tokens - iterator over the position command arguments
    /// @return: parsed position command, or a parse error
    pub(in crate::uci) fn from_tokens<'a>(
        mut tokens: impl Iterator<Item = &'a str>,
    ) -> Result<Self, ParseError> {
        // parse the required base position before considering the optional
        // move history
        let base = match tokens.next() {
            Some("startpos") => BasePosition::StartPos,
            Some("fen") => {
                // a FEN contains spaces, so collect every field up to the
                // optional moves delimiter
                let fields: Vec<_> = tokens
                    .by_ref()
                    .take_while(|token| *token != "moves")
                    .collect();
                if fields.is_empty() {
                    return Err(ParseError::MissingArgument("fen"));
                }
                let fen = fields.join(" ");
                let moves = parse_moves(tokens)?;
                return Ok(Self {
                    base: BasePosition::Fen(fen),
                    moves,
                });
            }
            _ => return Err(ParseError::InvalidArgument("position")),
        };

        // startpos commands may end after the base position or continue with a
        // moves delimiter and move history
        let moves = match tokens.next() {
            None => Vec::new(),
            Some("moves") => parse_moves(tokens)?,
            Some(_) => return Err(ParseError::InvalidArgument("position")),
        };

        Ok(Self { base, moves })
    }
}

/// parse_moves parses the remaining tokens as UCI moves
///
/// @param: tokens - iterator over UCI move strings
/// @return: parsed move history, or the first move parse error
fn parse_moves<'a>(tokens: impl Iterator<Item = &'a str>) -> Result<Vec<UciMove>, ParseError> {
    tokens.map(UciMove::from_str).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_start_position_with_moves() {
        let position =
            PositionCommand::from_tokens("startpos moves e2e4 e7e5".split_whitespace()).unwrap();

        assert_eq!(
            position,
            PositionCommand {
                base: BasePosition::StartPos,
                moves: vec!["e2e4".parse().unwrap(), "e7e5".parse().unwrap()],
            }
        );
    }

    #[test]
    fn parses_fen_position_with_moves() {
        let position = PositionCommand::from_tokens(
            "fen 8/8/8/8/8/8/4k3/7K w - - 0 1 moves h1g1".split_whitespace(),
        )
        .unwrap();

        assert_eq!(
            position,
            PositionCommand {
                base: BasePosition::Fen("8/8/8/8/8/8/4k3/7K w - - 0 1".to_owned()),
                moves: vec!["h1g1".parse().unwrap()],
            }
        );
    }
}
