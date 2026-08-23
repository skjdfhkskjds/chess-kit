use std::io;
use std::time::Duration;

use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;

use super::event::action;
use crate::{Action, App, Effect, UciRunner, render};

/// EVENT_POLL_INTERVAL bounds input and engine-output latency.
const EVENT_POLL_INTERVAL: Duration = Duration::from_millis(50);

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
    let result = ratatui::run(|terminal| run_loop(terminal, app, runner));
    let shutdown = runner.shutdown();
    result.and(shutdown)
}

/// run_loop multiplexes rendering, terminal input, and engine events.
///
/// @marker: RunnerT - UCI runner implementation
/// @param: terminal - initialized terminal
/// @param: app - application state
/// @param: runner - connected UCI engine runner
/// @return: Ok after a requested exit, or an I/O error
/// @side-effects: draws frames, reads input, and sends UCI commands
fn run_loop<RunnerT>(
    terminal: &mut DefaultTerminal,
    app: &mut App,
    runner: &mut RunnerT,
) -> io::Result<()>
where
    RunnerT: UciRunner,
{
    if apply_effects(app.update(Action::Connect), app, runner) {
        return Ok(());
    }
    loop {
        while let Some(engine_event) = runner.try_recv()? {
            let effects = app.update(Action::Runner(engine_event));
            if apply_effects(effects, app, runner) {
                return Ok(());
            }
        }

        terminal.draw(|frame| render(frame, app))?;
        if event::poll(EVENT_POLL_INTERVAL)?
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
