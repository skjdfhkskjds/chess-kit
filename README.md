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
- [eval](crates/eval): position evaluation algorithms
- [macros](crates/macros): derive macros used by other crates
- [movegen](crates/movegen): move generation logic
- [perft](crates/perft): perft utilities for validating move generation
- [position](crates/position): chess board and position representation
- [primitives](crates/primitives): core types used as the building blocks for other modules
- [transposition](crates/transposition): transposition table support

## Testing

Run the workspace's normal unit and integration tests with:

```sh
cargo test --workspace --lib --tests
```

Perft correctness lives with the `perft` crate:

```sh
cargo test -p chess-kit-perft --test perft_smoke
```

The larger perft suite is an explicit full regression run, not a default benchmark:

```sh
cargo test -p chess-kit-perft --release --test perft_full -- --ignored --nocapture
```

`cargo test --workspace` also runs doctests. Some existing macro doctest examples are not currently valid test cases, so use `--lib --tests` for the current default correctness path until those examples are repaired.
