mod fixtures;
mod utils;

use criterion::{criterion_group, criterion_main, Criterion};
use fixtures::HistoryState;

fn layout_benches(c: &mut Criterion) {
    let reports = [utils::layout::stack_report::<HistoryState, 255>(
        "stack_history_cap_255",
    )];

    utils::layout::bench_layout_reports(
        c,
        "stack/history_state/layout_report_generation",
        &reports,
    );
}

fn performance_benches(c: &mut Criterion) {
    utils::stack::bench_push_pop::<HistoryState, 255>(
        c,
        "stack/history_state/push_pop",
        HistoryState::with_seed,
    );
    utils::stack::bench_push_next::<HistoryState, 255>(
        c,
        "stack/history_state/push_next",
        HistoryState::update_header,
    );
}

criterion_group!(benches, layout_benches, performance_benches);
criterion_main!(benches);
