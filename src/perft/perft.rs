use crate::board::Board;
use crate::movegen::MoveGenerator;
use crate::primitives::{MoveList, MoveType};

pub fn perft(board: &mut Board, move_generator: &MoveGenerator, depth: u8) -> u64 {
    let mut move_list = MoveList::new();
    let mut nodes = 0;

    if depth == 0 {
        return 1;
    }

    move_generator.generate_moves(board, &mut move_list, MoveType::All);
    for mv in move_list.iter() {
        board.make_move(mv);

        // check if the move leaves the king in check
        if !move_generator.square_attacked(board, board.opponent(), mv.to()) {
            nodes += perft(board, move_generator, depth - 1);
        }

        // undo the move
        board.unmake_move();
    }

    nodes
}
