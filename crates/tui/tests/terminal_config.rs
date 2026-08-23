use std::ffi::OsStr;
use std::path::Path;

use chess_kit_tui::{ConfigError, GameMode, PieceSet, TerminalConfig};

#[test]
fn defaults_to_the_workspace_debug_engine() {
    let config = TerminalConfig::from_args(Vec::<String>::new()).unwrap();

    assert_eq!(
        config.program(),
        Path::new("target")
            .join("debug")
            .join(format!("chess-kit{}", std::env::consts::EXE_SUFFIX))
    );
    assert!(config.arguments().is_empty());
    assert_eq!(config.piece_set(), PieceSet::Ascii);
    assert_eq!(config.mode(), GameMode::PlayVsEngine);
}

#[test]
fn selects_analysis_mode_without_forwarding_it_to_the_engine() {
    let config =
        TerminalConfig::from_args(["--mode", "analysis", "--engine", "/usr/bin/stockfish"])
            .unwrap();

    assert_eq!(config.mode(), GameMode::Analysis);
    assert!(config.arguments().is_empty());

    let play = TerminalConfig::from_args(["--mode", "play"]).unwrap();
    assert_eq!(play.mode(), GameMode::PlayVsEngine);
}

#[test]
fn selects_piece_assets_without_forwarding_them_to_the_engine() {
    let config = TerminalConfig::from_args([
        "--pieces",
        "ascii",
        "--engine",
        "/usr/bin/stockfish",
        "--",
        "--threads",
        "2",
    ])
    .unwrap();

    assert_eq!(config.piece_set(), PieceSet::Ascii);
    assert_eq!(
        config.arguments(),
        [OsStr::new("--threads"), OsStr::new("2")]
    );
}

#[test]
fn accepts_flag_and_positional_engine_paths() {
    let flagged = TerminalConfig::from_args(["--engine", "/usr/bin/stockfish"]).unwrap();
    assert_eq!(flagged.program(), Path::new("/usr/bin/stockfish"));

    let positional = TerminalConfig::from_args(["./engine"]).unwrap();
    assert_eq!(positional.program(), Path::new("./engine"));
}

#[test]
fn forwards_arguments_after_the_separator() {
    let config =
        TerminalConfig::from_args(["engine", "--", "--threads", "2", "network.nnue"]).unwrap();

    assert_eq!(
        config.arguments(),
        [
            OsStr::new("--threads"),
            OsStr::new("2"),
            OsStr::new("network.nnue"),
        ]
    );
}

#[test]
fn reports_help_and_invalid_arguments() {
    let help = TerminalConfig::from_args(["--help"]).unwrap_err();
    assert_eq!(help, ConfigError::Help);
    assert!(help.to_string().contains("--pieces <set>"));
    assert!(help.to_string().contains("--mode <play|analysis>"));
    assert!(help.to_string().contains("Modes: play (default), analysis"));
    assert!(help.to_string().contains("Piece sets: ascii"));
    assert_eq!(
        TerminalConfig::from_args(["--engine"]).unwrap_err(),
        ConfigError::MissingEnginePath
    );
    assert_eq!(
        TerminalConfig::from_args(["--pieces"]).unwrap_err(),
        ConfigError::MissingPieceSet
    );
    assert_eq!(
        TerminalConfig::from_args(["--mode"]).unwrap_err(),
        ConfigError::MissingMode
    );
    assert_eq!(
        TerminalConfig::from_args(["--pieces", "unknown"]).unwrap_err(),
        ConfigError::InvalidPieceSet("unknown".to_owned())
    );
    assert_eq!(
        TerminalConfig::from_args(["--pieces", "ascii", "--pieces", "ascii"]).unwrap_err(),
        ConfigError::DuplicatePieceSet
    );
    assert_eq!(
        TerminalConfig::from_args(["--mode", "play", "--mode", "analysis"]).unwrap_err(),
        ConfigError::DuplicateMode
    );
    assert_eq!(
        TerminalConfig::from_args(["--mode", "spectate"]).unwrap_err(),
        ConfigError::InvalidMode("spectate".to_owned())
    );
    assert_eq!(
        TerminalConfig::from_args(["one", "two"]).unwrap_err(),
        ConfigError::DuplicateEnginePath
    );
    assert_eq!(
        TerminalConfig::from_args(["--unknown"]).unwrap_err(),
        ConfigError::UnexpectedArgument("--unknown".to_owned())
    );
}
