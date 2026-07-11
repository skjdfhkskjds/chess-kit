use chess_kit_collections::Map;
use criterion::{BatchSize, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;

use crate::fixtures::{colliding_key, spread_key, CompactValue, SplitU64Hasher};

const MAP_ITEMS: usize = 20_000;

pub fn compact_keys() -> Vec<u64> {
    (0..MAP_ITEMS as u64).map(spread_key).collect()
}

pub fn compact_values() -> Vec<CompactValue> {
    (0..MAP_ITEMS as u64).map(CompactValue::new).collect()
}

pub fn populated_compact_map(
    keys: &[u64],
    values: &[CompactValue],
) -> Map<u64, CompactValue, SplitU64Hasher> {
    let mut map = Map::<u64, CompactValue, SplitU64Hasher>::new(4);
    for (key, value) in keys.iter().zip(values.iter()) {
        map.set(key, *value);
    }
    map
}

pub fn bench_construction<V>(c: &mut Criterion, group_name: &str)
where
    V: chess_kit_collections::Value,
{
    let mut group = c.benchmark_group(group_name);
    for mib in [0_usize, 1, 8] {
        group.bench_function(BenchmarkId::new("new", mib), |b| {
            b.iter(|| black_box(Map::<u64, V, SplitU64Hasher>::new(black_box(mib))));
        });
    }
    group.finish();
}

pub fn bench_allocation_cost<V>(c: &mut Criterion, group_name: &str)
where
    V: chess_kit_collections::Value,
{
    let mut group = c.benchmark_group(group_name);
    for mib in [0_usize, 1, 8, 32] {
        group.bench_function(BenchmarkId::new("new", mib), |b| {
            b.iter(|| black_box(Map::<u64, V, SplitU64Hasher>::new(black_box(mib))));
        });
    }
    group.finish();
}

pub fn bench_compact_operations(c: &mut Criterion) {
    let keys = compact_keys();
    let values = compact_values();
    let populated = populated_compact_map(&keys, &values);

    let mut group = c.benchmark_group("map/compact_value/operations");
    group.throughput(Throughput::Elements(MAP_ITEMS as u64));

    group.bench_function("set_insert_spread", |b| {
        b.iter_batched(
            || Map::<u64, CompactValue, SplitU64Hasher>::new(4),
            |mut map| {
                for (key, value) in keys.iter().zip(values.iter()) {
                    map.set(black_box(key), black_box(*value));
                }
                black_box(map.len())
            },
            BatchSize::LargeInput,
        );
    });

    group.bench_function("set_update_existing", |b| {
        b.iter_batched(
            || populated_compact_map(&keys, &values),
            |mut map| {
                for (key, value) in keys.iter().zip(values.iter()) {
                    map.set(black_box(key), black_box(*value));
                }
                black_box(map.len())
            },
            BatchSize::LargeInput,
        );
    });

    group.bench_function("set_eviction_collisions", |b| {
        b.iter_batched(
            || Map::<u64, CompactValue, SplitU64Hasher>::new(1),
            |mut map| {
                for i in 0..MAP_ITEMS as u64 {
                    let key = colliding_key(0, i as u32 + 1);
                    map.set(black_box(&key), black_box(CompactValue::new(i)));
                }
                black_box(map.len())
            },
            BatchSize::LargeInput,
        );
    });

    group.bench_function("get_hits", |b| {
        b.iter(|| {
            let mut acc = 0_i32;
            for key in keys.iter() {
                if let Some(value) = populated.get(black_box(key)) {
                    acc ^= value.payload();
                }
            }
            black_box(acc)
        });
    });

    group.bench_function("get_misses", |b| {
        b.iter(|| {
            let mut acc = 0_usize;
            for key in keys.iter() {
                acc += populated.get(black_box(&key.wrapping_add(1))).is_some() as usize;
            }
            black_box(acc)
        });
    });

    group.finish();
}
