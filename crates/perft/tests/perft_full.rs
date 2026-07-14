mod common;

use chess_kit_attack_table::DefaultAttackTable;
use chess_kit_eval::{DefaultAccumulator, NoOpEvalState};
use chess_kit_movegen::DefaultMoveGenerator;
use chess_kit_perft::PerftData;
use chess_kit_position::DefaultPosition;
use chess_kit_transposition::DefaultTranspositionTable;
use common::{PerftHarness, PerftHarnessMode, load_cases};

#[test]
#[ignore = "run explicitly as the full perft regression suite"]
fn perft_full_suite() {
    let test_cases = load_cases(include_str!("fixtures/perft_full.epd"));
    let mut harness = PerftHarness::<
        DefaultMoveGenerator<DefaultAttackTable>,
        DefaultPosition<DefaultAttackTable>,
        DefaultAccumulator<NoOpEvalState>,
        NoOpEvalState,
        DefaultTranspositionTable<PerftData>,
    >::new(PerftHarnessMode::Default, test_cases, None);

    harness.run();
}
