use chess_kit_perft::NodeCount;
use std::fmt::{self, Display};
use std::time::Duration;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PerftRunReport {
    pub expected_nodes: NodeCount,
    pub nodes: NodeCount,
    pub elapsed: Duration,
    pub nodes_per_second: f64,
    pub tt_usage_percent: u16,
}

impl PerftRunReport {
    pub fn passed(&self) -> bool {
        self.nodes == self.expected_nodes
    }
}

impl Display for PerftRunReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.passed() { "ok" } else { "mismatch" };

        writeln!(f, "result")?;
        writeln!(f, "  status:   {status}")?;
        writeln!(f, "  expected: {}", self.expected_nodes)?;
        writeln!(f, "  actual:   {}", self.nodes)?;
        writeln!(f, "  elapsed:  {:?}", self.elapsed)?;
        writeln!(f, "  speed:    {:.0} nodes/s", self.nodes_per_second)?;
        writeln!(f, "  tt usage: {}%", self.tt_usage_percent)
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum PerftRunError {
    LoadFen { fen: String, source: String },
}

impl PerftRunError {
    pub fn summary(&self) -> String {
        match self {
            Self::LoadFen { fen, source } => format!("error loading FEN '{fen}': {source}"),
        }
    }
}

impl Display for PerftRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "status: error")?;
        writeln!(f, "  error:  {}", self.summary())
    }
}

pub(super) type PerftRunResult = Result<PerftRunReport, PerftRunError>;
