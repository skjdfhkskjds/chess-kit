mod fixtures;
mod support;

use criterion::{criterion_group, criterion_main, Criterion};
use fixtures::CompactValue;

fn layout_benches(c: &mut Criterion) {
    let reports = [
        support::layout::map_report::<CompactValue>("map_compact_1mib", 1),
        support::layout::map_report::<CompactValue>("map_compact_8mib", 8),
    ];

    support::layout::bench_layout_reports(
        c,
        "map/compact_value/layout_report_generation",
        &reports,
    );
}

fn performance_benches(c: &mut Criterion) {
    support::map::bench_construction::<CompactValue>(c, "map/compact_value/construction");
    support::map::bench_allocation_cost::<CompactValue>(c, "map/compact_value/allocation_cost");
    support::map::bench_compact_operations(c);
}

criterion_group!(benches, layout_benches, performance_benches);
criterion_main!(benches);
