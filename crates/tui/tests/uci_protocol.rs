use std::str::FromStr;
use std::time::Duration;

use chess_kit_tui::{
    EngineMessage, EngineMessageKind, IdentityField, Score, ScoreBound, ScoreValue, SearchRequest,
    UciCommand, UciPosition,
};

#[test]
fn serializes_the_supported_command_set() {
    let commands = [
        (UciCommand::Uci, "uci"),
        (UciCommand::IsReady, "isready"),
        (
            UciCommand::SetOption {
                name: "Hash".to_owned(),
                value: Some("128".to_owned()),
            },
            "setoption name Hash value 128",
        ),
        (UciCommand::UciNewGame, "ucinewgame"),
        (
            UciCommand::Position(UciPosition::startpos(vec![
                "e2e4".to_owned(),
                "e7e5".to_owned(),
            ])),
            "position startpos moves e2e4 e7e5",
        ),
        (
            UciCommand::Position(UciPosition::fen("8/8/8/8/8/8/8/K6k w - - 0 1", Vec::new())),
            "position fen 8/8/8/8/8/8/8/K6k w - - 0 1",
        ),
        (UciCommand::Go(SearchRequest::Depth(12)), "go depth 12"),
        (
            UciCommand::Go(SearchRequest::MoveTime(Duration::from_millis(250))),
            "go movetime 250",
        ),
        (
            UciCommand::Go(SearchRequest::MoveTime(Duration::from_nanos(1))),
            "go movetime 1",
        ),
        (UciCommand::Go(SearchRequest::Infinite), "go infinite"),
        (UciCommand::Stop, "stop"),
        (UciCommand::Quit, "quit"),
    ];

    for (command, expected) in commands {
        assert_eq!(command.to_string(), expected);
    }
}

#[test]
fn parses_identity_options_and_handshake_messages() {
    let name = EngineMessage::from_str("id name Example Engine 1.0\r\n").unwrap();
    assert_eq!(
        name.kind(),
        &EngineMessageKind::Id {
            field: IdentityField::Name,
            value: "Example Engine 1.0".to_owned(),
        }
    );

    let option = EngineMessage::from_str(
        "option name Style Choice type combo default Normal var Solid var Normal var Risky",
    )
    .unwrap();
    let EngineMessageKind::Option(option) = option.kind() else {
        panic!("expected option");
    };
    assert_eq!(option.name, "Style Choice");
    assert_eq!(option.kind, "combo");
    assert_eq!(option.default.as_deref(), Some("Normal"));
    assert_eq!(option.variants, ["Solid", "Normal", "Risky"]);
    assert_eq!(
        EngineMessage::from_str("uciok").unwrap().kind(),
        &EngineMessageKind::UciOk
    );
    assert_eq!(
        EngineMessage::from_str("readyok").unwrap().kind(),
        &EngineMessageKind::ReadyOk
    );
}

#[test]
fn parses_reordered_search_information_and_best_move() {
    let message = EngineMessage::from_str(
        "info nodes 4200 pv e2e4 e7e5 score mate -3 upperbound nps 210000 depth 18 hashfull 42",
    )
    .unwrap();
    let EngineMessageKind::Info(info) = message.kind() else {
        panic!("expected info");
    };
    assert_eq!(info.depth, Some(18));
    assert_eq!(info.nodes, Some(4200));
    assert_eq!(info.nodes_per_second, Some(210000));
    assert_eq!(info.hash_full, Some(42));
    assert_eq!(
        info.score,
        Some(Score {
            value: ScoreValue::Mate(-3),
            bound: Some(ScoreBound::Upper),
        })
    );
    assert_eq!(info.principal_variation, ["e2e4", "e7e5"]);

    let best = EngineMessage::from_str("bestmove g1f3 ponder d8f6").unwrap();
    assert_eq!(
        best.kind(),
        &EngineMessageKind::BestMove {
            best_move: Some("g1f3".to_owned()),
            ponder: Some("d8f6".to_owned()),
        }
    );
    assert_eq!(
        EngineMessage::from_str("bestmove 0000").unwrap().kind(),
        &EngineMessageKind::BestMove {
            best_move: None,
            ponder: None,
        }
    );
}

#[test]
fn preserves_unknown_and_diagnostic_lines() {
    let message = EngineMessage::from_str("custom engine extension").unwrap();
    assert_eq!(message.raw(), "custom engine extension");
    assert_eq!(message.kind(), &EngineMessageKind::Unknown);
}
