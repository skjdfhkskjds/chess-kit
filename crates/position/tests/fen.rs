use chess_kit_attack_table::DefaultAttackTable;
use chess_kit_position::{DefaultPosition, Fen, PositionView};
use chess_kit_primitives::{Pieces, Sides, Square};

const START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[test]
fn converts_a_validated_fen_into_a_position() {
    let fen = Fen::try_from(START_POSITION).unwrap();
    let position = DefaultPosition::<DefaultAttackTable>::try_from(fen).unwrap();

    assert_eq!(position.turn(), Sides::White);
    assert_eq!(position.piece_at(Square::A1), Pieces::Rook);
    assert_eq!(position.piece_at(Square::E8), Pieces::King);
}

#[test]
fn default_constructs_the_start_position() {
    let position = DefaultPosition::<DefaultAttackTable>::default();

    assert_eq!(position.turn(), Sides::White);
    assert_eq!(position.piece_at(Square::E1), Pieces::King);
    assert_eq!(position.piece_at(Square::E8), Pieces::King);
}

#[test]
fn parses_a_position_directly_from_fen_text() {
    let position = START_POSITION
        .parse::<DefaultPosition<DefaultAttackTable>>()
        .unwrap();

    assert_eq!(position.piece_at(Square::A1), Pieces::Rook);
    assert_eq!(position.piece_at(Square::H8), Pieces::Rook);
}
