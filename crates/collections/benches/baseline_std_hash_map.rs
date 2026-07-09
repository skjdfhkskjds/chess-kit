mod fixtures;
mod support;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use fixtures::CompactValue;
use std::collections::HashMap;
use std::hint::black_box;

const MAP_ITEMS: usize = 20_000;

fn performance_benches(c: &mut Criterion) {
    let keys = support::map::compact_keys();
    let values = support::map::compact_values();
    let mut group = c.benchmark_group("baseline/std_hash_map_non_equivalent");
    group.throughput(Throughput::Elements(MAP_ITEMS as u64));
    group.bench_function("insert", |b| {
        b.iter_batched(
            HashMap::<u64, CompactValue>::new,
            |mut map| {
                for (key, value) in keys.iter().zip(values.iter()) {
                    map.insert(black_box(*key), black_box(*value));
                }
                black_box(map.len())
            },
            BatchSize::LargeInput,
        );
    });
    group.finish();
}

criterion_group!(benches, performance_benches);
criterion_main!(benches);
