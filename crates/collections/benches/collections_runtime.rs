mod fixtures;

use chess_kit_collections::{Map, Stack};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use fixtures::{
    colliding_key, spread_key, CompactValue, HistoryState, SmallState, SplitU64Hasher, WideValue,
};
use std::collections::HashMap;
use std::hint::black_box;

const STACK_ITERS: usize = 10_000;
const MAP_ITEMS: usize = 20_000;

fn bench_stack_push_pop<T, const CAP: usize>(
    c: &mut Criterion,
    group_name: &str,
    make_value: impl Fn(u64) -> T + Copy,
) where
    T: chess_kit_collections::Copyable,
{
    let mut group = c.benchmark_group(group_name);
    group.throughput(Throughput::Elements(STACK_ITERS as u64));
    group.bench_function(BenchmarkId::new("push_pop", CAP), |b| {
        b.iter(|| {
            let mut stack = Stack::<T, CAP>::new();
            for i in 0..STACK_ITERS {
                if stack.size() + 1 >= CAP {
                    while !stack.is_empty() {
                        stack.pop();
                    }
                }
                stack.push(black_box(make_value(i as u64)));
            }
            black_box(stack.size())
        });
    });
    group.finish();
}

fn bench_stack_push_next<T, const CAP: usize>(
    c: &mut Criterion,
    group_name: &str,
    mut update: impl FnMut(&mut T, u64) + Copy,
) where
    T: chess_kit_collections::Copyable,
{
    let mut group = c.benchmark_group(group_name);
    group.throughput(Throughput::Elements(STACK_ITERS as u64));
    group.bench_function(BenchmarkId::new("push_next_pop", CAP), |b| {
        b.iter(|| {
            let mut stack = Stack::<T, CAP>::new();
            stack.push(T::default());
            for i in 0..STACK_ITERS {
                if stack.size() + 1 >= CAP {
                    while stack.size() > 1 {
                        stack.pop();
                    }
                }
                let item = stack.push_next();
                update(item, black_box(i as u64));
            }
            black_box(stack.size())
        });
    });
    group.finish();
}

fn bench_stack_iter(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack/iteration");
    for len in [16_usize, 128, 254] {
        group.throughput(Throughput::Elements(len as u64));
        group.bench_function(BenchmarkId::new("forward", len), |b| {
            b.iter_batched(
                || {
                    let mut stack = Stack::<SmallState, 255>::new();
                    for i in 0..len {
                        stack.push(SmallState::with_seed(i as u64));
                    }
                    stack
                },
                |stack| {
                    let mut acc = 0_usize;
                    for item in stack.iter() {
                        acc ^= black_box(item as *const SmallState as usize);
                    }
                    black_box(acc)
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_function(BenchmarkId::new("reverse_partial_until_last", len), |b| {
            b.iter_batched(
                || {
                    let mut stack = Stack::<SmallState, 255>::new();
                    for i in 0..len {
                        stack.push(SmallState::with_seed(i as u64));
                    }
                    stack
                },
                |stack| {
                    let mut iter = stack.iter().rev();
                    let mut acc = 0_usize;
                    for _ in 0..len.saturating_sub(1) {
                        let item = iter.next().expect("reverse iterator should have an item");
                        acc ^= black_box(item as *const SmallState as usize);
                    }
                    black_box(acc)
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_map_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("map/construction");
    for mib in [0_usize, 1, 8] {
        group.bench_function(BenchmarkId::new("new_compact", mib), |b| {
            b.iter(|| {
                black_box(Map::<u64, CompactValue, SplitU64Hasher>::new(black_box(
                    mib,
                )))
            });
        });
        group.bench_function(BenchmarkId::new("new_wide", mib), |b| {
            b.iter(|| black_box(Map::<u64, WideValue, SplitU64Hasher>::new(black_box(mib))));
        });
    }
    group.finish();
}

fn bench_map_set_get(c: &mut Criterion) {
    let keys: Vec<u64> = (0..MAP_ITEMS as u64).map(spread_key).collect();
    let values: Vec<CompactValue> = (0..MAP_ITEMS as u64).map(CompactValue::new).collect();
    let mut populated = Map::<u64, CompactValue, SplitU64Hasher>::new(4);
    for (key, value) in keys.iter().zip(values.iter()) {
        populated.set(key, *value);
    }

    let mut group = c.benchmark_group("map/operations");
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
            || {
                let mut map = Map::<u64, CompactValue, SplitU64Hasher>::new(4);
                for (key, value) in keys.iter().zip(values.iter()) {
                    map.set(key, *value);
                }
                map
            },
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

fn bench_hash_map_baseline(c: &mut Criterion) {
    let keys: Vec<u64> = (0..MAP_ITEMS as u64).map(spread_key).collect();
    let values: Vec<CompactValue> = (0..MAP_ITEMS as u64).map(CompactValue::new).collect();
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

fn stack_benches(c: &mut Criterion) {
    bench_stack_push_pop::<SmallState, 8>(c, "stack/small_state", SmallState::with_seed);
    bench_stack_push_pop::<SmallState, 64>(c, "stack/small_state", SmallState::with_seed);
    bench_stack_push_pop::<SmallState, 255>(c, "stack/small_state", SmallState::with_seed);
    bench_stack_push_pop::<HistoryState, 255>(c, "stack/history_state", HistoryState::with_seed);
    bench_stack_push_next::<SmallState, 255>(c, "stack/small_state", SmallState::update);
    bench_stack_push_next::<HistoryState, 255>(
        c,
        "stack/history_state",
        HistoryState::update_header,
    );
    bench_stack_iter(c);
}

fn map_benches(c: &mut Criterion) {
    bench_map_construction(c);
    bench_map_set_get(c);
    bench_hash_map_baseline(c);
}

criterion_group!(benches, stack_benches, map_benches);
criterion_main!(benches);
