use chess_kit::attack_table::DefaultAttackTable;
use chess_kit::position::{DefaultPosition, Position, PositionFromFEN};
use chess_kit::primitives::DefaultState;

const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() {
    let mut pos = DefaultPosition::<DefaultAttackTable, DefaultState>::new();
    match pos.load_fen(DEFAULT_FEN) {
        Ok(()) => println!("Board: {}", pos),
        Err(e) => println!("Error in parsing the FEN-string: {}", e),
    }
}
