use chess_kit_attack_table::DefaultAttackTable;
use chess_kit_collections::Copyable;
use chess_kit_eval::{Accumulator, DefaultAccumulator, EvalState, Score};
use chess_kit_movegen::{DefaultMoveGenerator, MoveGenerator};
use chess_kit_position::{DefaultPosition, PositionView};
use chess_kit_primitives::{Move, MoveDelta, PieceDeltaKind, Pieces, Sides, Square};
use chess_kit_search::Negamax;

type TestPosition = DefaultPosition<DefaultAttackTable>;
type TestMoveGenerator = DefaultMoveGenerator<DefaultAttackTable>;
type TestAccumulator = DefaultAccumulator<MaterialEvalState>;

#[derive(Copy, Clone, Default)]
struct MaterialEvalState {
    score: Score,
}

impl MaterialEvalState {
    const fn piece_value(piece: Pieces) -> Score {
        match piece {
            Pieces::Queen => 900,
            Pieces::Rook => 500,
            Pieces::Bishop | Pieces::Knight => 300,
            Pieces::Pawn => 100,
            Pieces::King | Pieces::None => 0,
        }
    }
}

impl EvalState for MaterialEvalState {
    fn from_position<P: PositionView>(position: &P) -> Self {
        let mut state = Self::default();
        for piece in Pieces::ALL {
            state.score += position
                .get_piece::<chess_kit_primitives::White>(piece)
                .count_ones() as Score
                * Self::piece_value(piece);
            state.score -= position
                .get_piece::<chess_kit_primitives::Black>(piece)
                .count_ones() as Score
                * Self::piece_value(piece);
        }
        state
    }

    fn apply(&mut self, delta: MoveDelta) {
        for change in delta.iter() {
            let value = Self::piece_value(change.piece());
            let sign =
                match (change.side(), change.kind()) {
                    (Sides::White, PieceDeltaKind::Added)
                    | (Sides::Black, PieceDeltaKind::Removed) => 1,
                    (Sides::White, PieceDeltaKind::Removed)
                    | (Sides::Black, PieceDeltaKind::Added) => -1,
                };
            self.score += sign * value;
        }
    }

    fn score(&mut self) -> Score {
        self.score
    }
}

impl Copyable for MaterialEvalState {
    fn copy_from(&mut self, other: &Self) {
        *self = *other;
    }
}

fn load(fen: &str) -> (TestPosition, TestMoveGenerator, TestAccumulator) {
    let position = fen.parse::<TestPosition>().unwrap();
    let eval = MaterialEvalState::from_position(&position);
    let mut accumulator = TestAccumulator::new();
    accumulator.push(eval);

    (position, TestMoveGenerator::new(), accumulator)
}

#[test]
fn depth_zero_returns_side_to_move_evaluation() {
    let (mut position, move_generator, mut accumulator) = load("4k3/8/8/8/8/8/8/3QK3 b - - 0 1");

    let result = Negamax::new().search(&mut position, &move_generator, &mut accumulator, 0);

    assert_eq!(result.best_move, None);
    assert_eq!(result.score, -900);
    assert_eq!(result.nodes, 1);
}

#[test]
fn selects_an_immediately_winning_capture() {
    let (mut position, move_generator, mut accumulator) = load("4k3/8/8/8/8/8/4q3/3Q2K1 w - - 0 1");

    let result = Negamax::new().search(&mut position, &move_generator, &mut accumulator, 1);

    assert_eq!(result.best_move, Some(Move::new(Square::D1, Square::E2)));
    assert_eq!(result.score, 900);
}

#[test]
fn scores_checkmate_and_stalemate() {
    let (mut checkmate, move_generator, mut checkmate_accumulator) =
        load("7k/6Q1/6K1/8/8/8/8/8 b - - 0 1");
    let mate_result = Negamax::new().search(
        &mut checkmate,
        &move_generator,
        &mut checkmate_accumulator,
        1,
    );

    assert_eq!(mate_result.best_move, None);
    assert_eq!(mate_result.score, -Negamax::CHECKMATE_SCORE);

    let (mut stalemate, move_generator, mut stalemate_accumulator) =
        load("7k/5Q2/6K1/8/8/8/8/8 b - - 0 1");
    let stalemate_result = Negamax::new().search(
        &mut stalemate,
        &move_generator,
        &mut stalemate_accumulator,
        1,
    );

    assert_eq!(stalemate_result.best_move, None);
    assert_eq!(stalemate_result.score, 0);
}

#[test]
fn search_restores_state_and_prunes_the_tree() {
    let (mut position, move_generator, mut accumulator) =
        load("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let original_key = position.key();
    let original_score = accumulator.latest_mut().score();

    let result = Negamax::new().search(&mut position, &move_generator, &mut accumulator, 2);

    assert_eq!(position.key(), original_key);
    assert_eq!(accumulator.latest_mut().score(), original_score);
    assert!(result.nodes < 421, "expected alpha-beta pruning");
}
