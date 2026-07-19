use std::env;

use chess_kit::comm::cli::InteractiveGame;
use chess_kit::engine::{DefaultEngine, EngineConfig};
use chess_kit::primitives::{Depth, SearchDepth};

/// DEFAULT_INTERACTIVE_SEARCH_DEPTH is the search depth used when `--depth` is
/// not supplied
pub const DEFAULT_INTERACTIVE_SEARCH_DEPTH: SearchDepth = match SearchDepth::new(6) {
    Ok(depth) => depth,
    Err(_) => panic!("default interactive search depth must be positive"),
};

/// INTERACTIVE_TRANSPOSITION_TABLE_SIZE_MB is the transposition table size used
/// by the interactive presentation
pub const INTERACTIVE_TRANSPOSITION_TABLE_SIZE_MB: usize = 1024;

const USAGE: &str = "Usage: game [OPTIONS]\n\
\n\
Options:\n\
  -d, --depth <PLIES>  Search depth for engine moves (default: 6)\n\
  -h, --help           Print help";

struct GameOptions {
    depth: SearchDepth,
}

fn parse_options(
    arguments: impl IntoIterator<Item = String>,
) -> Result<Option<GameOptions>, String> {
    let mut arguments = arguments.into_iter();
    let mut depth = None;

    while let Some(argument) = arguments.next() {
        let value = match argument.as_str() {
            "-h" | "--help" => return Ok(None),
            "-d" | "--depth" => Some(
                arguments
                    .next()
                    .ok_or_else(|| format!("{argument} requires a depth"))?,
            ),
            _ => argument.strip_prefix("--depth=").map(str::to_owned),
        };

        let Some(value) = value else {
            return Err(format!("unrecognized argument: {argument}"));
        };
        if depth.is_some() {
            return Err("depth may only be specified once".to_owned());
        }
        depth = Some(parse_depth(&value)?);
    }

    Ok(Some(GameOptions {
        depth: depth.unwrap_or(DEFAULT_INTERACTIVE_SEARCH_DEPTH),
    }))
}

fn parse_depth(value: &str) -> Result<SearchDepth, String> {
    let depth = value
        .parse::<Depth>()
        .map_err(|_| format!("depth must be a positive integer (got {value:?})"))?;
    SearchDepth::new(depth).map_err(|_| format!("depth must be a positive integer (got {value:?})"))
}

fn run() -> Result<(), String> {
    let Some(options) = parse_options(env::args().skip(1))? else {
        println!("{USAGE}");
        return Ok(());
    };

    let engine = DefaultEngine::new(EngineConfig::new(INTERACTIVE_TRANSPOSITION_TABLE_SIZE_MB))
        .map_err(|error| error.to_string())?;
    InteractiveGame::new(engine, options.depth)
        .run()
        .map_err(|error| error.to_string())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("chess-kit game example: {error}");
        eprintln!("\n{USAGE}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_default_and_explicit_search_depths() {
        let default = parse_options(Vec::new()).unwrap().unwrap();
        let explicit = parse_options(["--depth".to_owned(), "7".to_owned()])
            .unwrap()
            .unwrap();
        let equals = parse_options(["--depth=3".to_owned()]).unwrap().unwrap();

        assert_eq!(default.depth.get(), 6);
        assert_eq!(explicit.depth.get(), 7);
        assert_eq!(equals.depth.get(), 3);
    }

    #[test]
    fn rejects_unsupported_search_depths() {
        assert!(parse_options(["--depth".to_owned(), "0".to_owned()]).is_err());
        assert!(parse_options(["--depth=128".to_owned()]).is_err());
        assert!(parse_options(["--depth=nope".to_owned()]).is_err());
    }
}
