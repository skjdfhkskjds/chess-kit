use std::fmt::{self, Display};
use std::time::Instant;

use chess_kit_attack_table::DefaultAttackTable;
use chess_kit_eval::{Accumulator, DefaultAccumulator, EvalState, PSQTEvalState};
use chess_kit_movegen::{DefaultMoveGenerator, MoveGenerator};
use chess_kit_position::{DefaultPosition, Fen, PositionMoves, Setup};
use chess_kit_primitives::{Depth, Move, MoveList};
use chess_kit_search::{Negamax, SearchNode, iterative_deepening};
use chess_kit_transposition::{DefaultTranspositionTable, TranspositionTable};

use crate::{
    Engine, EngineError, PositionBase, SearchOutcome, SearchRequest, format_uci_move,
    types::{DEFAULT_SEARCH_DEPTH, DEFAULT_TRANSPOSITION_TABLE_SIZE_MB, MAX_SEARCH_DEPTH},
};

type EnginePosition = DefaultPosition<DefaultAttackTable>;
type EngineMoveGenerator = DefaultMoveGenerator<DefaultAttackTable>;
type EngineAccumulator = DefaultAccumulator<PSQTEvalState>;
type EngineTranspositionTable = DefaultTranspositionTable<SearchNode>;

/// `Engine` is the composed, protocol-agnostic chess engine session
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
    /// @return: initialized engine, or an engine error
    pub fn new() -> Result<Self, EngineError> {
        let (position, accumulator) = Self::build_position(PositionBase::StartPos, &[])?;

        Ok(Self {
            position,
            move_generator: EngineMoveGenerator::new(),
            accumulator,
            transposition_table: EngineTranspositionTable::new(DEFAULT_TRANSPOSITION_TABLE_SIZE_MB),
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

    /// search_with searches the current position using the given request
    ///
    /// @param: request - search constraints
    /// @return: completed search outcome, or an engine error
    /// @side-effects: may modify engine search state
    pub fn search_with(&mut self, request: SearchRequest) -> Result<SearchOutcome, EngineError> {
        let depth = request
            .depth
            .unwrap_or(DEFAULT_SEARCH_DEPTH)
            .clamp(1, MAX_SEARCH_DEPTH);
        let started = Instant::now();
        let result = iterative_deepening(
            &mut self.search,
            &mut self.position,
            &self.move_generator,
            &mut self.transposition_table,
            &mut self.accumulator,
            Depth::try_from(depth).expect("supported search depth fits the engine depth type"),
        );

        Ok(SearchOutcome {
            best_move: result.best_move,
            depth,
            score: result.score,
            nodes: result.nodes,
            elapsed: started.elapsed(),
        })
    }

    /// legal_moves returns the legal moves available in the current position
    ///
    /// @return: legal move list for the side to move
    pub fn legal_moves(&self) -> MoveList {
        let mut legal_moves = MoveList::new();
        self.move_generator
            .generate_legal_moves(&self.position, &mut legal_moves);
        legal_moves
    }

    /// Builds a complete replacement state so malformed position updates cannot
    /// leave the live engine partially updated.
    fn build_position(
        base: PositionBase,
        moves: &[&str],
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

        for uci_move in moves {
            let mv = find_legal_move(&move_generator, &position, uci_move)?;
            let eval = accumulator.push_next();
            let delta = position.play_unchecked(mv);
            eval.apply(delta);
        }

        Ok((position, accumulator))
    }
}

impl Engine for DefaultEngine {
    fn new_game(&mut self) -> Result<(), EngineError> {
        (self.position, self.accumulator) = Self::build_position(PositionBase::StartPos, &[])?;
        self.transposition_table.clear();
        self.search = Negamax::new();
        Ok(())
    }

    fn set_position(&mut self, base: PositionBase, moves: &[&str]) -> Result<(), EngineError> {
        (self.position, self.accumulator) = Self::build_position(base, moves)?;
        Ok(())
    }

    fn play_uci(&mut self, uci: &str) -> Result<(), EngineError> {
        let mv = find_legal_move(&self.move_generator, &self.position, uci)?;
        self.apply(mv)
    }

    fn apply(&mut self, mv: Move) -> Result<(), EngineError> {
        let eval = self.accumulator.push_next();
        let delta = self.position.play_unchecked(mv);
        eval.apply(delta);
        Ok(())
    }

    fn search(&mut self, depth: u8) -> Result<SearchOutcome, EngineError> {
        self.search_with(SearchRequest::with_depth(depth))
    }

    fn has_legal_moves(&self) -> bool {
        !self.legal_moves().as_slice().is_empty()
    }
}

impl Display for DefaultEngine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.position.fmt(f)
    }
}

/// find_legal_move resolves a UCI move string against the current legal moves
///
/// @param: move_generator - move generator used to enumerate legal moves
/// @param: position - position to search from
/// @param: uci - UCI move string
/// @return: matching legal move, or an engine error
fn find_legal_move(
    move_generator: &EngineMoveGenerator,
    position: &EnginePosition,
    uci: &str,
) -> Result<Move, EngineError> {
    let mut legal_moves = MoveList::new();
    move_generator.generate_legal_moves(position, &mut legal_moves);
    legal_moves
        .as_slice()
        .iter()
        .copied()
        .find(|mv| format_uci_move(*mv) == uci)
        .ok_or_else(|| EngineError::new(format!("illegal move: {uci}")))
}

#[cfg(test)]
mod tests {
    use chess_kit_position::PositionView;
    use chess_kit_primitives::{Pieces, Sides, Square};

    use super::*;

    #[test]
    fn position_applies_legal_move_history() {
        let mut engine = DefaultEngine::new().unwrap();
        engine
            .set_position(PositionBase::StartPos, &["e2e4", "e7e5"])
            .unwrap();

        assert_eq!(engine.position.piece_at(Square::E4), Pieces::Pawn);
        assert_eq!(engine.position.piece_at(Square::E5), Pieces::Pawn);
        assert_eq!(engine.position.turn(), Sides::White);
    }

    #[test]
    fn search_returns_a_legal_move() {
        let mut engine = DefaultEngine::new().unwrap();
        let outcome = engine.search(1).unwrap();

        assert!(outcome.best_move.is_some());
        assert_eq!(outcome.depth, 1);
        assert!(outcome.nodes > 1);
    }

    #[test]
    fn play_uci_rejects_illegal_moves_without_changing_turn() {
        let mut engine = DefaultEngine::new().unwrap();
        let turn = engine.position.turn();

        assert!(engine.play_uci("e2e5").is_err());
        assert_eq!(engine.position.turn(), turn);
    }

    #[test]
    fn has_legal_moves_is_true_from_the_start_position() {
        let engine = DefaultEngine::new().unwrap();
        assert!(engine.has_legal_moves());
    }
}
