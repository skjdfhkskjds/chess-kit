use std::fmt::{self, Display};
use std::str::FromStr;
use std::time::Duration;

/// A move encoded using UCI long algebraic notation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UciMove(String);

impl UciMove {
    /// The UCI null move, used when there is no legal move.
    pub fn null() -> Self {
        Self("0000".to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for UciMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for UciMove {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.to_ascii_lowercase();
        if value == "0000" {
            return Ok(Self(value));
        }

        let bytes = value.as_bytes();
        let valid_square =
            |file: u8, rank: u8| (b'a'..=b'h').contains(&file) && (b'1'..=b'8').contains(&rank);
        let valid = matches!(bytes.len(), 4 | 5)
            && valid_square(bytes[0], bytes[1])
            && valid_square(bytes[2], bytes[3])
            && (bytes.len() == 4 || matches!(bytes[4], b'q' | b'r' | b'b' | b'n'));

        if valid {
            Ok(Self(value))
        } else {
            Err(ParseError::InvalidMove(value))
        }
    }
}

/// The base position supplied by a UCI `position` command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BasePosition {
    StartPos,
    Fen(String),
}

/// A parsed UCI `position` command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PositionCommand {
    pub base: BasePosition,
    pub moves: Vec<UciMove>,
}

/// Search constraints supplied by a UCI `go` command.
///
/// Not every engine must use every constraint immediately. Keeping them in the
/// protocol boundary lets search and time-management implementations grow
/// without changing command parsing.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SearchLimits {
    pub white_time: Option<Duration>,
    pub black_time: Option<Duration>,
    pub white_increment: Option<Duration>,
    pub black_increment: Option<Duration>,
    pub moves_to_go: Option<u32>,
    pub depth: Option<u8>,
    pub nodes: Option<u64>,
    pub move_time: Option<Duration>,
    pub infinite: bool,
}

/// A GUI-to-engine UCI command understood by the protocol loop.
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
        let mut tokens = line.split_whitespace();
        let Some(command) = tokens.next() else {
            return Ok(Self::Unknown);
        };

        match command {
            "uci" => Ok(Self::Uci),
            "debug" => match tokens.next() {
                Some("on") => Ok(Self::Debug(true)),
                Some("off") => Ok(Self::Debug(false)),
                _ => Err(ParseError::InvalidArgument("debug")),
            },
            "isready" => Ok(Self::IsReady),
            "ucinewgame" => Ok(Self::UciNewGame),
            "position" => parse_position(tokens).map(Self::Position),
            "go" => parse_go(tokens).map(Self::Go),
            "stop" => Ok(Self::Stop),
            "ponderhit" => Ok(Self::PonderHit),
            "quit" => Ok(Self::Quit),
            _ => Ok(Self::Unknown),
        }
    }
}

fn parse_position<'a>(
    mut tokens: impl Iterator<Item = &'a str>,
) -> Result<PositionCommand, ParseError> {
    let base = match tokens.next() {
        Some("startpos") => BasePosition::StartPos,
        Some("fen") => {
            let fields: Vec<_> = tokens
                .by_ref()
                .take_while(|token| *token != "moves")
                .collect();
            if fields.is_empty() {
                return Err(ParseError::MissingArgument("fen"));
            }
            let fen = fields.join(" ");
            let moves = parse_moves(tokens)?;
            return Ok(PositionCommand {
                base: BasePosition::Fen(fen),
                moves,
            });
        }
        _ => return Err(ParseError::InvalidArgument("position")),
    };

    let moves = match tokens.next() {
        None => Vec::new(),
        Some("moves") => parse_moves(tokens)?,
        Some(_) => return Err(ParseError::InvalidArgument("position")),
    };

    Ok(PositionCommand { base, moves })
}

fn parse_moves<'a>(tokens: impl Iterator<Item = &'a str>) -> Result<Vec<UciMove>, ParseError> {
    tokens.map(UciMove::from_str).collect()
}

fn parse_go<'a>(mut tokens: impl Iterator<Item = &'a str>) -> Result<SearchLimits, ParseError> {
    let mut limits = SearchLimits::default();
    while let Some(token) = tokens.next() {
        match token {
            "wtime" => limits.white_time = Some(parse_millis(&mut tokens, "wtime")?),
            "btime" => limits.black_time = Some(parse_millis(&mut tokens, "btime")?),
            "winc" => limits.white_increment = Some(parse_millis(&mut tokens, "winc")?),
            "binc" => limits.black_increment = Some(parse_millis(&mut tokens, "binc")?),
            "movetime" => limits.move_time = Some(parse_millis(&mut tokens, "movetime")?),
            "movestogo" => limits.moves_to_go = Some(parse_number(&mut tokens, "movestogo")?),
            "depth" => limits.depth = Some(parse_number(&mut tokens, "depth")?),
            "nodes" => limits.nodes = Some(parse_number(&mut tokens, "nodes")?),
            "infinite" => limits.infinite = true,
            // UCI requires unknown tokens to be ignored. This also leaves room
            // for unsupported constraints such as `mate` and `searchmoves`.
            _ => {}
        }
    }
    Ok(limits)
}

fn parse_millis<'a>(
    tokens: &mut impl Iterator<Item = &'a str>,
    name: &'static str,
) -> Result<Duration, ParseError> {
    Ok(Duration::from_millis(parse_number(tokens, name)?))
}

fn parse_number<'a, T>(
    tokens: &mut impl Iterator<Item = &'a str>,
    name: &'static str,
) -> Result<T, ParseError>
where
    T: FromStr,
{
    tokens
        .next()
        .ok_or(ParseError::MissingArgument(name))?
        .parse()
        .map_err(|_| ParseError::InvalidArgument(name))
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_start_position_with_moves() {
        let command = "position startpos moves e2e4 e7e5"
            .parse::<Command>()
            .unwrap();

        assert_eq!(
            command,
            Command::Position(PositionCommand {
                base: BasePosition::StartPos,
                moves: vec!["e2e4".parse().unwrap(), "e7e5".parse().unwrap()],
            })
        );
    }

    #[test]
    fn parses_fen_position_with_moves() {
        let command = "position fen 8/8/8/8/8/8/4k3/7K w - - 0 1 moves h1g1"
            .parse::<Command>()
            .unwrap();

        assert_eq!(
            command,
            Command::Position(PositionCommand {
                base: BasePosition::Fen("8/8/8/8/8/8/4k3/7K w - - 0 1".to_owned()),
                moves: vec!["h1g1".parse().unwrap()],
            })
        );
    }

    #[test]
    fn parses_clock_and_fixed_search_limits() {
        let Command::Go(limits) =
            "go wtime 1000 btime 2000 winc 10 binc 20 movestogo 30 depth 4 nodes 500 movetime 50"
                .parse::<Command>()
                .unwrap()
        else {
            panic!("expected go command");
        };

        assert_eq!(limits.white_time, Some(Duration::from_millis(1000)));
        assert_eq!(limits.black_time, Some(Duration::from_millis(2000)));
        assert_eq!(limits.white_increment, Some(Duration::from_millis(10)));
        assert_eq!(limits.black_increment, Some(Duration::from_millis(20)));
        assert_eq!(limits.moves_to_go, Some(30));
        assert_eq!(limits.depth, Some(4));
        assert_eq!(limits.nodes, Some(500));
        assert_eq!(limits.move_time, Some(Duration::from_millis(50)));
    }

    #[test]
    fn validates_and_normalizes_uci_moves() {
        assert_eq!("e7e8Q".parse::<UciMove>().unwrap().as_str(), "e7e8q");
        assert!("e2e9".parse::<UciMove>().is_err());
        assert!("e7e8k".parse::<UciMove>().is_err());
    }
}
