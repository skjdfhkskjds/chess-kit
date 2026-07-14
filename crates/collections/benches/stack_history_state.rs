mod fixtures;
mod utils;

use criterion::{Criterion, criterion_group, criterion_main};
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
    utils::stack::bench_construction::<HistoryState, 255>(c, "stack/history_state/construction");
    utils::stack::bench_push_pop::<HistoryState, 255>(
        c,
        "stack/history_state/push_pop",
        HistoryState::with_seed,
    );
    utils::stack::bench_push_next::<HistoryState, 255>(
        c,
        "stack/history_state/push_next",
        HistoryState::update_metadata,
    );
    utils::stack::bench_push_next_pop_pair::<HistoryState, 255>(
        c,
        "stack/history_state/push_next_pop_pair",
        HistoryState::update_metadata,
    );
    utils::stack::bench_top_access::<HistoryState, 255>(c, "stack/history_state/top_access");
}

criterion_group!(benches, layout_benches, performance_benches);
criterion_main!(benches);
