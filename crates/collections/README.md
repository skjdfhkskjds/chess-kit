# chess-kit-collections

Custom collection types used by `chess-kit`.

## Types

- `Stack<T, CAP>`: fixed-capacity stack for copyable state histories.
- `Map<K, V, Hasher, Policy>`: fixed-memory hash map with bucket-local entries and priority-based eviction.

## Testing

Run the crate's unit tests with:

```sh
cargo test -p chess-kit-collections
```

Run only the stack tests with:

```sh
cargo test -p chess-kit-collections stack
```

Run only the map tests with:

```sh
cargo test -p chess-kit-collections map
```

## Benchmarking

Compile the benchmark targets without running them:

```sh
cargo bench -p chess-kit-collections --bench collections_runtime --no-run
cargo bench -p chess-kit-collections --bench collections_memory --no-run
```

Run the runtime benchmarks:

```sh
cargo bench -p chess-kit-collections --bench collections_runtime
```

Run the memory layout and allocation benchmarks:

```sh
cargo bench -p chess-kit-collections --bench collections_memory
```

Criterion writes benchmark reports under `target/criterion/`.
