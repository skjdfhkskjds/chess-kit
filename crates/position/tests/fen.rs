use chess_kit_attack_table::DefaultAttackTable;
use chess_kit_position::{DefaultPosition, Fen, PositionView, Setup};
use chess_kit_primitives::{Pieces, Sides, Square};

const START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[test]
fn converts_a_validated_fen_into_a_position() {
    let fen = Fen::try_from(START_POSITION).unwrap();
    let setup = Setup::from(fen);
    let position = DefaultPosition::<DefaultAttackTable>::from(setup);

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
fn setup_defaults_to_the_start_position() {
    let setup = Setup::default();

    assert_eq!(setup.side_to_move(), Sides::White);
    assert_eq!(
        setup.pieces()[Square::E1],
        Some((Sides::White, Pieces::King))
    );
    assert_eq!(
        setup.pieces()[Square::E8],
        Some((Sides::Black, Pieces::King))
    );
}
