use chess_kit::perft::{Depth, NodeCount, PerftData};
use std::convert::TryFrom;
use std::fmt::{self, Display};

// PerftTest is an object representing a single Perft test-case
#[derive(Debug, Default, Clone)]
pub struct PerftTest {
    pub(crate) fen: &'static str, // the FEN string for the test case
    pub(crate) data: PerftData,   // data containing <depth, expected leaf nodes>
}

impl TryFrom<&'static str> for PerftTest {
    type Error = &'static str;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        let parts = value.split('|').collect::<Vec<&str>>();
        if parts.len() != 3 {
            return Err("Invalid perft test string, expected format: <FEN>|<depth>|<leaf nodes>");
        }

        // the first part is the FEN string
        let fen = parts[0].trim();

        // parse the depth
        //
        // Note: ignore the prefix 'D'
        let depth = (parts[1]).parse::<Depth>().unwrap();

        // parse the expected number of leaf nodes
        let leaf_nodes = parts[2].parse::<NodeCount>().unwrap();

        Ok(Self {
            fen,
            data: PerftData::new(depth, leaf_nodes),
        })
    }
}

impl Display for PerftTest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // print the FEN string
        writeln!(f, "FEN: {}", self.fen)?;

        // print the perft data
        write!(f, "Perft Data: ")?;
        write!(f, "{} ", self.data)?;
        Ok(())
    }
}
