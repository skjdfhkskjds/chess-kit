# chess-kit-collections

Custom collection types used by `chess-kit`.

## Types

- `FixedArray<T, N>`: fixed-capacity, variable-length collection with inline storage.
- `Stack<T, N>`: fixed-capacity stack for copyable state histories.
- `Map<K, V, Hasher, Policy>`: fixed-memory hash map with bucket-local entries and priority-based eviction.

## Traits

- `Retain<T>`: in-place, order-preserving removal of items that do not match a predicate.

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
cargo bench -p chess-kit-collections --bench stack_small_state --no-run
cargo bench -p chess-kit-collections --bench stack_history_state --no-run
cargo bench -p chess-kit-collections --bench map_compact_value --no-run
cargo bench -p chess-kit-collections --bench map_wide_value --no-run
cargo bench -p chess-kit-collections --bench map_node_value --no-run
cargo bench -p chess-kit-collections --bench baseline_std_hash_map --no-run
cargo bench -p chess-kit-collections --bench fixed_array --no-run
```

Run benchmarks isolated by collection variant:

```sh
cargo bench -p chess-kit-collections --bench stack_small_state
cargo bench -p chess-kit-collections --bench stack_history_state
cargo bench -p chess-kit-collections --bench map_compact_value
cargo bench -p chess-kit-collections --bench map_wide_value
cargo bench -p chess-kit-collections --bench map_node_value
cargo bench -p chess-kit-collections --bench fixed_array
```

Run the non-equivalent standard library baseline:

```sh
cargo bench -p chess-kit-collections --bench baseline_std_hash_map
```
