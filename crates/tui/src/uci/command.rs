use std::fmt::{self, Display};
use std::time::Duration;

/// `UciPosition` identifies a UCI position base and its applied moves.
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UciPosition {
    StartPos { moves: Vec<String> },
    Fen { fen: String, moves: Vec<String> },
}

impl UciPosition {
    /// startpos creates a position rooted at the standard starting position.
    ///
    /// @param: moves - ordered UCI moves applied after the starting position
    /// @return: starting-position command value
    pub fn startpos(moves: Vec<String>) -> Self {
        Self::StartPos { moves }
    }

    /// fen creates a position rooted at a Forsyth-Edwards Notation value.
    ///
    /// @param: fen - complete FEN position
    /// @param: moves - ordered UCI moves applied after the FEN position
    /// @return: FEN-position command value
    pub fn fen(fen: impl Into<String>, moves: Vec<String>) -> Self {
        Self::Fen {
            fen: fen.into(),
            moves,
        }
    }
}

/// `SearchRequest` describes the supported first-pass UCI search limits.
///
/// @type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchRequest {
    Infinite,
    Depth(u16),
    MoveTime(Duration),
}

/// `UciCommand` is a typed GUI-to-engine UCI command.
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UciCommand {
    Uci,
    IsReady,
    SetOption { name: String, value: Option<String> },
    UciNewGame,
    Position(UciPosition),
    Go(SearchRequest),
    Stop,
    Quit,
}

impl Display for UciCommand {
    /// fmt serializes one newline-free UCI command.
    ///
    /// @impl: Display::fmt
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uci => formatter.write_str("uci"),
            Self::IsReady => formatter.write_str("isready"),
            Self::SetOption { name, value } => {
                write!(formatter, "setoption name {}", sanitize(name))?;
                if let Some(value) = value {
                    write!(formatter, " value {}", sanitize(value))?;
                }
                Ok(())
            }
            Self::UciNewGame => formatter.write_str("ucinewgame"),
            Self::Position(position) => write_position(formatter, position),
            Self::Go(request) => match request {
                SearchRequest::Infinite => formatter.write_str("go infinite"),
                SearchRequest::Depth(depth) => write!(formatter, "go depth {depth}"),
                SearchRequest::MoveTime(duration) => {
                    // UCI movetime is an integer millisecond value. Preserve a
                    // positive sub-millisecond request as the minimum search
                    // time instead of serializing it as an accidental zero.
                    let milliseconds = duration.as_millis().max(1);
                    write!(formatter, "go movetime {milliseconds}")
                }
            },
            Self::Stop => formatter.write_str("stop"),
            Self::Quit => formatter.write_str("quit"),
        }
    }
}

/// write_position serializes a UCI position command.
///
/// @param: formatter - destination formatter
/// @param: position - position base and move history
/// @return: formatting result
fn write_position(formatter: &mut fmt::Formatter<'_>, position: &UciPosition) -> fmt::Result {
    let moves = match position {
        UciPosition::StartPos { moves } => {
            formatter.write_str("position startpos")?;
            moves
        }
        UciPosition::Fen { fen, moves } => {
            write!(formatter, "position fen {}", sanitize(fen))?;
            moves
        }
    };

    if !moves.is_empty() {
        formatter.write_str(" moves")?;
        for chess_move in moves {
            write!(formatter, " {}", sanitize(chess_move))?;
        }
    }
    Ok(())
}

/// sanitize keeps user-provided command fragments on one protocol line.
///
/// @param: value - command fragment to sanitize
/// @return: whitespace-normalized command fragment
fn sanitize(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}
