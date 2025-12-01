use chess_kit::position::Position;

const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() {
    let board = Position::try_from(DEFAULT_FEN);
    match board {
        Ok(board) => println!("Board: {}", board),
        Err(e) => println!("Error in parsing the FEN-string: {}", e),
    }
}
