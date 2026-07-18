use crate::position::DefaultPosition;
use chess_kit_attack_table::AttackTable;
use chess_kit_primitives::{File, Pieces, Rank, Sides, call_as};
use std::{fmt, iter::once};

impl<AT> fmt::Display for DefaultPosition<AT>
where
    AT: AttackTable,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board = [['.'; File::TOTAL]; Rank::TOTAL];

        // construct the Unicode representation of the board
        for (side_idx, bitboards) in self.bitboards.iter().enumerate() {
            let side = Sides::from_idx(side_idx);
            for (piece_idx, bitboard) in bitboards.iter().enumerate() {
                let piece = Pieces::from_idx(piece_idx);
                let piece_char = call_as!(side, |SideT| piece.display::<SideT>().to_string())
                    .chars()
                    .next()
                    .expect("piece displays are never empty");

                for square in bitboard.iter() {
                    board[square.rank()][square.file()] = piece_char;
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
                board[rank_idx]
                    .into_iter()
                    .flat_map(|c| once(' ').chain(once(c)))
                    .skip(1)
                    .collect::<String>()
            )?;
        }
        writeln!(f, "  A B C D E F G H")?;

        let state = self.state();
        writeln!(f, "{} to move", state.turn())?;
        writeln!(f, "Castling rights: {}", state.castling())?;
        match state.en_passant() {
            Some(square) => writeln!(f, "En passant square: {square}")?,
            None => writeln!(f, "En passant square: None")?,
        }
        writeln!(f, "Halfmove clock: {}", state.halfmoves())?;
        writeln!(f, "Fullmove clock: {}", state.fullmoves())?;
        writeln!(
            f,
            "Repetition distance: {}",
            state.draw_state().repetition()
        )?;
        writeln!(
            f,
            "Material draw: {}",
            state.draw_state().is_material_draw()
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chess_kit_attack_table::DefaultAttackTable;
    use chess_kit_primitives::{Black, PieceDisplay, White};

    #[test]
    fn displays_position_with_side_appropriate_unicode_pieces() {
        let position = DefaultPosition::<DefaultAttackTable>::default();
        let displayed = position.to_string();
        let board = displayed.lines().take(10).collect::<Vec<_>>();
        let black_back_rank = [
            PieceDisplay::<Black>::ROOK,
            PieceDisplay::<Black>::KNIGHT,
            PieceDisplay::<Black>::BISHOP,
            PieceDisplay::<Black>::QUEEN,
            PieceDisplay::<Black>::KING,
            PieceDisplay::<Black>::BISHOP,
            PieceDisplay::<Black>::KNIGHT,
            PieceDisplay::<Black>::ROOK,
        ]
        .join(" ");
        let white_back_rank = [
            PieceDisplay::<White>::ROOK,
            PieceDisplay::<White>::KNIGHT,
            PieceDisplay::<White>::BISHOP,
            PieceDisplay::<White>::QUEEN,
            PieceDisplay::<White>::KING,
            PieceDisplay::<White>::BISHOP,
            PieceDisplay::<White>::KNIGHT,
            PieceDisplay::<White>::ROOK,
        ]
        .join(" ");

        assert_eq!(
            board,
            [
                String::new(),
                format!("8 {black_back_rank}"),
                format!("7 {}", [PieceDisplay::<Black>::PAWN; 8].join(" ")),
                "6 . . . . . . . .".to_owned(),
                "5 . . . . . . . .".to_owned(),
                "4 . . . . . . . .".to_owned(),
                "3 . . . . . . . .".to_owned(),
                format!("2 {}", [PieceDisplay::<White>::PAWN; 8].join(" ")),
                format!("1 {white_back_rank}"),
                "  A B C D E F G H".to_owned(),
            ]
        );
    }
}
