use chess_kit_collections::{FixedArray, Retain};
use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;

const CAPACITY: usize = 256;

fn populated() -> FixedArray<u64, CAPACITY> {
    core::array::from_fn(|index| index as u64).into()
}

fn push_pop(c: &mut Criterion) {
    let mut group = c.benchmark_group("fixed_array/push_pop");
    group.throughput(Throughput::Elements(CAPACITY as u64));

    group.bench_function("checked", |b| {
        b.iter(|| {
            let mut array = FixedArray::<u64, CAPACITY>::new();
            let len = black_box(CAPACITY as u64);
            for value in 0..len {
                array.push(black_box(value));
            }
            while let Some(value) = array.pop() {
                black_box(value);
            }
        });
    });

    group.bench_function("unchecked", |b| {
        b.iter(|| {
            let mut array = FixedArray::<u64, CAPACITY>::new();
            let len = black_box(CAPACITY as u64);
            for value in 0..len {
                // SAFETY: `black_box` does not change the value, so this loop
                // writes exactly `CAPACITY` elements.
                unsafe { array.push_unchecked(black_box(value)) };
            }
            while let Some(value) = array.pop() {
                black_box(value);
            }
        });
    });

    group.finish();
}

fn iteration(c: &mut Criterion) {
    let array = populated();
    let mut group = c.benchmark_group("fixed_array/iteration");
    group.throughput(Throughput::Elements(CAPACITY as u64));

    group.bench_function("forward", |b| {
        b.iter(|| {
            let sum = array
                .iter()
                .fold(0_u64, |sum, value| sum.wrapping_add(black_box(*value)));
            black_box(sum)
        });
    });

    group.bench_function("reverse", |b| {
        b.iter(|| {
            let sum = array
                .iter()
                .rev()
                .fold(0_u64, |sum, value| sum.wrapping_add(black_box(*value)));
            black_box(sum)
        });
    });

    group.finish();
}

fn retain(c: &mut Criterion) {
    let mut group = c.benchmark_group("fixed_array/retain");
    group.throughput(Throughput::Elements(CAPACITY as u64));

    group.bench_function("all", |b| {
        b.iter_batched(
            populated,
            |mut array| {
                array.retain(|value| black_box(*value) < CAPACITY as u64);
                black_box(array.len())
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("half", |b| {
        b.iter_batched(
            populated,
            |mut array| {
                array.retain(|value| black_box(*value) & 1 == 0);
                black_box(array.len())
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(benches, push_pop, iteration, retain);
criterion_main!(benches);
