use chess_kit_perft::{Depth, NodeCount};
use std::fmt::{self, Display};

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
        writeln!(f, "position")?;
        writeln!(f, "  fen:      {}", self.fen)?;
        writeln!(f, "  depth:    {}", self.depth)
    }
}

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
