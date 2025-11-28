use crate::board::Board;
use crate::movegen::MoveGenerator;
use crate::primitives::{Move, MoveList, MoveType};

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
        if !move_generator.square_attacked(board, board.turn(), board.king_square(board.opponent()))
        {
            nodes += perft(board, move_generator, depth - 1);
        }

        // undo the move
        board.unmake_move();
    }

    nodes
}

pub fn perft_divide(
    board: &mut Board,
    move_generator: &MoveGenerator,
    depth: u8,
) -> Vec<(Move, u64)> {
    assert!(depth > 0);

    let mut ml = MoveList::new();
    move_generator.generate_moves(board, &mut ml, MoveType::All);

    let mut result = Vec::with_capacity(ml.len());

    for mv in ml.iter() {
        board.make_move(mv);

        // check if the move leaves the king in check
        let mut nodes = 0;
        if !move_generator.square_attacked(board, board.turn(), board.king_square(board.opponent()))
        {
            nodes += perft(board, move_generator, depth - 1);
        }

        // undo the move
        board.unmake_move();
        result.push((mv, nodes));
    }

    result
}

pub fn perft_divide_print(board: &mut Board, move_generator: &MoveGenerator, depth: u8) -> u64 {
    let entries = perft_divide(board, move_generator, depth);
    let mut total = 0;
    for (mv, nodes) in entries {
        println!("{}: {}", mv, nodes);
        total += nodes;
    }
    println!("Total: {total}");
    total
}
