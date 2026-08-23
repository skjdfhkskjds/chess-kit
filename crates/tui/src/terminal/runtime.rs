use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;

use super::event::action;
use crate::{Action, App, ConnectionState, Effect, PieceSet, UciRunner, render_with_piece_set};

/// EVENT_POLL_INTERVAL bounds input and engine-output latency.
const EVENT_POLL_INTERVAL: Duration = Duration::from_millis(50);
/// MAX_ENGINE_EVENTS_PER_FRAME keeps terminal input and rendering responsive.
const MAX_ENGINE_EVENTS_PER_FRAME: usize = 128;

/// STOP_TIMEOUT bounds the wait for bestmove after a stop command.
const STOP_TIMEOUT: Duration = Duration::from_secs(2);

/// HANDSHAKE_TIMEOUT bounds UCI initialization and readiness waits.
const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(5);

/// run_terminal runs an interactive application and restores the terminal.
///
/// @marker: RunnerT - UCI runner implementation
/// @param: app - application state
/// @param: runner - connected UCI engine runner
/// @return: Ok after a requested exit, or an I/O error
/// @side-effects: enters raw alternate-screen mode, reads input, and drives UCI
pub fn run_terminal<RunnerT>(app: &mut App, runner: &mut RunnerT) -> io::Result<()>
where
    RunnerT: UciRunner,
{
    run_terminal_with_piece_set(app, runner, PieceSet::default())
}

/// run_terminal_with_piece_set runs an interactive application with selected piece assets.
///
/// @marker: RunnerT - UCI runner implementation
/// @param: app - application state
/// @param: runner - connected UCI engine runner
/// @param: piece_set - assets used to draw pieces
/// @return: Ok after a requested exit, or an I/O error
/// @side-effects: enters raw alternate-screen mode, reads input, and drives UCI
pub fn run_terminal_with_piece_set<RunnerT>(
    app: &mut App,
    runner: &mut RunnerT,
    piece_set: PieceSet,
) -> io::Result<()>
where
    RunnerT: UciRunner,
{
    let result = ratatui::run(|terminal| run_loop(terminal, app, runner, piece_set));
    let shutdown = runner.shutdown();
    result.and(shutdown)
}

/// run_loop multiplexes rendering, terminal input, and engine events.
///
/// @marker: RunnerT - UCI runner implementation
/// @param: terminal - initialized terminal
/// @param: app - application state
/// @param: runner - connected UCI engine runner
/// @param: piece_set - assets used to draw pieces
/// @return: Ok after a requested exit, or an I/O error
/// @side-effects: draws frames, reads input, and sends UCI commands
fn run_loop<RunnerT>(
    terminal: &mut DefaultTerminal,
    app: &mut App,
    runner: &mut RunnerT,
    piece_set: PieceSet,
) -> io::Result<()>
where
    RunnerT: UciRunner,
{
    if apply_effects(app.update(Action::Connect), app, runner) {
        return Ok(());
    }
    let mut stopping_since = None;
    let mut previous_connection = app.connection();
    let mut connection_since = Instant::now();
    loop {
        let mut saturated = false;
        for index in 0..MAX_ENGINE_EVENTS_PER_FRAME {
            let Some(engine_event) = runner.try_recv()? else {
                break;
            };
            let effects = app.update(Action::Runner(engine_event));
            if apply_effects(effects, app, runner) {
                return Ok(());
            }
            saturated = index + 1 == MAX_ENGINE_EVENTS_PER_FRAME;
        }

        let connection = app.connection();
        if connection != previous_connection {
            previous_connection = connection;
            connection_since = Instant::now();
        }
        if matches!(
            connection,
            ConnectionState::AwaitingUci | ConnectionState::AwaitingReady
        ) && connection_since.elapsed() >= HANDSHAKE_TIMEOUT
        {
            let message = "engine timed out during UCI initialization";
            app.update(Action::TransportFailed(message.to_owned()));
            return Err(io::Error::new(io::ErrorKind::TimedOut, message));
        }

        if connection == ConnectionState::Stopping {
            let started = stopping_since.get_or_insert_with(Instant::now);
            if started.elapsed() >= STOP_TIMEOUT {
                let message = "engine did not return bestmove after stop";
                app.update(Action::TransportFailed(message.to_owned()));
                return Err(io::Error::new(io::ErrorKind::TimedOut, message));
            }
        } else {
            stopping_since = None;
        }

        terminal.draw(|frame| render_with_piece_set(frame, app, piece_set))?;
        let poll_interval = if saturated {
            Duration::ZERO
        } else {
            EVENT_POLL_INTERVAL
        };
        if event::poll(poll_interval)?
            && let Event::Key(key) = event::read()?
            && let Some(action) = action(key, app)
        {
            let effects = app.update(action);
            if apply_effects(effects, app, runner) {
                return Ok(());
            }
        }
    }
}

/// apply_effects sends commands and reports transport failures to the app.
///
/// @marker: RunnerT - UCI runner implementation
/// @param: effects - ordered effects to execute
/// @param: app - application receiving transport failures
/// @param: runner - command destination
/// @return: true when exit was requested
/// @side-effects: writes commands and may update failure state
fn apply_effects<RunnerT>(effects: Vec<Effect>, app: &mut App, runner: &mut RunnerT) -> bool
where
    RunnerT: UciRunner,
{
    for effect in effects {
        match effect {
            Effect::Send(command) => {
                if let Err(error) = runner.send(&command) {
                    app.update(Action::TransportFailed(error.to_string()));
                }
            }
            Effect::Quit => return true,
        }
    }
    false
}
