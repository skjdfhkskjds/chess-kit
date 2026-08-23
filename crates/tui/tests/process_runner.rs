use std::io::{self, BufRead, Write};
use std::time::{Duration, Instant};

use chess_kit_tui::{
    EngineMessageKind, ProcessRunner, RunnerEvent, SearchRequest, UciCommand, UciRunner,
};

#[test]
fn exchanges_a_complete_session_with_a_child_process() {
    let executable = std::env::current_exe().unwrap();
    let arguments = [
        "--ignored",
        "--exact",
        "fake_uci_engine",
        "--nocapture",
        "--test-threads=1",
    ];
    let mut runner = ProcessRunner::spawn(executable, arguments).unwrap();

    runner.send(&UciCommand::Uci).unwrap();
    let handshake = receive_until(&mut runner, |kind| matches!(kind, EngineMessageKind::UciOk));
    assert!(matches!(handshake, EngineMessageKind::UciOk));

    runner.send(&UciCommand::IsReady).unwrap();
    assert!(matches!(
        receive_until(&mut runner, |kind| matches!(
            kind,
            EngineMessageKind::ReadyOk
        )),
        EngineMessageKind::ReadyOk
    ));

    runner
        .send(&UciCommand::Go(SearchRequest::Depth(4)))
        .unwrap();
    let info = receive_until(&mut runner, |kind| {
        matches!(kind, EngineMessageKind::Info(_))
    });
    let EngineMessageKind::Info(info) = info else {
        panic!("expected search info");
    };
    assert_eq!(info.depth, Some(4));
    assert_eq!(info.principal_variation, ["e2e4", "e7e5"]);

    let best_move = receive_until(&mut runner, |kind| {
        matches!(kind, EngineMessageKind::BestMove { .. })
    });
    assert_eq!(
        best_move,
        EngineMessageKind::BestMove {
            best_move: Some("e2e4".to_owned()),
            ponder: Some("e7e5".to_owned()),
        }
    );

    runner.shutdown().unwrap();
}

#[test]
#[ignore = "spawned by the process runner integration test"]
fn fake_uci_engine() {
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();
    for line in stdin.lock().lines() {
        match line.unwrap().trim() {
            "uci" => {
                writeln!(stdout, "id name Fixture Engine").unwrap();
                writeln!(stdout, "id author chess-kit").unwrap();
                writeln!(
                    stdout,
                    "option name Hash type spin default 16 min 1 max 1024"
                )
                .unwrap();
                writeln!(stdout, "uciok").unwrap();
            }
            "isready" => {
                writeln!(stdout, "readyok").unwrap();
            }
            command if command.starts_with("go ") => {
                writeln!(
                    stdout,
                    "info nodes 42 score cp 12 nps 21000 depth 4 pv e2e4 e7e5"
                )
                .unwrap();
                writeln!(stdout, "bestmove e2e4 ponder e7e5").unwrap();
            }
            "quit" => break,
            _ => {}
        }
        stdout.flush().unwrap();
    }
}

fn receive_until(
    runner: &mut ProcessRunner,
    predicate: impl Fn(&EngineMessageKind) -> bool,
) -> EngineMessageKind {
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        match runner.try_recv().unwrap() {
            Some(RunnerEvent::Message(message)) if predicate(message.kind()) => {
                return message.kind().clone();
            }
            Some(RunnerEvent::Error(error)) => panic!("runner error: {error}"),
            Some(RunnerEvent::Exited(code)) => panic!("engine exited unexpectedly: {code:?}"),
            Some(RunnerEvent::Message(_) | RunnerEvent::StandardError(_)) | None => {
                std::thread::sleep(Duration::from_millis(5));
            }
        }
    }
    panic!("timed out waiting for engine message");
}
