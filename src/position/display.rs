use crate::attack_table::AttackTable;
use crate::position::DefaultPosition;
use crate::primitives::{File, GameStateExt, Pieces, Rank, Sides, State};
use std::fmt;

impl<AT, StateT> fmt::Display for DefaultPosition<AT, StateT>
where
    AT: AttackTable,
    StateT: State + GameStateExt,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board = [['.'; File::TOTAL]; Rank::TOTAL];

        // construct the ascii representation of the board
        for (side_idx, bitboards) in self.bitboards.iter().enumerate() {
            let side = Sides::from_idx(side_idx);
            for (piece_idx, bitboard) in bitboards.iter().enumerate() {
                let base_char = Pieces::from_idx(piece_idx)
                    .to_string()
                    .chars()
                    .next()
                    .unwrap();
                let piece_char = match side {
                    Sides::White => base_char,
                    Sides::Black => base_char.to_ascii_lowercase(),
                };

                for square in bitboard.iter() {
                    let file_idx = square.file().idx();
                    let rank_idx = square.rank().idx();
                    board[rank_idx][file_idx] = piece_char;
                }
            }
        }

        // print the board
        writeln!(f)?;
        for rank_idx in (0..Rank::TOTAL).rev() {
            writeln!(
                f,
                "{} {}",
                rank_idx + 1,
                board[rank_idx].iter().collect::<String>()
            )?;
        }
        writeln!(f, "  ABCDEFGH")?;

        // print the game state metadata
        writeln!(f, "{}", self.state())?;

        Ok(())
    }
}
