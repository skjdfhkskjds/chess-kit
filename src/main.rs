use std::str::FromStr;
use std::time::Instant;

use chess_kit::attack_table::DefaultAttackTable;
use chess_kit::comm::uci::{
    BasePosition, PositionCommand, SearchInfo, SearchLimits, SearchResult, UciEngine, UciMove,
};
use chess_kit::eval::{Accumulator, DefaultAccumulator, EvalState, PSQTEvalState};
use chess_kit::movegen::{DefaultMoveGenerator, MoveGenerator};
use chess_kit::position::{DefaultPosition, Fen, PositionMoves, Setup};
use chess_kit::primitives::{Depth, Move, MoveList};
use chess_kit::search::{Negamax, SearchNode, iterative_deepening};
use chess_kit::transposition::{DefaultTranspositionTable, TranspositionTable};

const START_POSITION_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const DEFAULT_SEARCH_DEPTH: u8 = 4;
const MAX_SEARCH_DEPTH: u8 = 8;
const DEFAULT_TRANSPOSITION_TABLE_SIZE_MB: usize = 1024;

type EnginePosition = DefaultPosition<DefaultAttackTable>;
type EngineMoveGenerator = DefaultMoveGenerator<DefaultAttackTable>;
type EngineAccumulator = DefaultAccumulator<PSQTEvalState>;
type EngineTranspositionTable = DefaultTranspositionTable<SearchNode>;

struct ChessKitEngine {
    position: EnginePosition,
    move_generator: EngineMoveGenerator,
    accumulator: EngineAccumulator,
    transposition_table: EngineTranspositionTable,
    search: Negamax,
}

impl ChessKitEngine {
    fn new() -> Result<Self, String> {
        let command = PositionCommand {
            base: BasePosition::StartPos,
            moves: Vec::new(),
        };
        let (position, accumulator) = Self::build_position(&command)?;

        Ok(Self {
            position,
            move_generator: EngineMoveGenerator::new(),
            accumulator,
            transposition_table: EngineTranspositionTable::new(DEFAULT_TRANSPOSITION_TABLE_SIZE_MB),
            search: Negamax::new(),
        })
    }

    /// Builds a complete replacement state so malformed `position` commands
    /// cannot leave the live engine partially updated.
    fn build_position(
        command: &PositionCommand,
    ) -> Result<(EnginePosition, EngineAccumulator), String> {
        let fen = match &command.base {
            BasePosition::StartPos => START_POSITION_FEN,
            BasePosition::Fen(fen) => fen,
        };

        let fen = Fen::try_from(fen).map_err(|error| format!("invalid FEN: {error}"))?;
        let mut position = EnginePosition::from(Setup::from(fen));
        let eval = PSQTEvalState::from_position(&position);
        let mut accumulator = EngineAccumulator::new();
        accumulator.push(eval);
        let move_generator = EngineMoveGenerator::new();

        for uci_move in &command.moves {
            let mut legal_moves = MoveList::new();
            move_generator.generate_legal_moves(&position, &mut legal_moves);
            let mv = legal_moves
                .as_slice()
                .iter()
                .copied()
                .find(|mv| format_move(*mv) == uci_move.as_str())
                .ok_or_else(|| format!("illegal move in position command: {uci_move}"))?;

            let eval = accumulator.push_next();
            let delta = position.play_unchecked(mv);
            eval.apply(delta);
        }

        Ok((position, accumulator))
    }

    fn search_depth(limits: &SearchLimits) -> u8 {
        limits
            .depth
            .unwrap_or(DEFAULT_SEARCH_DEPTH)
            .clamp(1, MAX_SEARCH_DEPTH)
    }
}

impl UciEngine for ChessKitEngine {
    type Error = String;

    fn name(&self) -> &str {
        concat!("chess-kit ", env!("CARGO_PKG_VERSION"))
    }

    fn author(&self) -> &str {
        "chess-kit contributors"
    }

    fn new_game(&mut self) -> Result<(), Self::Error> {
        let command = PositionCommand {
            base: BasePosition::StartPos,
            moves: Vec::new(),
        };
        (self.position, self.accumulator) = Self::build_position(&command)?;
        self.transposition_table.clear();
        self.search = Negamax::new();
        Ok(())
    }

    fn set_position(&mut self, command: &PositionCommand) -> Result<(), Self::Error> {
        (self.position, self.accumulator) = Self::build_position(command)?;
        Ok(())
    }

    fn search(&mut self, limits: &SearchLimits) -> Result<SearchResult, Self::Error> {
        let depth = Self::search_depth(limits);
        let started = Instant::now();
        let result = iterative_deepening(
            &mut self.search,
            &mut self.position,
            &self.move_generator,
            &mut self.transposition_table,
            &mut self.accumulator,
            Depth::try_from(depth).expect("supported UCI depth fits the engine depth type"),
        );

        let mut uci_result = SearchResult::new(
            result
                .best_move
                .map(format_move)
                .map(|mv| UciMove::from_str(&mv).expect("engine moves have valid UCI notation")),
        );
        uci_result.info = SearchInfo {
            depth: Some(depth),
            score_cp: Some(result.score),
            nodes: Some(result.nodes),
            elapsed: Some(started.elapsed()),
        };
        Ok(uci_result)
    }
}

fn format_move(mv: Move) -> String {
    // Piece display uses uppercase letters, while UCI promotions are lowercase.
    mv.to_string().to_ascii_lowercase()
}

fn main() {
    if let Err(error) = run() {
        eprintln!("chess-kit: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = ChessKitEngine::new()?;
    chess_kit::comm::uci::run(&mut engine)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use chess_kit::position::PositionView;
    use chess_kit::primitives::{Pieces, Sides, Square};

    use super::*;

    #[test]
    fn position_command_applies_legal_move_history() {
        let command = PositionCommand {
            base: BasePosition::StartPos,
            moves: vec!["e2e4".parse().unwrap(), "e7e5".parse().unwrap()],
        };

        let (position, _) = ChessKitEngine::build_position(&command).unwrap();

        assert_eq!(position.piece_at(Square::E4), Pieces::Pawn);
        assert_eq!(position.piece_at(Square::E5), Pieces::Pawn);
        assert_eq!(position.turn(), Sides::White);
    }

    #[test]
    fn search_returns_a_legal_uci_move() {
        let mut engine = ChessKitEngine::new().unwrap();

        let result = engine
            .search(&SearchLimits {
                depth: Some(1),
                ..SearchLimits::default()
            })
            .unwrap();

        assert!(result.best_move.is_some());
        assert_eq!(result.info.depth, Some(1));
        assert!(result.info.nodes.unwrap() > 1);
    }
}
