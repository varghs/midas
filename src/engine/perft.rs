use std::time::Instant;

use super::{
    board::{Board, BoardState},
    r#move::{MoveList, MoveType},
};

pub fn perft_driver(mut board_state: &mut BoardState, nodes: &mut u64, depth: u16) {
    // perft driver

    if depth == 0 {
        *nodes += 1;
        return;
    }

    // let copy_board_state = board_state.clone();
    board_state.board.generate_moves();
    for m in (&board_state.board.move_list.moves[..board_state.board.move_list.count]).to_vec() {
        // preserve the state so u can later restore it
        board_state.preserve();
        board_state.make_move(m, MoveType::AllMoves);

        // call perft recursively
        perft_driver(&mut board_state, nodes, depth - 1);
        board_state.restore();
    }
}

pub fn perft_tester(board_state: &mut BoardState, nodes: &mut u64, depth: u16) {
    println!("\n\n\tPERFORMANCE TEST\n");

    let start_time = Instant::now();
    if depth == 0 {
        *nodes += 1;
        return;
    }

    let mut copy_board_state = board_state.clone();
    copy_board_state.board.generate_moves();

    for m in
        (&copy_board_state.board.move_list.moves[..copy_board_state.board.move_list.count]).to_vec()
    {
        // preserve the state so u can later restore it
        copy_board_state.preserve();
        copy_board_state.make_move(m, MoveType::AllMoves);

        let cummalitive_nodes = nodes.clone();

        // call perft recursively
        perft_driver(&mut copy_board_state, nodes, depth - 1);

        let old_nodes: u64 = *nodes - cummalitive_nodes;

        copy_board_state.restore();

        println!("\tmove {}\t\t nodes: {}", m, old_nodes);
    }
    let end_time = Instant::now();

    println!("\n\tDepth: {}", depth);
    println!("\tNodes: {}", *nodes);
    println!("\tTime: {:?}", end_time - start_time);
}
