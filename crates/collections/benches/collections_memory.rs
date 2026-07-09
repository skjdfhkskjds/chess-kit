mod fixtures;

use chess_kit_collections::{Map, Stack};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use fixtures::{CompactValue, HistoryState, SmallState, SplitU64Hasher, WideValue};
use std::hint::black_box;
use std::mem::{align_of, size_of};

const MIB: usize = 1024 * 1024;

#[derive(Clone, Copy)]
struct LayoutReport {
    name: &'static str,
    requested_mib: usize,
    type_size: usize,
    type_align: usize,
    buckets: usize,
    capacity: usize,
    payload_bytes: usize,
    requested_bytes: usize,
}

impl LayoutReport {
    #[inline]
    fn overhead_bytes(self) -> isize {
        self.requested_bytes as isize - self.payload_bytes as isize
    }

    #[inline]
    fn payload_efficiency(self) -> f64 {
        if self.requested_bytes == 0 {
            0.0
        } else {
            self.payload_bytes as f64 / self.requested_bytes as f64
        }
    }
}

fn map_report<V>(name: &'static str, requested_mib: usize) -> LayoutReport
where
    V: chess_kit_collections::Value,
{
    let map = Map::<u64, V, SplitU64Hasher>::new(requested_mib);
    LayoutReport {
        name,
        requested_mib,
        type_size: size_of::<Map<u64, V, SplitU64Hasher>>(),
        type_align: align_of::<Map<u64, V, SplitU64Hasher>>(),
        buckets: map.buckets(),
        capacity: map.capacity(),
        payload_bytes: map.capacity() * size_of::<V>(),
        requested_bytes: requested_mib * MIB,
    }
}

fn print_report(report: LayoutReport) {
    println!(
        "memory_report name={} requested_mib={} type_size={} type_align={} buckets={} capacity={} payload_bytes={} requested_bytes={} overhead_bytes={} payload_efficiency={:.4}",
        report.name,
        report.requested_mib,
        report.type_size,
        report.type_align,
        report.buckets,
        report.capacity,
        report.payload_bytes,
        report.requested_bytes,
        report.overhead_bytes(),
        report.payload_efficiency(),
    );
}

fn bench_layout_reports(c: &mut Criterion) {
    let reports = [
        LayoutReport {
            name: "stack_small_cap_8",
            requested_mib: 0,
            type_size: size_of::<Stack<SmallState, 8>>(),
            type_align: align_of::<Stack<SmallState, 8>>(),
            buckets: 0,
            capacity: 8,
            payload_bytes: 8 * size_of::<SmallState>(),
            requested_bytes: 0,
        },
        LayoutReport {
            name: "stack_small_cap_255",
            requested_mib: 0,
            type_size: size_of::<Stack<SmallState, 255>>(),
            type_align: align_of::<Stack<SmallState, 255>>(),
            buckets: 0,
            capacity: 255,
            payload_bytes: 255 * size_of::<SmallState>(),
            requested_bytes: 0,
        },
        LayoutReport {
            name: "stack_history_cap_255",
            requested_mib: 0,
            type_size: size_of::<Stack<HistoryState, 255>>(),
            type_align: align_of::<Stack<HistoryState, 255>>(),
            buckets: 0,
            capacity: 255,
            payload_bytes: 255 * size_of::<HistoryState>(),
            requested_bytes: 0,
        },
        map_report::<CompactValue>("map_compact_1mib", 1),
        map_report::<CompactValue>("map_compact_8mib", 8),
        map_report::<WideValue>("map_wide_1mib", 1),
        map_report::<WideValue>("map_wide_8mib", 8),
    ];

    for report in reports {
        print_report(report);
    }

    c.bench_function("memory/layout_report_generation", |b| {
        b.iter(|| {
            let mut acc = 0_usize;
            for report in reports {
                acc ^= black_box(report.type_size);
                acc ^= black_box(report.capacity);
                acc ^= black_box(report.payload_bytes);
            }
            black_box(acc)
        });
    });
}

fn bench_map_allocation_cost(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory/map_allocation_cost");
    for mib in [0_usize, 1, 8, 32] {
        group.bench_function(BenchmarkId::new("compact", mib), |b| {
            b.iter(|| {
                black_box(Map::<u64, CompactValue, SplitU64Hasher>::new(black_box(
                    mib,
                )))
            });
        });
        group.bench_function(BenchmarkId::new("wide", mib), |b| {
            b.iter(|| black_box(Map::<u64, WideValue, SplitU64Hasher>::new(black_box(mib))));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_layout_reports, bench_map_allocation_cost);
criterion_main!(benches);
