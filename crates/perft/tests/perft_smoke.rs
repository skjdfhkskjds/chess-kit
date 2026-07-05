mod common;

use chess_kit_attack_table::DefaultAttackTable;
use chess_kit_eval::{DefaultAccumulator, PSQTEvalState};
use chess_kit_movegen::DefaultMoveGenerator;
use chess_kit_perft::PerftData;
use chess_kit_position::{DefaultPosition, DefaultState};
use chess_kit_transposition::DefaultTranspositionTable;
use common::{PerftHarness, PerftHarnessMode, load_cases};

#[test]
fn perft_smoke_suite() {
    let test_cases = load_cases(include_str!("fixtures/perft_smoke.epd"));
    let mut harness = PerftHarness::<
        DefaultMoveGenerator<DefaultAttackTable>,
        DefaultPosition<DefaultAttackTable, DefaultState>,
        DefaultAccumulator<PSQTEvalState>,
        PSQTEvalState,
        DefaultTranspositionTable<PerftData>,
    >::new(PerftHarnessMode::Default, test_cases, None);

    harness.run();
}
