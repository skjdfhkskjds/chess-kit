use chess_kit::attack_table::DefaultAttackTable;
use chess_kit::eval::{Accumulator, DefaultAccumulator, EvalState, PSQTEvalState};
use chess_kit::position::{DefaultPosition, DefaultState, Position, PositionFromFEN};

const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() {
    let mut pos = DefaultPosition::<DefaultAttackTable, DefaultState>::new();
    let mut accumulator = DefaultAccumulator::<PSQTEvalState>::new();
    match pos.load_fen::<PSQTEvalState>(DEFAULT_FEN) {
        Ok(eval) => {
            println!("Board: {}", pos);
            accumulator.push(eval);
            println!("Score: {}", accumulator.latest_mut().score());
        }
        Err(e) => panic!("Error in parsing the FEN-string: {}", e),
    }
}
