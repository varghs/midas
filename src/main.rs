use std::{io::stdin, mem::size_of, time::Instant};

use midas::{
    engine::{
        attacks::slider_attacks::SliderAttacks,
        attacks::AttackTables,
        bitboard::{print_bitboard, Bitboard, EMPTY, LS1B, ONE},
        board::{Board, BoardState, Castle, Color, Piece},
        fen::*,
        move_gen::*,
        perft::{perft_driver, perft_tester},
        r#move::{Move, MoveList, MoveType},
        uci::parse_move_string,
    },
    set_bit,
};

fn main() {
    /*
    for i in 0..64 {
        print_bitboard(RookAttacks::mask_rook_attack((i as usize).try_into().unwrap()));
        println!();
    }
    */
    const N: usize = 1_000_000_000;

    std::thread::Builder::new()
        .stack_size(size_of::<u64>() * N)
        .spawn(|| {
            let mut b = BoardState::new();
            // let mut buf = String::new();
            let fen = FEN("r3k2r/p1ppRpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1 ");
            let mut input = String::new();

            b.board.parse_fen(START_POSITION);
            
            let move_list = b.board.generate_moves();

            /*
            for m in (&move_list.moves[..move_list.count]).to_vec() {
                b.preserve();
                println!("{}", b.board);
                stdin().read_line(&mut input).expect("Failure");

                b.make_move(m, MoveType::AllMoves);
                println!("{}", b.board);
                stdin().read_line(&mut input).expect("Failure");

                b.restore();
            }
            */

            /*
            parse_move_string("e2e4");
            parse_move_string("a1d2");
            */

            let mut nodes: u64 = 0;
            perft_tester(&mut b, &mut nodes, 6);
            println!("{}", nodes);
        })
        .unwrap()
        .join()
        .unwrap();
}
