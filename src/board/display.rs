use crate::board::Board;
use crate::primitives::{Files, Piece, Ranks, Sides};
use std::fmt;

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board = [['.'; Ranks::TOTAL]; Files::TOTAL];

        // construct the ascii representation of the board
        for (side, bitboards) in self.bitboards.iter().enumerate() {
            for (piece, bitboard) in bitboards.iter().enumerate() {
                for file in (Files::A..=Files::H).rev() {
                    for rank in (Ranks::R1..=Ranks::R8).rev() {
                        if !((bitboard >> ((rank * 8) as u32) + (file as u32)) & 1).is_empty() {
                            let piece_str = format!("{}", Piece::new(piece));
                            board[rank][file] = match side {
                                Sides::WHITE => piece_str.chars().next().unwrap(),
                                Sides::BLACK => {
                                    piece_str.chars().next().unwrap().to_ascii_lowercase()
                                }
                                _ => return Err(fmt::Error),
                            };
                        }
                    }
                }
            }
        }

        // print the board
        for rank in (Ranks::R1..=Ranks::R8).rev() {
            writeln!(
                f,
                "{} {}",
                rank + 1,
                board[rank as usize].iter().collect::<String>()
            )?;
        }
        writeln!(f, "  ABCDEFGH")?;

        // print the game state metadata
        writeln!(f, "{}", self.state)?;

        Ok(())
    }
}
