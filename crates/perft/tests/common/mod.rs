use chess_kit_eval::{Accumulator, EvalState};
use chess_kit_movegen::MoveGenerator;
use chess_kit_perft::{Depth, NodeCount, PerftData, perft, perft_divide_print};
use chess_kit_position::Position;
use chess_kit_primitives::ZobristKey;
use chess_kit_transposition::{NodeData, TranspositionTable};
use std::fmt::{self, Display};
use std::marker::PhantomData;
use std::time::{Duration, Instant};

const DEFAULT_TT_SIZE: usize = 0;

#[derive(Debug, Clone)]
pub struct PerftCase {
    pub fen: String,
    pub depth: Depth,
    pub expected_nodes: NodeCount,
}

impl TryFrom<&str> for PerftCase {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.split('|').collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err("expected format: <FEN>|<depth>|<leaf nodes>".to_string());
        }

        let depth = parts[1]
            .parse::<Depth>()
            .map_err(|err| format!("invalid depth '{}': {err}", parts[1]))?;
        let expected_nodes = parts[2]
            .parse::<NodeCount>()
            .map_err(|err| format!("invalid node count '{}': {err}", parts[2]))?;

        Ok(Self {
            fen: parts[0].trim().to_string(),
            depth,
            expected_nodes,
        })
    }
}

impl Display for PerftCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FEN: {}\nPerft Data: depth={}, nodes={}",
            self.fen, self.depth, self.expected_nodes
        )
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum PerftHarnessMode {
    Default,
    Divide,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct PerftRunReport {
    pub nodes: NodeCount,
    pub elapsed: Duration,
    pub nodes_per_second: f64,
    pub tt_usage_percent: u16,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum PerftRunError {
    LoadFen { fen: String, source: String },
}

impl Display for PerftRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LoadFen { fen, source } => {
                write!(f, "error loading FEN '{fen}': {source}")
            }
        }
    }
}

type PerftRunResult = Result<PerftRunReport, PerftRunError>;

pub fn load_cases(input: &str) -> Vec<PerftCase> {
    input
        .lines()
        .enumerate()
        .filter_map(|(line_number, line)| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                None
            } else {
                Some(PerftCase::try_from(line).unwrap_or_else(|err| {
                    panic!("invalid perft fixture line {}: {err}", line_number + 1)
                }))
            }
        })
        .collect()
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
            test_cases,
            move_generator: MoveGeneratorT::new(),
            accumulator: AccumulatorT::new(),
            tt: TranspositionTableT::new(tt_size),
            position: PositionT::new(),
            _eval_state: PhantomData,
        }
    }



    pub fn run(&mut self) -> Vec<PerftRunReport> {
        let tests = self.test_cases.clone();
        let mut reports = Vec::with_capacity(self.test_cases.len());

        for test in tests {
            let report = self
                .run_test(&test)
                .unwrap_or_else(|err| panic!("perft failed for {test}: {err}"));
            assert_eq!(
                report.nodes, test.expected_nodes,
                "perft mismatch for {test}"
            );
            reports.push(report);
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
            nodes,
            elapsed,
            nodes_per_second,
            tt_usage_percent,
        })
    }
}
