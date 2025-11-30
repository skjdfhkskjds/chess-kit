use std::convert::TryFrom;
use std::fmt::{self, Display};

pub type Depth = u8;
pub type LeafNodes = usize;

// PerftTest is an object representing a single Perft test-case
// 
// Within each test-case, there are sub-cases defined as a pair of
//   <depth, expected leaf nodes>
#[derive(Debug, Default, Clone)]
pub struct PerftTest {
    pub(crate) fen: &'static str,  // the FEN string for the test case
    data: Vec<(Depth, LeafNodes)>, // data vector containing <depth, expected leaf nodes> pairs
}

impl PerftTest {
    pub fn iter(&self) -> impl Iterator<Item = (Depth, LeafNodes)> {
        self.data.iter().copied()
    }
}

impl TryFrom<&'static str> for PerftTest {
    type Error = &'static str;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        let parts = value.split(';').collect::<Vec<&str>>();
        if parts.len() < 2 {
            return Err("Invalid perft case");
        }

        // the first part is the FEN string
        let fen = parts[0].trim();

        // the remaining parts are the depth data
        let data = parts[1..].iter().map(|part| {
            // each part should be of the form 'D<depth> <leaf nodes>'
            let parts = part.split(' ').collect::<Vec<&str>>();

            // parse the depth
            // 
            // Note: ignore the prefix 'D'
            let depth = (parts[0][1..]).parse::<Depth>().unwrap();

            // parse the expected number of leaf nodes
            let leaf_nodes = parts[1].parse::<LeafNodes>().unwrap();

            (depth, leaf_nodes)
        }).collect::<Vec<(Depth, LeafNodes)>>();

        Ok(Self { fen, data })
    }
}

impl Display for PerftTest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // print the FEN string
        writeln!(f, "FEN: {}", self.fen)?;

        // print the perft data
        write!(f, "Perft Data: ")?;
        for (depth, leaf_nodes) in &self.data {
            write!(f, "{depth}: {leaf_nodes} ")?;
        }
        Ok(())
    }
}
