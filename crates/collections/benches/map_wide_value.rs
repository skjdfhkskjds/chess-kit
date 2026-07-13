mod fixtures;
mod utils;

use criterion::{Criterion, criterion_group, criterion_main};
use fixtures::WideValue;

fn layout_benches(c: &mut Criterion) {
    let reports = [
        utils::layout::map_report::<WideValue>("map_wide_1mib", 1),
        utils::layout::map_report::<WideValue>("map_wide_8mib", 8),
    ];

    utils::layout::bench_layout_reports(c, "map/wide_value/layout_report_generation", &reports);
}

fn performance_benches(c: &mut Criterion) {
    utils::map::bench_construction::<WideValue>(c, "map/wide_value/construction");
    utils::map::bench_allocation_cost::<WideValue>(c, "map/wide_value/allocation_cost");
}

criterion_group!(benches, layout_benches, performance_benches);
criterion_main!(benches);
