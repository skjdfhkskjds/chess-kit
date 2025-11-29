# chess-kit {#chess-kit}

[chess-kit](#chess-kit) aims to be a modular, open-source chess engine development toolkit written in Rust. The goal is to provide out-of-the-box usable [modules](#modules) to assemble a chess engine, while remaining modular through well-defined types and traits so that builders can easily swap out components to make their own custom engines.

## Design Ideology {#ideology}

[chess-kit](#chess-kit) is built around the following principles:

1. modularity: each component of the chess engine should be encapsulated behind its own trait(s), while still aiming for low(zero)-cost abstractions. Building engines with alternative implementations should be as plug-and-play as possible.
2. extensibility: this engine is not (and will likely never be) "complete". New features, optimizations, and algorithms should always be "one trait away" from being integrated into the existing codebase (see above for topic of modularity).
3. readability: although performance is expected, this project emphasizes code clarity and maintainability. The consequence is that certain optimizations may be ommitted to favour readability.

## Modules {#modules}

- [board](#board): the representation of the chess board
- [movegen](#movegen): move generation logic
- [search](#search): state/game tree searching
- [tablebase](#tablebase): opening book and endgame tablebase support
- [eval](#eval): position evaluation algorithms
- [uci](#uci): UCI protocol implementation for engine communication
- [primitives](#primitives): core types used as the building blocks for other modules
