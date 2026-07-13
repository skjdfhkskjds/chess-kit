use crate::common::run_report::PerftRunResult;
use crate::common::{PerftCase, PerftRunError, PerftRunReport};
use chess_kit_eval::{Accumulator, EvalState};
use chess_kit_movegen::MoveGenerator;
use chess_kit_perft::{PerftData, perft, perft_divide_print};
use chess_kit_position::Position;
use chess_kit_transposition::TranspositionTable;
use std::marker::PhantomData;
use std::time::Instant;

const DEFAULT_TT_SIZE: usize = 0;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum PerftHarnessMode {
    Default,
    Divide,
}

pub struct PerftHarness<MoveGeneratorT, PositionT, AccumulatorT, EvalStateT, TranspositionTableT>
where
    MoveGeneratorT: MoveGenerator,
    PositionT: Position,
    AccumulatorT: Accumulator<EvalStateT>,
    EvalStateT: EvalState,
    TranspositionTableT: TranspositionTable<PerftData>,
{
    mode: PerftHarnessMode,
    fail_fast: bool,
    test_cases: Vec<PerftCase>,
    move_generator: MoveGeneratorT,
    accumulator: AccumulatorT,
    tt: TranspositionTableT,
    position: PositionT,
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
    pub fn new(mode: PerftHarnessMode, test_cases: Vec<PerftCase>, tt_size: Option<usize>) -> Self {
        let tt_size = tt_size.unwrap_or(DEFAULT_TT_SIZE);

        Self {
            mode,
            fail_fast: false,
            test_cases,
            move_generator: MoveGeneratorT::new(),
            accumulator: AccumulatorT::new(),
            tt: TranspositionTableT::new(tt_size),
            position: PositionT::new(),
            _eval_state: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub fn with_fail_fast(mut self, fail_fast: bool) -> Self {
        self.fail_fast = fail_fast;
        self
    }
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
    pub fn run(&mut self) -> Vec<PerftRunReport> {
        let total = self.test_cases.len();
        let tests = self.test_cases.clone();
        let mut reports = Vec::with_capacity(total);
        let mut failures = Vec::new();

        for (case_index, test) in tests.iter().enumerate() {
            let index = case_index + 1;
            println!(
                "================================================================================"
            );
            println!();
            println!("perft {index}/{total}");
            println!("{test}");
            println!();

            match self.run_test(test) {
                Ok(report) => {
                    println!("{report}");

                    if report.passed() {
                        reports.push(report);
                    } else {
                        failures.push(format!(
                            "case {index}/{total}: expected {} nodes, got {} for {test}",
                            test.expected_nodes, report.nodes
                        ));
                        reports.push(report);

                        if self.fail_fast {
                            break;
                        }
                    }
                }
                Err(error) => {
                    println!("{error}");
                    failures.push(format!(
                        "case {index}/{total}: {} for {test}",
                        error.summary()
                    ));

                    if self.fail_fast {
                        break;
                    }
                }
            }
        }

        if !failures.is_empty() {
            panic!("{}", Self::format_failure_summary(&failures));
        }

        reports
    }

    fn run_test(&mut self, test: &PerftCase) -> PerftRunResult {
        self.position.reset();
        self.accumulator.reset();
        self.tt.clear();

        let eval = self
            .position
            .load_fen::<EvalStateT>(&test.fen)
            .map_err(|err| PerftRunError::LoadFen {
                fen: test.fen.clone(),
                source: err.to_string(),
            })?;
        self.accumulator.push(eval);

        let started_at = Instant::now();
        let nodes = match self.mode {
            PerftHarnessMode::Default => perft(
                &mut self.position,
                &self.move_generator,
                &mut self.tt,
                &mut self.accumulator,
                test.depth,
            ),
            PerftHarnessMode::Divide => perft_divide_print(
                &mut self.position,
                &self.move_generator,
                &mut self.tt,
                &mut self.accumulator,
                test.depth,
            ),
        };
        let elapsed = started_at.elapsed();
        let elapsed_secs = elapsed.as_secs_f64();
        let nodes_per_second = if elapsed_secs == 0.0 {
            nodes as f64
        } else {
            nodes as f64 / elapsed_secs
        };

        let tt_usage_percent = self.tt.usage_percent();

        Ok(PerftRunReport {
            expected_nodes: test.expected_nodes,
            nodes,
            elapsed,
            nodes_per_second,
            tt_usage_percent,
        })
    }

    fn format_failure_summary(failures: &[String]) -> String {
        let failure_count = failures.len();
        let plural = if failure_count == 1 { "" } else { "s" };
        let mut summary = format!("perft suite failed: {failure_count} failure{plural}");

        for failure in failures {
            summary.push_str("\n- ");
            summary.push_str(failure);
        }

        summary
    }
}
