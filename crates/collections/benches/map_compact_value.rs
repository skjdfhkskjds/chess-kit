mod fixtures;
mod utils;

use criterion::{Criterion, criterion_group, criterion_main};
use fixtures::CompactValue;

fn layout_benches(c: &mut Criterion) {
    let reports = [
        utils::layout::map_report::<CompactValue>("map_compact_1mib", 1),
        utils::layout::map_report::<CompactValue>("map_compact_8mib", 8),
    ];

    utils::layout::bench_layout_reports(c, "map/compact_value/layout_report_generation", &reports);
}

fn performance_benches(c: &mut Criterion) {
    utils::map::bench_construction::<CompactValue>(c, "map/compact_value/construction");
    utils::map::bench_allocation_cost::<CompactValue>(c, "map/compact_value/allocation_cost");
    utils::map::bench_compact_operations(c);
    utils::map::bench_disabled_operations(c);
    utils::map::bench_clear(c);
    utils::map::bench_bucket_probes(c);
}

criterion_group!(benches, layout_benches, performance_benches);
criterion_main!(benches);
