use std::time::Instant;

use super::{
    board::{Board, BoardState},
    r#move::{MoveList, MoveType},
};

pub fn perft_driver(board_state: &mut BoardState, nodes: &mut u64, depth: u16) {
    // perft driver

    let mut input = String::new();

    if depth == 0 {
        *nodes += 1;
        return;
    }

    // let copy_board_state = board_state.clone();
    let move_list = board_state.board.generate_moves();

    for m in (&move_list.moves[..move_list.count]).to_vec() {
        // preserve the state so u can later restore it
        let c = board_state.preserve();

        if !board_state.make_move(m, MoveType::AllMoves) {
            continue;
        }

        // call perft recursively
        perft_driver(board_state, nodes, depth - 1);
        board_state.restore(&c);
    }
}

pub fn perft_tester(board_state: &mut BoardState, nodes: &mut u64, depth: u16) {
    println!("\n\n\tPERFORMANCE TEST\n");

    let mut input = String::new();

    let start_time = Instant::now();
    if depth == 0 {
        *nodes += 1;
        return;
    }

    let move_list = board_state.board.generate_moves();

    for m in (&move_list.moves[..move_list.count]).to_vec() {
        // preserve the state so u can later restore it
        let c = board_state.preserve();

        if !board_state.make_move(m, MoveType::AllMoves) {
            continue;
        }

        let cumulative_nodes = nodes.clone();

        // call perft recursively
        perft_driver(board_state, nodes, depth - 1);

        let old_nodes: u64 = *nodes - cumulative_nodes;

        board_state.restore(&c);

        println!("\tmove {}\t\t nodes: {}", m, old_nodes);
    }
    let end_time = Instant::now();

    println!("\n\tDepth: {}", depth);
    println!("\tNodes: {}", *nodes);
    println!("\tTime: {:?}", end_time - start_time);
}
