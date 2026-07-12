use chess_kit_collections::Stack;
use criterion::{BatchSize, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;

use crate::fixtures::SmallState;

const STACK_ITERS: usize = 10_000;

pub fn bench_construction<T, const CAP: usize>(c: &mut Criterion, group_name: &str)
where
    T: chess_kit_collections::Copyable,
{
    let mut group = c.benchmark_group(group_name);
    group.throughput(Throughput::Elements(CAP as u64));
    group.bench_function(BenchmarkId::new("new", CAP), |b| {
        b.iter(|| black_box(Stack::<T, CAP>::new()));
    });
    group.finish();
}

pub fn bench_push_pop<T, const CAP: usize>(
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
                if stack.is_full() {
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

pub fn bench_push_next<T, const CAP: usize>(
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
                if stack.is_full() {
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

pub fn bench_push_next_pop_pair<T, const CAP: usize>(
    c: &mut Criterion,
    group_name: &str,
    mut update: impl FnMut(&mut T, u64) + Copy,
) where
    T: chess_kit_collections::Copyable,
{
    let mut group = c.benchmark_group(group_name);
    group.throughput(Throughput::Elements(STACK_ITERS as u64));
    group.bench_function(BenchmarkId::new("push_next_pop_pair", CAP), |b| {
        b.iter(|| {
            let mut stack = Stack::<T, CAP>::new();
            stack.push(T::default());
            for i in 0..STACK_ITERS {
                let item = stack.push_next();
                update(item, black_box(i as u64));
                stack.pop();
            }
            black_box(stack.size())
        });
    });
    group.finish();
}

pub fn bench_top_access<T, const CAP: usize>(c: &mut Criterion, group_name: &str)
where
    T: chess_kit_collections::Copyable,
{
    let mut group = c.benchmark_group(group_name);
    group.throughput(Throughput::Elements(STACK_ITERS as u64));

    group.bench_function(BenchmarkId::new("top", CAP), |b| {
        b.iter_batched(
            || {
                let mut stack = Stack::<T, CAP>::new();
                stack.push(T::default());
                stack
            },
            |stack| {
                for _ in 0..STACK_ITERS {
                    black_box(stack.top());
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function(BenchmarkId::new("top_mut", CAP), |b| {
        b.iter_batched(
            || {
                let mut stack = Stack::<T, CAP>::new();
                stack.push(T::default());
                stack
            },
            |mut stack| {
                for _ in 0..STACK_ITERS {
                    black_box(stack.top_mut());
                }
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

pub fn bench_small_state_iteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("stack/small_state/iteration");
    for len in [16_usize, 128, 255] {
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

        group.bench_function(BenchmarkId::new("reverse", len), |b| {
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
                    for item in stack.iter().rev() {
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
