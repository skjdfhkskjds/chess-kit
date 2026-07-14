use chess_kit_attack_table::DefaultAttackTable;
use chess_kit_eval::NoOpEvalState;
use chess_kit_position::{
    DefaultPosition, DefaultState, Position, PositionFromFEN, PositionMoves, PositionState,
};
use chess_kit_primitives::{Move, Pieces, Square};

type TestPosition = DefaultPosition<DefaultAttackTable, DefaultState>;

const START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn load(fen: &str) -> (TestPosition, NoOpEvalState) {
    let mut position = TestPosition::new();
    let eval = position.load_fen::<NoOpEvalState>(fen).unwrap();
    (position, eval)
}

#[test]
fn initializes_material_draw_state_from_fen() {
    let (kings, _) = load("8/8/8/8/8/8/4k3/7K w - - 0 1");
    let (start, _) = load(START_POSITION);

    assert!(kings.draw_state().is_material_draw());
    assert!(kings.is_draw_by_insufficient_material());
    assert!(!start.draw_state().is_material_draw());
    assert!(!start.is_draw_by_insufficient_material());
}

#[test]
fn updates_and_restores_material_draw_state_after_capture() {
    let (mut position, mut eval) = load("7k/8/8/8/8/2p5/1B6/7K w - - 0 1");
    let capture = Move::new(Square::B2, Square::C3);

    assert!(!position.draw_state().is_material_draw());

    position.make_move(capture, &mut eval);
    assert!(position.draw_state().is_material_draw());
    assert_eq!(position.piece_at(Square::C3), Pieces::Bishop);

    position.unmake_move(capture);
    assert!(!position.draw_state().is_material_draw());
    assert_eq!(position.piece_at(Square::C3), Pieces::Pawn);
}

#[test]
fn updates_and_restores_material_draw_state_after_promotion() {
    let (mut position, mut eval) = load("7k/P7/8/8/8/8/8/7K w - - 0 1");
    let promotion = Move::new(Square::A7, Square::A8).with_promotion(Pieces::Knight);

    assert!(!position.draw_state().is_material_draw());

    position.make_move(promotion, &mut eval);
    assert!(position.draw_state().is_material_draw());
    assert_eq!(position.piece_at(Square::A8), Pieces::Knight);

    position.unmake_move(promotion);
    assert!(!position.draw_state().is_material_draw());
    assert_eq!(position.piece_at(Square::A7), Pieces::Pawn);
}

#[test]
fn tracks_repetition_incrementally_and_restores_it_on_unmake() {
    let (mut position, mut eval) = load(START_POSITION);
    let cycle = [
        Move::new(Square::G1, Square::F3),
        Move::new(Square::G8, Square::F6),
        Move::new(Square::F3, Square::G1),
        Move::new(Square::F6, Square::G8),
    ];

    for mv in cycle {
        position.make_move(mv, &mut eval);
    }

    assert_eq!(position.draw_state().repetition(), 4);
    assert!(!position.is_draw_by_repetition());
    assert!(!position.is_draw_by_repetition_in_search(4));
    assert!(position.is_draw_by_repetition_in_search(5));

    for mv in cycle {
        position.make_move(mv, &mut eval);
    }

    assert_eq!(position.draw_state().repetition(), -4);
    assert!(position.is_draw_by_repetition());
    assert!(position.is_draw_by_repetition_in_search(0));

    position.unmake_move(cycle[3]);
    assert_eq!(position.draw_state().repetition(), 4);
    assert!(!position.is_draw_by_repetition());
}

#[test]
fn zeroing_move_clears_repetition_state() {
    let (mut position, mut eval) = load(START_POSITION);
    let cycle = [
        Move::new(Square::G1, Square::F3),
        Move::new(Square::G8, Square::F6),
        Move::new(Square::F3, Square::G1),
        Move::new(Square::F6, Square::G8),
    ];

    for mv in cycle {
        position.make_move(mv, &mut eval);
    }
    assert_eq!(position.draw_state().repetition(), 4);

    position.make_move(Move::new(Square::E2, Square::E4), &mut eval);
    assert_eq!(position.draw_state().repetition(), 0);
}
