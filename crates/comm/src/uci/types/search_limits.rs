use std::str::FromStr;
use std::time::Duration;

use chess_kit_primitives::SearchDepth;

use super::ParseError;

/// `SearchLimits` is a type that represents search constraints supplied by a UCI
/// `go` command
///
/// Not every engine must use every constraint immediately. Keeping them in the
/// protocol boundary lets search and time-management implementations grow
/// without changing command parsing
///
/// @type
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SearchLimits {
    pub white_time: Option<Duration>,      // remaining white clock time
    pub black_time: Option<Duration>,      // remaining black clock time
    pub white_increment: Option<Duration>, // white increment per move
    pub black_increment: Option<Duration>, // black increment per move
    pub moves_to_go: Option<u32>,          // moves until the next time control
    pub depth: Option<SearchDepth>,        // maximum positive search depth in plies
    pub nodes: Option<u64>,                // maximum number of nodes to search
    pub move_time: Option<Duration>,       // fixed time allocated to this move
    pub infinite: bool,                    // whether search should continue until stopped
}

impl SearchLimits {
    /// from_tokens parses the arguments following a UCI `go` command
    ///
    /// @param: tokens - iterator over the search limit arguments
    /// @return: parsed search limits, or a parse error
    pub(in crate::uci) fn from_tokens<'a>(
        mut tokens: impl Iterator<Item = &'a str>,
    ) -> Result<Self, ParseError> {
        let mut limits = Self::default();

        // consume recognized constraints and their values until all command
        // arguments have been examined
        while let Some(token) = tokens.next() {
            match token {
                "wtime" => limits.white_time = Some(parse_millis(&mut tokens, "wtime")?),
                "btime" => limits.black_time = Some(parse_millis(&mut tokens, "btime")?),
                "winc" => limits.white_increment = Some(parse_millis(&mut tokens, "winc")?),
                "binc" => limits.black_increment = Some(parse_millis(&mut tokens, "binc")?),
                "movetime" => limits.move_time = Some(parse_millis(&mut tokens, "movetime")?),
                "movestogo" => limits.moves_to_go = Some(parse_number(&mut tokens, "movestogo")?),
                "depth" => limits.depth = Some(parse_depth(&mut tokens)?),
                "nodes" => limits.nodes = Some(parse_number(&mut tokens, "nodes")?),
                "infinite" => limits.infinite = true,
                // UCI requires unknown tokens to be ignored; this also leaves
                // room for unsupported constraints such as `mate` and
                // `searchmoves`
                _ => {}
            }
        }
        Ok(limits)
    }
}

/// parse_depth parses a positive search depth
///
/// @param: tokens - iterator positioned before the depth value
/// @return: parsed positive search depth, or a parse error
fn parse_depth<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<SearchDepth, ParseError> {
    let depth = parse_number(tokens, "depth")?;
    SearchDepth::new(depth).map_err(|_| ParseError::InvalidArgument("depth"))
}

/// parse_millis parses the next argument as a millisecond duration
///
/// @param: tokens - iterator positioned before the duration value
/// @param: name - constraint name used to identify parse errors
/// @return: parsed duration, or a parse error
fn parse_millis<'a>(
    tokens: &mut impl Iterator<Item = &'a str>,
    name: &'static str,
) -> Result<Duration, ParseError> {
    Ok(Duration::from_millis(parse_number(tokens, name)?))
}

/// parse_number parses the next argument as the requested numeric type
///
/// @marker: T - numeric output type
/// @param: tokens - iterator positioned before the numeric value
/// @param: name - constraint name used to identify parse errors
/// @return: parsed number, or a parse error
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_clock_and_fixed_search_limits() {
        let limits = SearchLimits::from_tokens(
            "wtime 1000 btime 2000 winc 10 binc 20 movestogo 30 depth 4 nodes 500 movetime 50"
                .split_whitespace(),
        )
        .unwrap();

        assert_eq!(limits.white_time, Some(Duration::from_millis(1000)));
        assert_eq!(limits.black_time, Some(Duration::from_millis(2000)));
        assert_eq!(limits.white_increment, Some(Duration::from_millis(10)));
        assert_eq!(limits.black_increment, Some(Duration::from_millis(20)));
        assert_eq!(limits.moves_to_go, Some(30));
        assert_eq!(limits.depth.map(SearchDepth::get), Some(4));
        assert_eq!(limits.nodes, Some(500));
        assert_eq!(limits.move_time, Some(Duration::from_millis(50)));
    }

    #[test]
    fn rejects_non_positive_search_depths() {
        assert!(SearchLimits::from_tokens("depth 0".split_whitespace()).is_err());
        assert!(SearchLimits::from_tokens("depth -1".split_whitespace()).is_err());
    }
}
