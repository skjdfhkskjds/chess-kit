# chess-kit-tui

`chess-kit-tui` is an interactive terminal client for a local UCI chess
engine. The first pass deliberately uses a child process because UCI is a
newline-delimited standard-input/standard-output protocol. A transport
boundary keeps room for socket or RPC-backed engines later.

The interface follows familiar chess analysis layouts:

- the board is the primary pane;
- engine identity, connection state, and search state stay visible;
- search evaluation, depth, nodes, speed, and principal variation share an
  analysis pane;
- move history and raw protocol activity are available without obscuring the
  board; and
- a compact footer advertises the active keyboard controls.

## Run

Build the repository engine, then launch the TUI from the workspace root:

```sh
cargo build
cargo run -p chess-kit-tui
```

The default engine path is `target/debug/chess-kit`. Any local UCI engine can
be selected explicitly:

```sh
cargo run -p chess-kit-tui -- --engine /path/to/stockfish
```

Use `--` after the engine path to forward arguments to the engine process.

## Controls

| Key | Action |
| --- | --- |
| Arrow keys or `h j k l` | Move the board cursor |
| `Enter` | Select a source square or play to a destination |
| `Esc` | Cancel the current selection |
| `Space` | Start or stop analysis |
| `f` | Flip the board |
| `n` | Start a new game |
| `p` | Toggle the protocol log |
| `?` | Toggle help |
| `q` | Quit |

## Design

The crate is split into protocol, process, application, terminal, and UI
modules. Blocking engine output is read on a dedicated thread and delivered
to the application as typed events. Terminal input and engine output both
become application actions, which keeps state transitions independent of
rendering and makes them testable without a real terminal or chess engine.

Protocol and process tests use deterministic fixtures and never require a
system Stockfish installation.

