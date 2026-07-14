use chess_kit_attack_table::DefaultAttackTable;
use chess_kit_position::{DefaultPosition, Fen, PositionMoves, PositionView, Setup};
use chess_kit_primitives::{Move, MoveDelta, PieceDelta, Pieces, Sides, Square};

type TestPosition = DefaultPosition<DefaultAttackTable>;

const START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn load(fen: &str) -> TestPosition {
    Setup::from(Fen::try_from(fen).unwrap()).into()
}

fn changes(delta: MoveDelta) -> Vec<PieceDelta> {
    delta.iter().collect()
}

#[test]
fn move_deltas_cover_every_legal_move_shape_in_deterministic_order() {
    let mut normal = load(START_POSITION);
    assert_eq!(
        changes(normal.play_unchecked(Move::new(Square::E2, Square::E4))),
        vec![
            PieceDelta::removed(Sides::White, Pieces::Pawn, Square::E2),
            PieceDelta::added(Sides::White, Pieces::Pawn, Square::E4),
        ]
    );

    let mut capture = load("7k/8/8/8/8/2p5/1B6/7K w - - 0 1");
    assert_eq!(
        changes(capture.play_unchecked(Move::new(Square::B2, Square::C3))),
        vec![
            PieceDelta::removed(Sides::Black, Pieces::Pawn, Square::C3),
            PieceDelta::removed(Sides::White, Pieces::Bishop, Square::B2),
            PieceDelta::added(Sides::White, Pieces::Bishop, Square::C3),
        ]
    );

    let mut promotion = load("7k/P7/8/8/8/8/8/7K w - - 0 1");
    assert_eq!(
        changes(
            promotion
                .play_unchecked(Move::new(Square::A7, Square::A8).with_promotion(Pieces::Knight),),
        ),
        vec![
            PieceDelta::removed(Sides::White, Pieces::Pawn, Square::A7),
            PieceDelta::added(Sides::White, Pieces::Knight, Square::A8),
        ]
    );

    let mut en_passant = load("7k/8/8/3pP3/8/8/8/7K w - d6 0 1");
    assert_eq!(
        changes(en_passant.play_unchecked(Move::new(Square::E5, Square::D6).with_en_passant()),),
        vec![
            PieceDelta::removed(Sides::Black, Pieces::Pawn, Square::D5),
            PieceDelta::removed(Sides::White, Pieces::Pawn, Square::E5),
            PieceDelta::added(Sides::White, Pieces::Pawn, Square::D6),
        ]
    );

    let mut castle = load("4k3/8/8/8/8/8/8/R3K2R w KQ - 0 1");
    assert_eq!(
        changes(castle.play_unchecked(Move::new(Square::E1, Square::G1).with_castle())),
        vec![
            PieceDelta::removed(Sides::White, Pieces::King, Square::E1),
            PieceDelta::removed(Sides::White, Pieces::Rook, Square::H1),
            PieceDelta::added(Sides::White, Pieces::King, Square::G1),
            PieceDelta::added(Sides::White, Pieces::Rook, Square::F1),
        ]
    );
}

#[test]
fn initializes_material_draw_state_from_fen() {
    let kings = load("8/8/8/8/8/8/4k3/7K w - - 0 1");
    let start = load(START_POSITION);

    assert!(kings.draw_state().is_material_draw());
    assert!(kings.is_draw_by_insufficient_material());
    assert!(!start.draw_state().is_material_draw());
    assert!(!start.is_draw_by_insufficient_material());
}

#[test]
fn updates_and_restores_material_draw_state_after_capture() {
    let mut position = load("7k/8/8/8/8/2p5/1B6/7K w - - 0 1");
    let capture = Move::new(Square::B2, Square::C3);

    assert!(!position.draw_state().is_material_draw());

    let _ = position.play_unchecked(capture);
    assert!(position.draw_state().is_material_draw());
    assert_eq!(position.piece_at(Square::C3), Pieces::Bishop);

    position.undo(capture);
    assert!(!position.draw_state().is_material_draw());
    assert_eq!(position.piece_at(Square::C3), Pieces::Pawn);
}

#[test]
fn updates_and_restores_material_draw_state_after_promotion() {
    let mut position = load("7k/P7/8/8/8/8/8/7K w - - 0 1");
    let promotion = Move::new(Square::A7, Square::A8).with_promotion(Pieces::Knight);

    assert!(!position.draw_state().is_material_draw());

    let _ = position.play_unchecked(promotion);
    assert!(position.draw_state().is_material_draw());
    assert_eq!(position.piece_at(Square::A8), Pieces::Knight);

    position.undo(promotion);
    assert!(!position.draw_state().is_material_draw());
    assert_eq!(position.piece_at(Square::A7), Pieces::Pawn);
}

#[test]
fn tracks_repetition_incrementally_and_restores_it_on_unmake() {
    let mut position = load(START_POSITION);
    let cycle = [
        Move::new(Square::G1, Square::F3),
        Move::new(Square::G8, Square::F6),
        Move::new(Square::F3, Square::G1),
        Move::new(Square::F6, Square::G8),
    ];

    for mv in cycle {
        let _ = position.play_unchecked(mv);
    }

    assert_eq!(position.draw_state().repetition(), 4);
    assert!(!position.is_draw_by_repetition());
    assert!(!position.is_draw_by_repetition_in_search(4));
    assert!(position.is_draw_by_repetition_in_search(5));

    for mv in cycle {
        let _ = position.play_unchecked(mv);
    }

    assert_eq!(position.draw_state().repetition(), -4);
    assert!(position.is_draw_by_repetition());
    assert!(position.is_draw_by_repetition_in_search(0));

    position.undo(cycle[3]);
    assert_eq!(position.draw_state().repetition(), 4);
    assert!(!position.is_draw_by_repetition());
}

#[test]
fn zeroing_move_clears_repetition_state() {
    let mut position = load(START_POSITION);
    let cycle = [
        Move::new(Square::G1, Square::F3),
        Move::new(Square::G8, Square::F6),
        Move::new(Square::F3, Square::G1),
        Move::new(Square::F6, Square::G8),
    ];

    for mv in cycle {
        let _ = position.play_unchecked(mv);
    }
    assert_eq!(position.draw_state().repetition(), 4);

    let _ = position.play_unchecked(Move::new(Square::E2, Square::E4));
    assert_eq!(position.draw_state().repetition(), 0);
}
