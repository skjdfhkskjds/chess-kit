use chess_kit_attack_table::DefaultAttackTable;
use chess_kit_collections::Copyable;
use chess_kit_eval::{Accumulator, DefaultAccumulator, EvalState, Score};
use chess_kit_movegen::{DefaultMoveGenerator, MoveGenerator};
use chess_kit_position::{DefaultPosition, Fen, PositionView, Setup};
use chess_kit_primitives::{Move, MoveDelta, PieceDeltaKind, Pieces, Sides, Square};
use chess_kit_search::{Bound, Negamax, SearchNode, iterative_deepening};
use chess_kit_transposition::{DefaultTranspositionTable, TranspositionTable};

type TestPosition = DefaultPosition<DefaultAttackTable>;
type TestMoveGenerator = DefaultMoveGenerator<DefaultAttackTable>;
type TestAccumulator = DefaultAccumulator<MaterialEvalState>;
type TestTranspositionTable = DefaultTranspositionTable<SearchNode>;

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

fn load_with_table_size(
    fen: &str,
    table_size_mb: usize,
) -> (
    TestPosition,
    TestMoveGenerator,
    TestTranspositionTable,
    TestAccumulator,
) {
    let position = TestPosition::from(Setup::from(Fen::try_from(fen).unwrap()));
    let eval = MaterialEvalState::from_position(&position);
    let mut accumulator = TestAccumulator::new();
    accumulator.push(eval);

    (
        position,
        TestMoveGenerator::new(),
        TestTranspositionTable::new(table_size_mb),
        accumulator,
    )
}

fn load(
    fen: &str,
) -> (
    TestPosition,
    TestMoveGenerator,
    TestTranspositionTable,
    TestAccumulator,
) {
    load_with_table_size(fen, 1)
}

#[test]
fn depth_zero_returns_side_to_move_evaluation() {
    let (mut position, move_generator, mut transposition_table, mut accumulator) =
        load("4k3/8/8/8/8/8/8/3QK3 b - - 0 1");

    let result = iterative_deepening(
        &mut Negamax::new(),
        &mut position,
        &move_generator,
        &mut transposition_table,
        &mut accumulator,
        0,
    );

    assert_eq!(result.best_move, None);
    assert_eq!(result.score, -900);
    assert_eq!(result.nodes, 1);
}

#[test]
fn quiescence_resolves_captures_beyond_the_main_search_horizon() {
    let (mut position, move_generator, mut transposition_table, mut accumulator) =
        load("3q2k1/3Q4/8/8/8/8/8/6K1 b - - 0 1");
    let original_key = position.key();
    let original_score = accumulator.latest_mut().score();

    let result = Negamax::new().search(
        &mut position,
        &move_generator,
        &mut transposition_table,
        &mut accumulator,
        0,
    );

    assert_eq!(result.best_move, None);
    assert_eq!(result.score, 900);
    assert_eq!(result.nodes, 2);
    assert_eq!(position.key(), original_key);
    assert_eq!(accumulator.latest_mut().score(), original_score);
}

#[test]
fn quiescence_searches_quiet_check_evasions() {
    let (mut position, move_generator, mut transposition_table, mut accumulator) =
        load("4k3/8/8/8/8/8/8/4R1K1 b - - 0 1");
    let original_key = position.key();

    let result = Negamax::new().search(
        &mut position,
        &move_generator,
        &mut transposition_table,
        &mut accumulator,
        0,
    );

    assert_eq!(result.best_move, None);
    assert_eq!(result.score, -500);
    assert!(result.nodes > 1);
    assert_eq!(position.key(), original_key);
}

#[test]
fn quiescence_scores_terminal_positions_at_depth_zero() {
    let (mut checkmate, move_generator, mut checkmate_table, mut checkmate_accumulator) =
        load("7k/6Q1/6K1/8/8/8/8/8 b - - 0 1");
    let mate_result = Negamax::new().search(
        &mut checkmate,
        &move_generator,
        &mut checkmate_table,
        &mut checkmate_accumulator,
        0,
    );

    assert_eq!(mate_result.best_move, None);
    assert_eq!(mate_result.score, -Negamax::CHECKMATE_SCORE);
    assert_eq!(mate_result.nodes, 1);

    let (mut stalemate, move_generator, mut stalemate_table, mut stalemate_accumulator) =
        load("7k/5Q2/6K1/8/8/8/8/8 b - - 0 1");
    let stalemate_result = Negamax::new().search(
        &mut stalemate,
        &move_generator,
        &mut stalemate_table,
        &mut stalemate_accumulator,
        0,
    );

    assert_eq!(stalemate_result.best_move, None);
    assert_eq!(stalemate_result.score, 0);
    assert_eq!(stalemate_result.nodes, 1);
}

#[test]
fn selects_an_immediately_winning_capture() {
    let (mut position, move_generator, mut transposition_table, mut accumulator) =
        load("4k3/8/8/8/8/8/4q3/3Q2K1 w - - 0 1");

    let result = Negamax::new().search(
        &mut position,
        &move_generator,
        &mut transposition_table,
        &mut accumulator,
        1,
    );

    assert_eq!(result.best_move, Some(Move::new(Square::D1, Square::E2)));
    assert_eq!(result.score, 900);
}

#[test]
fn scores_checkmate_and_stalemate() {
    let (mut checkmate, move_generator, mut checkmate_table, mut checkmate_accumulator) =
        load("7k/6Q1/6K1/8/8/8/8/8 b - - 0 1");
    let mut search = Negamax::new();
    let mate_result = search.search(
        &mut checkmate,
        &move_generator,
        &mut checkmate_table,
        &mut checkmate_accumulator,
        1,
    );

    assert_eq!(mate_result.best_move, None);
    assert_eq!(mate_result.score, -Negamax::CHECKMATE_SCORE);
    let cached_mate = search.search(
        &mut checkmate,
        &move_generator,
        &mut checkmate_table,
        &mut checkmate_accumulator,
        1,
    );
    assert_eq!(cached_mate.score, mate_result.score);
    assert_eq!(cached_mate.nodes, 1);

    let (mut stalemate, move_generator, mut stalemate_table, mut stalemate_accumulator) =
        load("7k/5Q2/6K1/8/8/8/8/8 b - - 0 1");
    let stalemate_result = search.search(
        &mut stalemate,
        &move_generator,
        &mut stalemate_table,
        &mut stalemate_accumulator,
        1,
    );

    assert_eq!(stalemate_result.best_move, None);
    assert_eq!(stalemate_result.score, 0);
    let cached_stalemate = search.search(
        &mut stalemate,
        &move_generator,
        &mut stalemate_table,
        &mut stalemate_accumulator,
        1,
    );
    assert_eq!(cached_stalemate.score, stalemate_result.score);
    assert_eq!(cached_stalemate.nodes, 1);
}

#[test]
fn iterative_deepening_visits_each_depth_and_stores_the_final_root() {
    let (mut position, move_generator, mut transposition_table, mut accumulator) =
        load("7k/6Q1/6K1/8/8/8/8/8 b - - 0 1");
    let root_key = position.key();
    let mut search = Negamax::new();

    let result = iterative_deepening(
        &mut search,
        &mut position,
        &move_generator,
        &mut transposition_table,
        &mut accumulator,
        3,
    );

    assert_eq!(result.best_move, None);
    assert_eq!(result.score, -Negamax::CHECKMATE_SCORE);
    assert_eq!(result.nodes, 3);

    let root = transposition_table.probe(root_key).copied().unwrap();
    assert_eq!(root.depth(), 3);
    assert_eq!(root.bound(), Bound::Exact);
    assert_eq!(root.best_move(), None);
}

#[test]
fn search_restores_state_and_prunes_the_tree() {
    let (mut position, move_generator, mut transposition_table, mut accumulator) =
        load("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let original_key = position.key();
    let original_score = accumulator.latest_mut().score();

    let result = Negamax::new().search(
        &mut position,
        &move_generator,
        &mut transposition_table,
        &mut accumulator,
        2,
    );

    assert_eq!(position.key(), original_key);
    assert_eq!(accumulator.latest_mut().score(), original_score);
    assert!(result.nodes < 421, "expected alpha-beta pruning");
}

#[test]
fn repeated_search_uses_the_cached_root_node() {
    let (mut position, move_generator, mut transposition_table, mut accumulator) =
        load("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let mut search = Negamax::new();
    let original_key = position.key();
    let original_score = accumulator.latest_mut().score();

    let first = search.search(
        &mut position,
        &move_generator,
        &mut transposition_table,
        &mut accumulator,
        3,
    );
    let second = search.search(
        &mut position,
        &move_generator,
        &mut transposition_table,
        &mut accumulator,
        3,
    );

    assert_eq!(second.best_move, first.best_move);
    assert_eq!(second.score, first.score);
    assert_eq!(second.nodes, 1);
    assert_eq!(position.key(), original_key);
    assert_eq!(accumulator.latest_mut().score(), original_score);
}

#[test]
fn shallow_cache_entry_does_not_answer_a_deeper_search() {
    let fen = "4k3/8/8/8/8/8/4q3/3Q2K1 w - - 0 1";
    let (mut position, move_generator, mut transposition_table, mut accumulator) = load(fen);
    let mut search = Negamax::new();

    search.search(
        &mut position,
        &move_generator,
        &mut transposition_table,
        &mut accumulator,
        1,
    );
    let cached = search.search(
        &mut position,
        &move_generator,
        &mut transposition_table,
        &mut accumulator,
        2,
    );

    let (mut fresh_position, fresh_move_generator, mut fresh_table, mut fresh_accumulator) =
        load(fen);
    let fresh = Negamax::new().search(
        &mut fresh_position,
        &fresh_move_generator,
        &mut fresh_table,
        &mut fresh_accumulator,
        2,
    );

    assert!(cached.nodes > 1);
    assert_eq!(cached.best_move, fresh.best_move);
    assert_eq!(cached.score, fresh.score);
}

#[test]
fn transposition_table_reduces_nodes_in_a_cold_search() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let (
        mut uncached_position,
        uncached_move_generator,
        mut disabled_table,
        mut uncached_accumulator,
    ) = load_with_table_size(fen, 0);
    let uncached = Negamax::new().search(
        &mut uncached_position,
        &uncached_move_generator,
        &mut disabled_table,
        &mut uncached_accumulator,
        4,
    );

    let (mut cached_position, cached_move_generator, mut enabled_table, mut cached_accumulator) =
        load(fen);
    let cached = Negamax::new().search(
        &mut cached_position,
        &cached_move_generator,
        &mut enabled_table,
        &mut cached_accumulator,
        4,
    );

    assert_eq!(cached.best_move, uncached.best_move);
    assert_eq!(cached.score, uncached.score);
    assert!(
        cached.nodes < uncached.nodes,
        "expected transposition caching to reduce nodes: cached={}, uncached={}",
        cached.nodes,
        uncached.nodes,
    );
}
