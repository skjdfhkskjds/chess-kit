use crate::fen::FENError;
use chess_kit_primitives::{Black, Castling, Clock, Pieces, Sides, Square, White};
use std::str::FromStr;

type PieceOnSquare = Option<(Sides, Pieces)>;

/// Setup is a validated position decoded from Forsyth-Edwards Notation
///
/// Setup separates FEN parsing and validation from construction of a concrete position
/// implementation
///
/// @type
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Setup {
    pieces: [PieceOnSquare; Square::TOTAL],
    side_to_move: Sides,
    castling: Castling,
    en_passant: Option<Square>,
    halfmoves: Clock,
    fullmoves: Clock,
}

impl Setup {
    pub fn pieces(&self) -> &[Option<(Sides, Pieces)>; Square::TOTAL] {
        &self.pieces
    }

    pub const fn side_to_move(&self) -> Sides {
        self.side_to_move
    }

    pub const fn castling(&self) -> Castling {
        self.castling
    }

    pub const fn en_passant(&self) -> Option<Square> {
        self.en_passant
    }

    pub const fn halfmoves(&self) -> Clock {
        self.halfmoves
    }

    pub const fn fullmoves(&self) -> Clock {
        self.fullmoves
    }

    fn parse_pieces(segment: &str) -> Result<[PieceOnSquare; Square::TOTAL], FENError> {
        let ranks = segment.split('/').collect::<Vec<_>>();
        if ranks.len() != 8 {
            return Err(FENError::InvalidPieces);
        }

        let mut pieces = [None; Square::TOTAL];
        let mut king_counts = [0_u8; Sides::TOTAL];

        for (fen_rank, rank) in ranks.iter().enumerate() {
            let board_rank = 7 - fen_rank;
            let mut file = 0;

            for character in rank.chars() {
                if let Some(empty_squares) = character.to_digit(10) {
                    if !(1..=8).contains(&empty_squares) {
                        return Err(FENError::InvalidPieces);
                    }
                    file += empty_squares as usize;
                    if file > 8 {
                        return Err(FENError::InvalidPieces);
                    }
                    continue;
                }

                if file >= 8 {
                    return Err(FENError::InvalidPieces);
                }

                let (side, piece) = match character {
                    'P' => (Sides::White, Pieces::Pawn),
                    'N' => (Sides::White, Pieces::Knight),
                    'B' => (Sides::White, Pieces::Bishop),
                    'R' => (Sides::White, Pieces::Rook),
                    'Q' => (Sides::White, Pieces::Queen),
                    'K' => (Sides::White, Pieces::King),
                    'p' => (Sides::Black, Pieces::Pawn),
                    'n' => (Sides::Black, Pieces::Knight),
                    'b' => (Sides::Black, Pieces::Bishop),
                    'r' => (Sides::Black, Pieces::Rook),
                    'q' => (Sides::Black, Pieces::Queen),
                    'k' => (Sides::Black, Pieces::King),
                    _ => return Err(FENError::InvalidPieces),
                };

                pieces[board_rank * 8 + file] = Some((side, piece));
                if piece == Pieces::King {
                    king_counts[side] += 1;
                }
                file += 1;
            }

            if file != 8 {
                return Err(FENError::InvalidPieces);
            }
        }

        // Position initialization assumes exactly one king for each side.
        if king_counts != [1, 1] {
            return Err(FENError::InvalidPieces);
        }

        Ok(pieces)
    }

    fn parse_side_to_move(segment: &str) -> Result<Sides, FENError> {
        match segment {
            "w" => Ok(Sides::White),
            "b" => Ok(Sides::Black),
            _ => Err(FENError::InvalidTurn),
        }
    }

    fn parse_castling(segment: &str) -> Result<Castling, FENError> {
        if segment == "-" {
            return Ok(Castling::none());
        }
        if segment.is_empty() || segment.len() > 4 || segment.contains('-') {
            return Err(FENError::InvalidCastling);
        }

        let mut castling = Castling::none();
        let mut seen = [false; 4];
        for character in segment.chars() {
            let index = match character {
                'K' => {
                    castling = castling.with_kingside::<White>();
                    0
                }
                'Q' => {
                    castling = castling.with_queenside::<White>();
                    1
                }
                'k' => {
                    castling = castling.with_kingside::<Black>();
                    2
                }
                'q' => {
                    castling = castling.with_queenside::<Black>();
                    3
                }
                _ => return Err(FENError::InvalidCastling),
            };
            if seen[index] {
                return Err(FENError::InvalidCastling);
            }
            seen[index] = true;
        }

        Ok(castling)
    }

    #[rustfmt::skip]
    fn parse_en_passant(segment: &str) -> Result<Option<Square>, FENError> {
        if segment == "-" {
            return Ok(None);
        }

        let square = Square::try_from(segment).map_err(|_| FENError::InvalidEnPassant)?;
        match square {
            Square::A3
            | Square::B3
            | Square::C3
            | Square::D3
            | Square::E3
            | Square::F3
            | Square::G3
            | Square::H3
            | Square::A6
            | Square::B6
            | Square::C6
            | Square::D6
            | Square::E6
            | Square::F6
            | Square::G6
            | Square::H6 => Ok(Some(square)),
            _ => Err(FENError::InvalidEnPassant),
        }
    }
}

impl FromStr for Setup {
    type Err = FENError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let segments = value.split_whitespace().collect::<Vec<_>>();
        if segments.len() != 6 {
            return Err(FENError::InvalidFormat);
        }

        let pieces = Self::parse_pieces(segments[0])?;
        let side_to_move = Self::parse_side_to_move(segments[1])?;
        let castling = Self::parse_castling(segments[2])?;
        let en_passant = Self::parse_en_passant(segments[3])?;
        let halfmoves = segments[4]
            .parse::<Clock>()
            .map_err(|_| FENError::InvalidHalfmoveCount)?;
        let fullmoves = segments[5]
            .parse::<Clock>()
            .map_err(|_| FENError::InvalidFullmoveCount)?;
        if fullmoves == 0 {
            return Err(FENError::InvalidFullmoveCount);
        }

        Ok(Self {
            pieces,
            side_to_move,
            castling,
            en_passant,
            halfmoves,
            fullmoves,
        })
    }
}

impl TryFrom<&str> for Setup {
    type Error = FENError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl TryFrom<String> for Setup {
    type Error = FENError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// Fen is the backwards-compatible name for a validated [`Setup`]
///
/// Fen allows existing callers to use the previous public name while position construction
/// transitions to the more general setup terminology
///
/// @type
pub type Fen = Setup;

#[cfg(test)]
mod tests {
    use super::*;

    const START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    #[test]
    fn parses_a_valid_fen() {
        let fen = Fen::try_from(START_POSITION).unwrap();

        assert_eq!(fen.side_to_move(), Sides::White);
        assert_eq!(fen.pieces()[Square::A1], Some((Sides::White, Pieces::Rook)));
        assert_eq!(fen.pieces()[Square::E8], Some((Sides::Black, Pieces::King)));
        assert_eq!(fen.fullmoves(), 1);
    }

    #[test]
    fn rejects_malformed_piece_placement() {
        for placement in [
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBN",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR/8",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR9",
        ] {
            let input = format!("{placement} w KQkq - 0 1");
            assert!(matches!(Fen::try_from(input), Err(FENError::InvalidPieces)));
        }
    }

    #[test]
    fn rejects_invalid_castling_and_clocks() {
        assert!(matches!(
            Fen::try_from(START_POSITION.replace("KQkq", "K-")),
            Err(FENError::InvalidCastling)
        ));
        assert!(matches!(
            Fen::try_from(START_POSITION.replace("0 1", "0 0")),
            Err(FENError::InvalidFullmoveCount)
        ));
    }

    #[test]
    fn clocks_use_the_u32_primitive() {
        let fen = Fen::try_from(START_POSITION.replace("0 1", "65536 65537")).unwrap();

        assert_eq!(fen.halfmoves(), 65_536);
        assert_eq!(fen.fullmoves(), 65_537);
    }
}
