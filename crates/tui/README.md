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

The platform-specific default engine path is `target/debug/chess-kit` (with
`.exe` on Windows) and assumes the TUI is launched from the workspace root.
Any local UCI engine can be selected explicitly, which is required when
running elsewhere:

```sh
cargo run -p chess-kit-tui -- --engine /path/to/stockfish
```

Piece assets are selected independently from the engine. The `ascii` set is
the default and can be selected explicitly:

```sh
cargo run -p chess-kit-tui -- --pieces ascii --engine /path/to/stockfish
```

Arguments after the standalone `--` continue to be forwarded to the engine.
The piece-set boundary lives under `ui/pieces`, so additional renderers such as
the planned Braille set can be added without changing board layout or cell
highlighting.

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

## First-pass scope

- The implemented transport launches one local child process and speaks UCI
  over standard input and output. The public runner boundary leaves room for
  socket or RPC transports, but they are not implemented yet.
- The client handles `id`, `option`, `uciok`, `readyok`, streaming
  `info`, and `bestmove`; it sends new-game, position, infinite-search,
  stop, readiness, option, and shutdown commands.
- This is an analysis board, not a play-vs-engine mode. `bestmove` is shown
  but is not automatically played. Moves entered on the board are validated
  through the toolkit's engine boundary.
- Promotion currently defaults to a queen. An underpromotion chooser is a
  follow-up interaction.
- The full interface requires at least a 40-column by 18-row terminal and uses
  Unicode chess symbols, block graphics, and color. A too-small terminal
  receives a resize message; terminals without Unicode support are not part of
  this POC.
- UCI initialization/readiness waits are limited to five seconds, a stopped
  search must return `bestmove` within two seconds, and shutdown allows 500
  milliseconds before terminating the direct child process. Engines should
  not daemonize or leave descendants holding inherited output pipes.

## Design

The crate is split into protocol, process, application, terminal, and UI
modules. Blocking engine output is read on a dedicated thread and delivered
to the application as typed events. Terminal input and engine output both
become application actions, which keeps state transitions independent of
rendering and makes them testable without a real terminal or chess engine.

Protocol and process tests use deterministic fixtures and never require a
system Stockfish installation.
