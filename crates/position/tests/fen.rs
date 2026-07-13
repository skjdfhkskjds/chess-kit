use chess_kit_attack_table::DefaultAttackTable;
use chess_kit_position::{DefaultPosition, DefaultState, Fen, PositionState};
use chess_kit_primitives::{Pieces, Sides, Square};

const START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[test]
fn converts_a_validated_fen_into_a_position() {
    let fen = Fen::try_from(START_POSITION).unwrap();
    let position = DefaultPosition::<DefaultAttackTable, DefaultState>::try_from(fen).unwrap();

    assert_eq!(position.turn(), Sides::White);
    assert_eq!(position.piece_at(Square::A1), Pieces::Rook);
    assert_eq!(position.piece_at(Square::E8), Pieces::King);
}
