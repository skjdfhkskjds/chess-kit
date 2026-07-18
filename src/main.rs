use chess_kit::comm::uci::UciAdapter;
use chess_kit::engine::{DefaultEngine, EngineConfig};
use chess_kit::primitives::SearchDepth;

/// DEFAULT_UCI_SEARCH_DEPTH is the fallback depth for an unconstrained UCI search
const DEFAULT_UCI_SEARCH_DEPTH: SearchDepth = match SearchDepth::new(4) {
    Ok(depth) => depth,
    Err(_) => panic!("default UCI search depth must be positive"),
};

/// UCI_TRANSPOSITION_TABLE_SIZE_MB is the transposition table size selected by
/// the UCI presentation
const UCI_TRANSPOSITION_TABLE_SIZE_MB: usize = 1024;

fn main() {
    if let Err(error) = run() {
        eprintln!("chess-kit: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let engine = DefaultEngine::new(EngineConfig::new(UCI_TRANSPOSITION_TABLE_SIZE_MB))?;
    let mut adapter = UciAdapter::new(engine, DEFAULT_UCI_SEARCH_DEPTH);
    chess_kit::comm::uci::run(&mut adapter)?;
    Ok(())
}
