# chess-kit-tui

`chess-kit-tui` is an interactive terminal client for playing against or
analyzing with a local UCI chess engine. It uses a child process because UCI is
a newline-delimited standard-input/standard-output protocol. A transport
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

The default mode is a game against the engine. You play White and the engine
automatically replies after each legal move with a fixed search depth of 6.
To move pieces for either side and start or stop analysis manually, launch in
analysis mode:

```sh
cargo run -p chess-kit-tui -- --mode analysis
```

The platform-specific default engine path is `target/debug/chess-kit` (with
`.exe` on Windows) and assumes the TUI is launched from the workspace root.
Any local UCI engine can be selected explicitly, which is required when
running elsewhere:

```sh
cargo run -p chess-kit-tui -- --engine /path/to/stockfish
```

Pieces use the default `ascii` set, which samples external CC0 silhouette masks
into opaque Unicode block cells. It can also be selected explicitly:

```sh
cargo run -p chess-kit-tui -- --pieces ascii --engine /path/to/stockfish
```

Arguments after the standalone `--` continue to be forwarded to the engine.
The piece-set boundary lives under `ui/pieces`, so renderers can be added
without changing board layout or cell highlighting. Asset provenance is
documented in `assets/pieces/ascii/SOURCE.md`.

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

`Space` applies only in analysis mode. In the default play mode, entering a
legal White move starts the engine reply automatically. The current mode is
always shown in the header; select it at launch with `--mode play` or
`--mode analysis`.

## First-pass scope

- The implemented transport launches one local child process and speaks UCI
  over standard input and output. The public runner boundary leaves room for
  socket or RPC transports, but they are not implemented yet.
- The client handles `id`, `option`, `uciok`, `readyok`, streaming
  `info`, and `bestmove`; it sends new-game, position, infinite-search,
  stop, readiness, option, and shutdown commands.
- Play mode supports a human playing White against automatic fixed-depth-6
  engine replies. Color choice and adjustable engine strength are not yet
  implemented. Analysis mode remains available for moving either side and
  manually starting or stopping an infinite search. Moves entered on the board
  and engine replies are validated through the toolkit's engine boundary.
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
