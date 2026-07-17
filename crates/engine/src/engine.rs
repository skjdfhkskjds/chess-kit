use std::time::Instant;

use chess_kit_attack_table::DefaultAttackTable;
use chess_kit_eval::{Accumulator, DefaultAccumulator, EvalState, PSQTEvalState};
use chess_kit_movegen::{DefaultMoveGenerator, MoveGenerator};
use chess_kit_position::{DefaultPosition, Fen, PositionMoves, PositionView, Setup};
use chess_kit_primitives::{Depth, Move, MoveList, MoveType, Pieces, Sides, Square, White};
use chess_kit_search::{Negamax, SearchNode, iterative_deepening};
use chess_kit_transposition::{DefaultTranspositionTable, TranspositionTable};

use crate::{Board, Engine, EngineConfig, EngineError, PositionBase, SearchOutcome};

type EnginePosition = DefaultPosition<DefaultAttackTable>;
type EngineMoveGenerator = DefaultMoveGenerator<DefaultAttackTable>;
type EngineAccumulator = DefaultAccumulator<PSQTEvalState>;
type EngineTranspositionTable = DefaultTranspositionTable<SearchNode>;

/// `DefaultEngine` is the composed, protocol-agnostic chess engine session
///
/// It owns the live position, evaluation accumulator, transposition table, and
/// search algorithm. Presentation layers should call this API rather than
/// wiring toolkit crates themselves
///
/// @type
pub struct DefaultEngine {
    position: EnginePosition,
    move_generator: EngineMoveGenerator,
    accumulator: EngineAccumulator,
    transposition_table: EngineTranspositionTable,
    search: Negamax,
}

impl DefaultEngine {
    /// new creates an engine at the standard starting position
    ///
    /// @param: configuration - construction-time engine settings
    /// @return: initialized engine, or an engine error
    pub fn new(configuration: EngineConfig) -> Result<Self, EngineError> {
        let (position, accumulator) = Self::build_position(PositionBase::StartPos, &[])?;

        Ok(Self {
            position,
            move_generator: EngineMoveGenerator::new(),
            accumulator,
            transposition_table: EngineTranspositionTable::new(
                configuration.transposition_table_size_mb,
            ),
            search: Negamax::new(),
        })
    }

    /// name returns the engine name advertised to presentation adapters
    ///
    /// @return: engine name
    pub fn name(&self) -> &'static str {
        concat!("chess-kit ", env!("CARGO_PKG_VERSION"))
    }

    /// author returns the engine author advertised to presentation adapters
    ///
    /// @return: engine author
    pub fn author(&self) -> &'static str {
        "chess-kit contributors"
    }

    /// legal_moves returns the legal moves available in the current position
    ///
    /// @return: legal move list for the side to move
    pub fn legal_moves(&self) -> Vec<Move> {
        self.primitive_legal_moves().as_slice().to_vec()
    }

    /// primitive_legal_moves returns the movegen representation used internally
    ///
    /// @return: generated legal moves for the current position
    fn primitive_legal_moves(&self) -> MoveList {
        let mut legal_moves = MoveList::new();
        self.move_generator
            .generate_legal_moves(&self.position, &mut legal_moves);
        legal_moves
    }

    /// build_position builds a complete replacement engine position
    ///
    /// Building replacement state first prevents malformed updates from leaving
    /// the live engine partially updated
    ///
    /// @param: base - root position before applying moves
    /// @param: moves - ordered moves to apply
    /// @return: replacement position and accumulator, or an engine error
    fn build_position(
        base: PositionBase,
        moves: &[Move],
    ) -> Result<(EnginePosition, EngineAccumulator), EngineError> {
        let fen = match &base {
            PositionBase::StartPos => Fen::default(),
            PositionBase::Fen(fen) => Fen::try_from(fen.as_str())
                .map_err(|error| EngineError::new(format!("invalid FEN: {error}")))?,
        };

        let mut position = EnginePosition::from(Setup::from(fen));
        let eval = PSQTEvalState::from_position(&position);
        let mut accumulator = EngineAccumulator::new();
        accumulator.push(eval);
        let move_generator = EngineMoveGenerator::new();

        for requested_move in moves {
            let mv = find_legal_move(&move_generator, &position, *requested_move)?;
            let eval = accumulator.push_next();
            let delta = position.play_unchecked(mv);
            eval.apply(delta);
        }

        Ok((position, accumulator))
    }
}

impl Engine for DefaultEngine {
    /// @impl: Engine::name
    fn name(&self) -> &str {
        self.name()
    }

    /// @impl: Engine::author
    fn author(&self) -> &str {
        self.author()
    }

    /// @impl: Engine::board
    fn board(&self) -> Board {
        let side_to_move = self.position.turn();
        let white = self.position.occupancy::<White>();
        let mut board = Board::empty(side_to_move);
        for square in Square::ALL {
            let piece = match self.position.piece_at(square) {
                Pieces::None => continue,
                piece => piece,
            };
            board.set_piece(
                square,
                Some((
                    if white.has_square(square) {
                        Sides::White
                    } else {
                        Sides::Black
                    },
                    piece,
                )),
            );
        }
        board
    }

    /// @impl: Engine::new_game
    fn new_game(&mut self) -> Result<(), EngineError> {
        (self.position, self.accumulator) = Self::build_position(PositionBase::StartPos, &[])?;
        self.transposition_table.clear();
        self.search = Negamax::new();
        Ok(())
    }

    /// @impl: Engine::set_position
    fn set_position(&mut self, base: PositionBase, moves: &[Move]) -> Result<(), EngineError> {
        (self.position, self.accumulator) = Self::build_position(base, moves)?;
        Ok(())
    }

    /// @impl: Engine::play
    fn play(&mut self, requested_move: Move) -> Result<(), EngineError> {
        let mv = find_legal_move(&self.move_generator, &self.position, requested_move)?;
        let eval = self.accumulator.push_next();
        let delta = self.position.play_unchecked(mv);
        eval.apply(delta);
        Ok(())
    }

    /// @impl: Engine::search
    fn search(&mut self, depth: Depth) -> Result<SearchOutcome, EngineError> {
        if depth < 1 {
            return Err(EngineError::new("search depth must be positive"));
        }

        let started = Instant::now();
        let result = iterative_deepening(
            &mut self.search,
            &mut self.position,
            &self.move_generator,
            &mut self.transposition_table,
            &mut self.accumulator,
            depth,
        );

        Ok(SearchOutcome {
            best_move: result.best_move,
            depth,
            score: result.score,
            nodes: result.nodes,
            elapsed: started.elapsed(),
        })
    }

    /// @impl: Engine::has_legal_moves
    fn has_legal_moves(&self) -> bool {
        !self.primitive_legal_moves().as_slice().is_empty()
    }
}

/// find_legal_move resolves an engine move against the current legal moves
///
/// @param: move_generator - move generator used to enumerate legal moves
/// @param: position - position to search from
/// @param: requested_move - protocol-neutral move requested by a caller
/// @return: matching legal move, or an engine error
fn find_legal_move(
    move_generator: &EngineMoveGenerator,
    position: &EnginePosition,
    requested_move: Move,
) -> Result<Move, EngineError> {
    let mut legal_moves = MoveList::new();
    move_generator.generate_legal_moves(position, &mut legal_moves);
    legal_moves
        .as_slice()
        .iter()
        .copied()
        .find(|mv| requested_move_matches(*mv, requested_move))
        .ok_or_else(|| EngineError::new("illegal move"))
}

/// requested_move_matches compares an adapter move with a generated legal move
///
/// Adapters cannot infer internal castle and en-passant flags from coordinate
/// notation, so non-promotion moves match by source and destination. Promotions
/// additionally require the same promoted piece
///
/// @param: legal_move - generated legal move containing complete move metadata
/// @param: requested_move - move supplied through the engine boundary
/// @return: true when both values describe the same playable move
fn requested_move_matches(legal_move: Move, requested_move: Move) -> bool {
    if legal_move.from() != requested_move.from() || legal_move.to() != requested_move.to() {
        return false;
    }

    match legal_move.type_of() {
        MoveType::Promotion => {
            requested_move.type_of() == MoveType::Promotion
                && legal_move.promoted_to() == requested_move.promoted_to()
        }
        _ => requested_move.type_of() != MoveType::Promotion,
    }
}

#[cfg(test)]
mod tests {
    use chess_kit_position::PositionView;
    use chess_kit_primitives::{Pieces, Sides, Square as BoardSquare};

    use super::*;

    const TEST_TRANSPOSITION_TABLE_SIZE_MB: usize = 1;

    fn engine() -> DefaultEngine {
        DefaultEngine::new(EngineConfig::new(TEST_TRANSPOSITION_TABLE_SIZE_MB)).unwrap()
    }

    #[test]
    fn position_applies_legal_move_history() {
        let mut engine = engine();
        engine
            .set_position(
                PositionBase::StartPos,
                &[
                    Move::new(Square::E2, Square::E4),
                    Move::new(Square::E7, Square::E5),
                ],
            )
            .unwrap();

        assert_eq!(engine.position.piece_at(BoardSquare::E4), Pieces::Pawn);
        assert_eq!(engine.position.piece_at(BoardSquare::E5), Pieces::Pawn);
        assert_eq!(engine.position.turn(), Sides::White);
    }

    #[test]
    fn search_returns_a_legal_move() {
        let mut engine = engine();
        let outcome = engine.search(1).unwrap();

        assert!(outcome.best_move.is_some());
        assert_eq!(outcome.depth, 1);
        assert!(outcome.nodes > 1);
    }

    #[test]
    fn play_rejects_illegal_moves_without_changing_turn() {
        let mut engine = engine();
        let turn = engine.position.turn();

        let illegal = Move::new(Square::E2, Square::E5);
        assert!(engine.play(illegal).is_err());
        assert_eq!(engine.position.turn(), turn);
    }

    #[test]
    fn play_resolves_coordinate_only_castling_moves() {
        let mut engine = engine();
        engine
            .set_position(
                PositionBase::Fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1".to_owned()),
                &[],
            )
            .unwrap();

        engine.play(Move::new(Square::E1, Square::G1)).unwrap();

        assert_eq!(engine.position.piece_at(Square::G1), Pieces::King);
        assert_eq!(engine.position.piece_at(Square::F1), Pieces::Rook);
    }

    #[test]
    fn search_rejects_non_positive_depths() {
        let mut engine = engine();

        assert!(engine.search(0).is_err());
        assert!(engine.search(-1).is_err());
    }

    #[test]
    fn has_legal_moves_is_true_from_the_start_position() {
        let engine = engine();
        assert!(engine.has_legal_moves());
    }

    #[test]
    fn board_exposes_a_protocol_neutral_position_snapshot() {
        let engine = engine();
        let board = engine.board();

        assert_eq!(board.side_to_move(), Sides::White);
        assert_eq!(
            board.piece_at(Square::E1),
            Some((Sides::White, Pieces::King))
        );
    }
}
