# Style Guide

This guide documents repository-specific conventions that supplement standard
Rust style. Its purpose is to preserve readable code, compile-time
specialization, and clear boundaries between crates.

## Documentation

Use Rust documentation comments for the contract of an item:

- `//!` documents a crate or module.
- `///` documents a type, trait, function, method, constant, or field.
- `//` explains implementation details that are relevant only to nearby code.

Documentation should explain observable behavior, invariants, design intent,
and non-obvious constraints. Do not merely restate the syntax of the item.

Begin item documentation with a concise sentence naming the item and describing
its purpose:

```rust
/// `MoveGenerator` defines the move-generation contract.
///
/// @trait
pub trait MoveGenerator {
    // ...
}
```

Use backticks around Rust identifiers and code expressions.

### Documentation annotations

Annotations form a compact, consistent description of an item's contract.
Place them after the prose description, separated from it by a blank
documentation line.

Item annotations are:

- `@type` for a struct, enum, or type alias.
- `@trait` for a trait.
- `@marker-type` for a marker type used in compile-time dispatch.

Callable annotations are:

- `@marker: Name - description` for a generic marker parameter.
- `@param: name - description` for a runtime parameter.
- `@return: description` for the returned value.
- `@side-effects: description` for externally observable mutation or effects.
- `@requires: description` for a precondition the caller must uphold.
- `@impl: Trait::method` for a trait method implementation.

List `@marker` and `@param` annotations in signature order. Follow them with
`@return`, `@side-effects`, and `@requires`, when applicable. Use
`@return: void` for a function that returns `()`.

A trait method defines the complete contract:

```rust
/// `play_unchecked` applies a move without checking its legality.
///
/// @param: mv - move to apply
/// @return: deterministic piece delta produced by the move
/// @side-effects: modifies the position and its internal state
/// @requires: `mv` must be legal in the current position
fn play_unchecked(&mut self, mv: Move) -> MoveDelta;
```

An implementation references that contract instead of duplicating it:

```rust
/// @impl: PositionMoves::play_unchecked
fn play_unchecked(&mut self, mv: Move) -> MoveDelta {
    // ...
}
```

When additional implementation behavior is important, explain it before the
`@impl` annotation.

### Implementation comments

Implementation comments should explain why the code takes a particular
approach, especially when they describe:

- an invariant not represented by the type system;
- a performance-sensitive choice;
- an intentionally omitted case;
- a runtime-to-compile-time dispatch boundary; or
- behavior that would otherwise look accidental.

Use conventional markers consistently:

- `TODO:` for known, scoped follow-up work.
- `FIXME:` for behavior known to be incorrect.
- `SAFETY:` for the invariant that makes an `unsafe` operation valid.
- `NOTE:` sparingly for important local context that does not fit the item
  contract.

A `TODO` or `FIXME` must describe the missing or incorrect behavior. Include an
issue reference when one exists.

## Imports and Re-exports

Organize imports into blocks in this order:

1. standard library imports from `std`, `core`, or `alloc`;
2. external crate imports, including other workspace crates;
3. crate-local imports using `crate`, `self`, or `super`;
4. public re-exports using `pub use`;
5. crate-visible re-exports using `pub(crate) use`.

Separate each non-empty block with a blank line. For example, implementation
files place standard, external, and crate-local imports in distinct blocks:

```rust
use std::time::Instant;

use chess_kit_eval::{Accumulator, EvalState};
use chess_kit_primitives::{Move, SearchDepth};

use crate::{Engine, EngineConfig, EngineError};
```

In a `lib.rs`, re-export blocks follow all ordinary import blocks, with public
re-exports before crate-visible re-exports:

```rust
mod moving_pieces;
mod table;

use chess_kit_primitives::{Bitboard, Side, Square};

pub use table::DefaultAttackTable;

pub(crate) use moving_pieces::{NOT_A_FILE, NOT_H_FILE};
```

Do not use wildcard imports. Every imported or re-exported item must be named
explicitly.

The sole exception is `use super::*;` inside a unit-test module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // ...
}
```

This exception does not apply to integration tests, production code, or
wildcard imports from any path other than `super`.

## Marker Types and Static Dispatch

Prefer marker types when a small, closed category changes behavior and the
choice can be resolved at compile time. Typical examples include chess sides
and interchangeable stateless implementations.

A marker type should:

- be a zero-sized unit struct;
- contain no runtime state;
- implement a trait that provides its associated types, constants, or
  operations;
- use the `@marker-type` documentation annotation; and
- be passed as a generic type parameter rather than as a value.

```rust
/// `White` selects white-side behavior at compile time.
///
/// @marker-type
pub struct White;

pub trait Side {
    type Other: Side;
    const SIDE: Sides;
}
```

Name marker generic parameters after their role and give them a `T` suffix,
such as `SideT`, `PositionT`, or `AttackTableT`. Prefer descriptive names over
abbreviations in public interfaces.

```rust
fn occupancy<SideT: Side>(&self) -> Bitboard;
```

When a struct is generic over a marker but stores no value of that type, retain
the relationship with `PhantomData`:

```rust
pub struct DefaultMoveGenerator<AttackTableT: AttackTable> {
    _attack_table: PhantomData<AttackTableT>,
}
```

Runtime values should be converted to marker types once, at the narrowest
practical boundary. Code below that boundary should remain generic so the
selected path can be statically dispatched:

```rust
call_as!(position.turn(), |SideT| {
    generate_for_side::<SideT>(position)
});
```

Do not introduce a marker type when the choice is inherently dynamic, carries
runtime data, or would cause unreasonable duplication without a measured
benefit.

## Crate Boundaries

Every crate owns an abstraction boundary. Code in another crate must consume
its capabilities through traits rather than depend on its concrete
implementations.

The trait or traits that define a crate's publicly accessible contract must be
defined in that crate's `lib.rs`. This makes the supported boundary visible
without requiring consumers to understand the crate's internal module layout.

Default implementations may live in private modules, but each must be
explicitly re-exported from `lib.rs` using a dedicated `pub use` statement. Do
not group a default implementation with other re-exports on the same line:

```rust
mod movegen;

pub use movegen::DefaultMoveGenerator;

/// `MoveGenerator` defines the move-generation contract.
///
/// @trait
pub trait MoveGenerator {
    // ...
}
```

Cross-crate code must:

- accept or store dependencies through trait bounds or trait objects;
- call behavior declared by those traits;
- use generic parameters for statically dispatched implementations; and
- use test doubles that implement the same traits.

Cross-crate code must not:

- depend on another crate's concrete implementation in operational logic;
- call another crate's implementation-specific inherent methods;
- inspect another crate's internal representation; or
- reproduce behavior that belongs behind the providing crate's trait.

A composition root may name a concrete implementation to select and construct
it. After construction, consuming code must interact with its capabilities
through the corresponding trait. Naming a type for composition does not make
its implementation-specific API part of the boundary.

### Primitives exception

`chess-kit-primitives` is the sole exception to the trait-only boundary rule.
It defines the fundamental vocabulary and value types shared by the
repository. Its public types may be named, passed, stored, matched, and
otherwise consumed directly across crate boundaries.

This exception applies only to primitive value types and their intrinsic
operations. It must not be used to move higher-level engine behavior or
crate-specific implementation details into `chess-kit-primitives`.

When deciding where a new type belongs:

- place it in `chess-kit-primitives` only when it is a fundamental,
  implementation-independent building block used broadly across the
  repository;
- otherwise, keep it in the crate that owns the concept and expose cross-crate
  behavior through a trait.
