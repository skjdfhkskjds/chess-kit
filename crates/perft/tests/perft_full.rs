mod common;

use chess_kit_attack_table::DefaultAttackTable;
use chess_kit_eval::{DefaultAccumulator, NoOpEvalState};
use chess_kit_movegen::DefaultMoveGenerator;
use chess_kit_perft::PerftData;
use chess_kit_position::{DefaultPosition, DefaultState};
use chess_kit_transposition::DefaultTranspositionTable;
use common::{PerftHarness, PerftHarnessMode, load_cases};

#[test]
#[ignore = "run explicitly as the full perft regression suite"]
fn perft_full_suite() {
    let test_cases = load_cases(include_str!("fixtures/perft_full.epd"));
    let total = test_cases.len();
    let mut harness = PerftHarness::<
        DefaultMoveGenerator<DefaultAttackTable>,
        DefaultPosition<DefaultAttackTable, DefaultState>,
        DefaultAccumulator<NoOpEvalState>,
        NoOpEvalState,
        DefaultTranspositionTable<PerftData>,
    >::new(PerftHarnessMode::Default, test_cases, None);

    let reports = harness.run();
    for (index, report) in reports.iter().enumerate() {
        println!(
            "perft_full {}/{}: {} nodes [{:?}, {:.0} nodes/s, tt usage: {}%]",
            index + 1,
            total,
            report.nodes,
            report.elapsed,
            report.nodes_per_second,
            report.tt_usage_percent
        );
    }
}
