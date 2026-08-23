use std::ffi::OsStr;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command, ExitStatus, Stdio};
use std::str::FromStr;
use std::sync::mpsc::{self, Receiver, SyncSender, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};

use crate::{EngineMessage, UciCommand, UciRunner};

/// SHUTDOWN_GRACE_PERIOD is the time allowed for a clean UCI exit.
const SHUTDOWN_GRACE_PERIOD: Duration = Duration::from_millis(500);

/// EVENT_BUFFER_CAPACITY bounds unread engine output.
const EVENT_BUFFER_CAPACITY: usize = 1024;

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

/// `ReaderEvent` adds pipe lifecycle signals to public runner events.
///
/// @type
enum ReaderEvent {
    Event(RunnerEvent),
    StandardOutputClosed,
    StandardErrorClosed,
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
    events: Receiver<ReaderEvent>,
    exit_status: Option<ExitStatus>,
    stdout_closed: bool,
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
        // Bound unread output so a noisy engine cannot consume memory without
        // limit while the terminal is processing input and rendering.
        let (sender, events) = mpsc::sync_channel(EVENT_BUFFER_CAPACITY);
        spawn_stdout_reader(output, sender.clone());
        spawn_stderr_reader(errors, sender);

        Ok(Self {
            child,
            input: Some(input),
            events,
            exit_status: None,
            stdout_closed: false,
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
            if let Some(status) = self.child.try_wait()? {
                self.exit_status = Some(status);
                self.exit_reported = true;
                return Ok(());
            }
            thread::sleep(Duration::from_millis(10));
        }

        self.child.kill()?;
        self.exit_status = Some(self.child.wait()?);
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
        loop {
            match self.events.try_recv() {
                Ok(ReaderEvent::Event(event)) => return Ok(Some(event)),
                Ok(ReaderEvent::StandardOutputClosed) => self.stdout_closed = true,
                Ok(ReaderEvent::StandardErrorClosed) => {}
                Err(TryRecvError::Empty | TryRecvError::Disconnected) => break,
            }
        }

        if self.exit_status.is_none() {
            self.exit_status = self.child.try_wait()?;
        }
        // EOF is queued after all stdout lines, preventing Exited from
        // overtaking a final bestmove or info response.
        if !self.exit_reported
            && self.stdout_closed
            && let Some(status) = self.exit_status
        {
            self.exit_reported = true;
            return Ok(Some(RunnerEvent::Exited(status.code())));
        }
        Ok(None)
    }

    /// @impl: UciRunner::shutdown
    fn shutdown(&mut self) -> io::Result<()> {
        if let Some(status) = self.child.try_wait()? {
            self.input.take();
            self.exit_status = Some(status);
            self.exit_reported = true;
            return Ok(());
        }

        let send_result = self.input.take().map_or(Ok(()), |mut input| {
            writeln!(input, "{}", UciCommand::Quit)?;
            input.flush()
        });
        // Always reap or terminate the child, even after a broken input pipe.
        let exit_result = self.wait_for_exit();
        exit_result.and(send_result)
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
fn spawn_stdout_reader(output: ChildStdout, sender: SyncSender<ReaderEvent>) {
    thread::spawn(move || {
        read_lines(
            output,
            |line| {
                let message = EngineMessage::from_str(&line).unwrap_or_else(|never| match never {});
                RunnerEvent::Message(message)
            },
            &sender,
            ReaderEvent::StandardOutputClosed,
        );
    });
}

/// spawn_stderr_reader forwards engine standard error lines as diagnostics.
///
/// @param: errors - engine standard error pipe
/// @param: sender - event destination
/// @return: void
/// @side-effects: starts a detached reader thread
fn spawn_stderr_reader(errors: ChildStderr, sender: SyncSender<ReaderEvent>) {
    thread::spawn(move || {
        read_lines(
            errors,
            RunnerEvent::StandardError,
            &sender,
            ReaderEvent::StandardErrorClosed,
        );
    });
}

/// read_lines drains one engine pipe until EOF or a read error.
///
/// @marker: ReaderT - readable engine pipe type
/// @marker: MapT - line-to-event mapping type
/// @param: reader - engine output pipe
/// @param: map - successful line mapping
/// @param: sender - event destination
/// @param: closed - pipe closure signal
/// @return: void
/// @side-effects: reads the pipe and sends events
fn read_lines<ReaderT, MapT>(
    reader: ReaderT,
    map: MapT,
    sender: &SyncSender<ReaderEvent>,
    closed: ReaderEvent,
) where
    ReaderT: Read,
    MapT: Fn(String) -> RunnerEvent,
{
    for line in BufReader::new(reader).lines() {
        let event = match line {
            Ok(line) => map(line),
            Err(error) => RunnerEvent::Error(error.to_string()),
        };
        if sender.send(ReaderEvent::Event(event)).is_err() {
            break;
        }
    }
    let _ = sender.send(closed);
}
