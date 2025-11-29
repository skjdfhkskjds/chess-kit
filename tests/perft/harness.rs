use crate::perft::{PerftTest, perft, perft_divide_print};
use chess_kit::board::Board;
use chess_kit::movegen::MoveGenerator;
use std::time::Instant;

pub enum PerftHarnessMode {
    Default,
    Divide,
}

// PerftHarness is a test harness for running perft tests
pub struct PerftHarness {
    mode: PerftHarnessMode,        // the mode to run the harness in
    test_cases: Vec<PerftTest>,    // the test cases to run
    move_generator: MoveGenerator, // global move generator, shared across tests
    board: Board,                  // global board, shared across tests
}

impl PerftHarness {
    // new creates a new perft harness
    //
    // @param: test_cases - the test cases to run
    // @return: a new perft harness
    pub fn new(mode: PerftHarnessMode, test_cases: Vec<PerftTest>) -> Self {
        Self {
            mode,
            test_cases,
            move_generator: MoveGenerator::new(),
            board: Board::new(),
        }
    }

    // run_test runs a single test case
    //
    // @param: test - the test case to run
    fn run_test(&mut self, test: &PerftTest) {
        // setup the board from the FEN string
        let board = Board::try_from(test.fen);
        if board.is_err() {
            println!(
                "Error: {} in parsing the FEN-string, skipping...",
                board.err().unwrap()
            );
            return;
        }
        self.board = board.unwrap();

        // run the test case per depth
        for (depth, expected_nodes) in test.iter() {
            let now = Instant::now();
            let nodes = match self.mode {
                PerftHarnessMode::Default => perft(&mut self.board, &self.move_generator, depth),
                PerftHarnessMode::Divide => {
                    perft_divide_print(&mut self.board, &self.move_generator, depth)
                }
            };

            // output test results
            let elapsed = now.elapsed().as_millis();
            let moves_per_second = ((nodes * 1000) as f64 / elapsed as f64).floor();
            print!("Depth {}: {} ", depth, nodes);
            println!("[{}ms, {} moves/s]", elapsed, moves_per_second);

            // assert the results
            assert_eq!(nodes, expected_nodes);
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
        }
    }
}
