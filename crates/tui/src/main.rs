//! `chess-kit-tui` binary composition root.

use std::error::Error;

use chess_kit_engine::{DefaultEngine, EngineConfig};
use chess_kit_tui::{App, ProcessRunner, TerminalConfig, run_terminal_with_piece_set};

/// LOCAL_TRANSPOSITION_TABLE_SIZE_MB is the local rules session cache size.
const LOCAL_TRANSPOSITION_TABLE_SIZE_MB: usize = 16;

fn main() {
    let config = match TerminalConfig::from_args(std::env::args_os().skip(1)) {
        Ok(config) => config,
        Err(error) if error.is_help() => {
            println!("{error}");
            return;
        }
        Err(error) => {
            eprintln!("chess-kit-tui: {error}");
            std::process::exit(2);
        }
    };
    if let Err(error) = run(&config) {
        eprintln!("chess-kit-tui: {error}");
        std::process::exit(1);
    }
}

/// run composes the local rules session, UCI runner, and terminal application.
///
/// @param: config - engine process configuration
/// @return: Ok after normal exit, or an application error
/// @side-effects: starts an engine process and enters terminal raw mode
fn run(config: &TerminalConfig) -> Result<(), Box<dyn Error>> {
    let local_engine = DefaultEngine::new(EngineConfig::new(LOCAL_TRANSPOSITION_TABLE_SIZE_MB))?;
    let mut runner = ProcessRunner::spawn(config.program(), config.arguments())?;
    let mut app = App::with_mode(Box::new(local_engine), config.mode());
    run_terminal_with_piece_set(&mut app, &mut runner, config.piece_set())?;
    Ok(())
}
