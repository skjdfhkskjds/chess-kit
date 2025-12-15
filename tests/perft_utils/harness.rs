use crate::perft_utils::PerftTest;
use chess_kit::eval::{Accumulator, EvalState};
use chess_kit::movegen::MoveGenerator;
use chess_kit::perft::{PerftData, perft, perft_divide_print};
use chess_kit::position::Position;
use chess_kit::transposition::TranspositionTable;
use std::marker::PhantomData;
use std::time::Instant;

#[cfg(feature = "no_tt")]
const TT_SIZE: usize = 0;

#[cfg(not(feature = "no_tt"))]
const TT_SIZE: usize = 32;

#[allow(dead_code)]
pub enum PerftHarnessMode {
    Default,
    Divide,
}

// PerftHarness is a test harness for running perft tests
pub struct PerftHarness<MoveGeneratorT, PositionT, AccumulatorT, EvalStateT, TranspositionTableT>
where
    MoveGeneratorT: MoveGenerator,
    PositionT: Position,
    AccumulatorT: Accumulator<EvalStateT>,
    EvalStateT: EvalState,
    TranspositionTableT: TranspositionTable<PerftData>,
{
    mode: PerftHarnessMode,         // the mode to run the harness in
    test_cases: Vec<PerftTest>,     // the test cases to run
    move_generator: MoveGeneratorT, // global move generator, shared across tests
    accumulator: AccumulatorT,      // global accumulator, shared across tests
    tt: TranspositionTableT,        // global transposition table, shared across tests
    position: PositionT,            // global position, shared across tests

    _eval_state: PhantomData<EvalStateT>,
}

impl<MoveGeneratorT, PositionT, AccumulatorT, EvalStateT, TranspositionTableT>
    PerftHarness<MoveGeneratorT, PositionT, AccumulatorT, EvalStateT, TranspositionTableT>
where
    MoveGeneratorT: MoveGenerator,
    PositionT: Position,
    AccumulatorT: Accumulator<EvalStateT>,
    EvalStateT: EvalState,
    TranspositionTableT: TranspositionTable<PerftData>,
{
    // new creates a new perft harness
    //
    // @param: test_cases - the test cases to run
    // @return: a new perft harness
    pub fn new(mode: PerftHarnessMode, test_cases: Vec<PerftTest>) -> Self {
        let tt = TranspositionTableT::new(TT_SIZE);

        Self {
            mode,
            test_cases,
            move_generator: MoveGeneratorT::new(),
            accumulator: AccumulatorT::new(),
            tt,
            position: PositionT::new(),
            _eval_state: PhantomData,
        }
    }

    // run_test runs a single test case
    //
    // @param: test - the test case to run
    fn run_test(&mut self, test: &PerftTest) {
        // clear old state
        self.position.reset();
        self.accumulator.reset();

        // setup the board from the FEN string
        let result = self.position.load_fen(test.fen);
        if result.is_err() {
            println!(
                "Error: {} in parsing the FEN-string, skipping...",
                result.err().unwrap()
            );
            return;
        }
        self.accumulator.init(&self.position);

        // run the test case per depth
        let now = Instant::now();
        let nodes = match self.mode {
            PerftHarnessMode::Default => perft(
                &mut self.position,
                &self.move_generator,
                &mut self.tt,
                &mut self.accumulator,
                test.data.depth(),
            ),
            PerftHarnessMode::Divide => perft_divide_print(
                &mut self.position,
                &self.move_generator,
                &mut self.tt,
                &mut self.accumulator,
                test.data.depth(),
            ),
        };

        // output test results
        let elapsed = now.elapsed().as_millis();
        let moves_per_second = ((nodes * 1000) as f64 / elapsed as f64).floor();
        print!("Depth {}: {} ", test.data.depth(), nodes);
        print!("[{}ms, {} moves/s] ", elapsed, moves_per_second);
        println!("[tt usage: {}%]", self.tt.usage_percent());

        // assert the results
        assert_eq!(nodes, test.data.node_count());
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
