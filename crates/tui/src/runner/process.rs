use std::ffi::OsStr;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command, Stdio};
use std::str::FromStr;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};

use crate::{EngineMessage, UciCommand, UciRunner};

/// SHUTDOWN_GRACE_PERIOD is the time allowed for a clean UCI exit.
const SHUTDOWN_GRACE_PERIOD: Duration = Duration::from_millis(500);

/// `RunnerEvent` describes output and lifecycle changes from an engine.
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunnerEvent {
    Message(EngineMessage),
    StandardError(String),
    Error(String),
    Exited(Option<i32>),
}

/// `ProcessRunner` communicates with a local UCI child process.
///
/// Standard output and standard error are drained on dedicated threads so an
/// engine cannot block on a full pipe while the terminal is waiting for input.
///
/// @type
pub struct ProcessRunner {
    child: Child,
    input: Option<ChildStdin>,
    events: Receiver<RunnerEvent>,
    exit_reported: bool,
}

impl ProcessRunner {
    /// spawn starts an executable with piped UCI input and output.
    ///
    /// @marker: ProgramT - executable path type
    /// @marker: ArgsT - argument collection type
    /// @marker: ArgumentT - argument value type
    /// @param: program - engine executable path
    /// @param: arguments - arguments passed to the executable
    /// @return: connected process runner, or an I/O error
    /// @side-effects: starts a child process and two reader threads
    pub fn spawn<ProgramT, ArgsT, ArgumentT>(
        program: ProgramT,
        arguments: ArgsT,
    ) -> io::Result<Self>
    where
        ProgramT: AsRef<OsStr>,
        ArgsT: IntoIterator<Item = ArgumentT>,
        ArgumentT: AsRef<OsStr>,
    {
        let mut child = Command::new(program)
            .args(arguments)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let input = child
            .stdin
            .take()
            .ok_or_else(|| io::Error::other("engine stdin was not piped"))?;
        let output = child
            .stdout
            .take()
            .ok_or_else(|| io::Error::other("engine stdout was not piped"))?;
        let errors = child
            .stderr
            .take()
            .ok_or_else(|| io::Error::other("engine stderr was not piped"))?;
        let (sender, events) = mpsc::channel();
        spawn_stdout_reader(output, sender.clone());
        spawn_stderr_reader(errors, sender);

        Ok(Self {
            child,
            input: Some(input),
            events,
            exit_reported: false,
        })
    }

    /// wait_for_exit waits briefly and terminates an unresponsive child.
    ///
    /// @return: Ok after the child is reaped, or an I/O error
    /// @side-effects: may terminate the child process
    fn wait_for_exit(&mut self) -> io::Result<()> {
        let deadline = Instant::now() + SHUTDOWN_GRACE_PERIOD;
        while Instant::now() < deadline {
            if self.child.try_wait()?.is_some() {
                self.exit_reported = true;
                return Ok(());
            }
            thread::sleep(Duration::from_millis(10));
        }

        self.child.kill()?;
        self.child.wait()?;
        self.exit_reported = true;
        Ok(())
    }
}

impl UciRunner for ProcessRunner {
    /// @impl: UciRunner::send
    fn send(&mut self, command: &UciCommand) -> io::Result<()> {
        let input = self
            .input
            .as_mut()
            .ok_or_else(|| io::Error::new(io::ErrorKind::BrokenPipe, "engine input is closed"))?;
        writeln!(input, "{command}")?;
        input.flush()
    }

    /// @impl: UciRunner::try_recv
    fn try_recv(&mut self) -> io::Result<Option<RunnerEvent>> {
        match self.events.try_recv() {
            Ok(event) => return Ok(Some(event)),
            Err(TryRecvError::Empty | TryRecvError::Disconnected) => {}
        }

        if !self.exit_reported && let Some(status) = self.child.try_wait()? {
            self.exit_reported = true;
            return Ok(Some(RunnerEvent::Exited(status.code())));
        }
        Ok(None)
    }

    /// @impl: UciRunner::shutdown
    fn shutdown(&mut self) -> io::Result<()> {
        if self.child.try_wait()?.is_some() {
            self.input.take();
            self.exit_reported = true;
            return Ok(());
        }

        if let Some(mut input) = self.input.take() {
            writeln!(input, "{}", UciCommand::Quit)?;
            input.flush()?;
        }
        self.wait_for_exit()
    }
}

impl Drop for ProcessRunner {
    /// drop prevents an engine child from outliving its runner.
    ///
    /// @impl: Drop::drop
    fn drop(&mut self) {
        if self.shutdown().is_err() {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

/// spawn_stdout_reader converts engine standard output lines into typed events.
///
/// @param: output - engine standard output pipe
/// @param: sender - event destination
/// @return: void
/// @side-effects: starts a detached reader thread
fn spawn_stdout_reader(output: ChildStdout, sender: Sender<RunnerEvent>) {
    thread::spawn(move || {
        read_lines(
            output,
            |line| {
                let message = EngineMessage::from_str(&line).unwrap_or_else(|never| match never {});
                RunnerEvent::Message(message)
            },
            &sender,
        );
    });
}

/// spawn_stderr_reader forwards engine standard error lines as diagnostics.
///
/// @param: errors - engine standard error pipe
/// @param: sender - event destination
/// @return: void
/// @side-effects: starts a detached reader thread
fn spawn_stderr_reader(errors: ChildStderr, sender: Sender<RunnerEvent>) {
    thread::spawn(move || {
        read_lines(errors, RunnerEvent::StandardError, &sender);
    });
}

/// read_lines drains one engine pipe until EOF or a read error.
///
/// @marker: ReaderT - readable engine pipe type
/// @marker: MapT - line-to-event mapping type
/// @param: reader - engine output pipe
/// @param: map - successful line mapping
/// @param: sender - event destination
/// @return: void
/// @side-effects: reads the pipe and sends events
fn read_lines<ReaderT, MapT>(reader: ReaderT, map: MapT, sender: &Sender<RunnerEvent>)
where
    ReaderT: Read,
    MapT: Fn(String) -> RunnerEvent,
{
    for line in BufReader::new(reader).lines() {
        let event = match line {
            Ok(line) => map(line),
            Err(error) => RunnerEvent::Error(error.to_string()),
        };
        if sender.send(event).is_err() {
            break;
        }
    }
}
