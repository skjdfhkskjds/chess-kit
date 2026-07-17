# chess-kit

[chess-kit](#chess-kit) aims to be a modular, open-source chess engine development toolkit written in Rust. The goal is to provide out-of-the-box usable [modules](#modules) to assemble a chess engine, while remaining modular through well-defined types and traits so that builders can easily swap out components to make their own custom engines.

## Design Ideology

[chess-kit](#chess-kit) is built around the following principles:

1. modularity: each component of the chess engine should be encapsulated behind its own trait(s), while still aiming for low(zero)-cost abstractions. Building engines with alternative implementations should be as plug-and-play as possible.
2. extensibility: this engine is not (and will likely never be) "complete". New features, optimizations, and algorithms should always be "one trait away" from being integrated into the existing codebase (see above for topic of modularity).
3. readability: although performance is expected, this project emphasizes code clarity and maintainability. The consequence is that certain optimizations may be ommitted to favour readability.

## Modules

- [attack_table](crates/attack_table): attack table generation and lookup
- [collections](crates/collections): custom collection types
- [comm](crates/comm): communication protocols for chess engines
- [eval](crates/eval): position evaluation algorithms
- [macros](crates/macros): derive macros used by other crates
- [movegen](crates/movegen): move generation logic
- [perft](crates/perft): perft utilities for validating move generation
- [position](crates/position): chess board and position representation
- [primitives](crates/primitives): core types used as the building blocks for other modules
- [search](crates/search): chess position search algorithms
- [transposition](crates/transposition): transposition table support

## UCI

The top-level binary is a minimal [UCI](https://backscattering.de/chess/uci/)
engine backed by the current iterative-deepening search with a fixed target
depth:

```sh
cargo run --release
```

It supports the minimum command set needed by common chess GUIs and SPRT
runners: `uci`, `isready`, `ucinewgame`, `position startpos`, `position fen`,
clock-based `go`, and `quit`. It also accepts `go depth`, `go nodes`,
`go movetime`, `stop`, and `ponderhit` as protocol primitives, although the
current search is synchronous and only uses the depth constraint (clamped to
the supported range of 1–5 plies).

See [docs/sprt.md](docs/sprt.md) for an initial local SPRT workflow.

### Play in the terminal

An interactive façade keeps the move history, displays the current position,
and searches to depth 6 by default:

```sh
cargo run --release -p chess-kit-comm --example game
```

Use `--depth` to choose a search depth from 1 through 8 plies:

```sh
cargo run --release -p chess-kit-comm --example game -- --depth 4
```

You play White. Enter one move at a time in UCI notation, such as `e2e4` or
`e7e8q`; enter `quit` to stop. This interactive façade is an example target and
is not included in the `chess-kit-comm` library or the top-level UCI binary.

## Testing

Run the workspace's normal unit and integration tests with:

```sh
cargo test --workspace --lib --tests
```

Perft correctness lives with the `perft` crate:

```sh
cargo test -p chess-kit-perft --release --test perft_smoke
```

The larger perft suite is useful as a benchmarking tool but superfluous for correctness testing:

```sh
cargo test -p chess-kit-perft --release --test perft_full -- --ignored --no-capture
```
