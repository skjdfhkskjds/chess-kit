use crate::perft_utils::PerftTest;
use chess_kit::attack_table::{default_attack_table, DefaultAttackTable};
use chess_kit::movegen::MoveGenerator;
use chess_kit::perft::{PerftData, perft, perft_divide_print};
use chess_kit::position::Position;
use chess_kit::primitives::DefaultState;
use chess_kit::transposition::{DefaultTranspositionTable, TranspositionTable};
use std::time::Instant;

#[cfg(feature = "no_tt")]
const TT_SIZE: usize = 0;

#[cfg(not(feature = "no_tt"))]
const TT_SIZE: usize = 32;

pub enum PerftHarnessMode {
    Default,
    Divide,
}

// PerftHarness is a test harness for running perft tests
pub struct PerftHarness {
    mode: PerftHarnessMode,                   // the mode to run the harness in
    test_cases: Vec<PerftTest>,               // the test cases to run
    attack_table: &'static DefaultAttackTable, // global attack table, shared across tests
    move_generator: MoveGenerator<DefaultAttackTable>, // global move generator, shared across tests
    tt: DefaultTranspositionTable<PerftData>, // global transposition table, shared across tests
    position: Position<DefaultAttackTable, DefaultState>, // global position, shared across tests
}

impl PerftHarness {
    // new creates a new perft harness
    //
    // @param: test_cases - the test cases to run
    // @return: a new perft harness
    pub fn new(mode: PerftHarnessMode, test_cases: Vec<PerftTest>) -> Self {
        let tt = DefaultTranspositionTable::<PerftData>::new(TT_SIZE);
        println!(
            "tt config: [{} buckets, {} entries]",
            tt.buckets(),
            tt.capacity()
        );

        let attack_table = default_attack_table();
        Self {
            mode,
            test_cases,
            attack_table,
            move_generator: MoveGenerator::<DefaultAttackTable>::new(attack_table),
            tt,
            position: Position::<DefaultAttackTable, DefaultState>::new(attack_table),
        }
    }

    // run_test runs a single test case
    //
    // @param: test - the test case to run
    fn run_test(&mut self, test: &PerftTest) {
        // setup the board from the FEN string
        self.position = Position::<DefaultAttackTable, DefaultState>::new(self.attack_table);
        let result = self.position.load_fen(test.fen);
        if result.is_err() {
            println!(
                "Error: {} in parsing the FEN-string, skipping...",
                result.err().unwrap()
            );
            return;
        }

        // run the test case per depth
        for expected in test.iter() {
            let now = Instant::now();
            let nodes = match self.mode {
                PerftHarnessMode::Default => perft(
                    &mut self.position,
                    &self.move_generator,
                    &mut self.tt,
                    expected.depth(),
                ),
                PerftHarnessMode::Divide => perft_divide_print(
                    &mut self.position,
                    &self.move_generator,
                    &mut self.tt,
                    expected.depth(),
                ),
            };

            // output test results
            let elapsed = now.elapsed().as_millis();
            let moves_per_second = ((nodes * 1000) as f64 / elapsed as f64).floor();
            print!("Depth {}: {} ", expected.depth(), nodes);
            print!("[{}ms, {} moves/s] ", elapsed, moves_per_second);
            println!("[tt usage: {}%]", self.tt.usage_percent());

            // assert the results
            assert_eq!(nodes, expected.node_count());
        }
    }

    // run runs all the test cases
    //
    // @return: void
    pub fn run(&mut self) {
        let tests = self.test_cases.clone();
        let total = tests.len();
        for (i, test) in tests.into_iter().enumerate() {
            // dump the test run header
            println!("================================================");
            println!("Test {} of {}", i + 1, total);
            println!("{}", test);
            println!("================================================");

            // run the test case
            println!();
            self.run_test(&test);
            println!();

            // reset for the next test
            self.tt.clear();
        }
    }
}
