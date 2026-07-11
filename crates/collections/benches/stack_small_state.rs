mod fixtures;
mod utils;

use criterion::{criterion_group, criterion_main, Criterion};
use fixtures::SmallState;

fn layout_benches(c: &mut Criterion) {
    let reports = [
        utils::layout::stack_report::<SmallState, 8>("stack_small_cap_8"),
        utils::layout::stack_report::<SmallState, 64>("stack_small_cap_64"),
        utils::layout::stack_report::<SmallState, 255>("stack_small_cap_255"),
    ];

    utils::layout::bench_layout_reports(
        c,
        "stack/small_state/layout_report_generation",
        &reports,
    );
}

fn performance_benches(c: &mut Criterion) {
    utils::stack::bench_push_pop::<SmallState, 8>(
        c,
        "stack/small_state/push_pop",
        SmallState::with_seed,
    );
    utils::stack::bench_push_pop::<SmallState, 64>(
        c,
        "stack/small_state/push_pop",
        SmallState::with_seed,
    );
    utils::stack::bench_push_pop::<SmallState, 255>(
        c,
        "stack/small_state/push_pop",
        SmallState::with_seed,
    );
    utils::stack::bench_push_next::<SmallState, 255>(
        c,
        "stack/small_state/push_next",
        SmallState::update,
    );
    utils::stack::bench_small_state_iteration(c);
}

criterion_group!(benches, layout_benches, performance_benches);
criterion_main!(benches);
