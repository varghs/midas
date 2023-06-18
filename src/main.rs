use std::mem::size_of;

use midas::{
    engine::{
        attacks::slider_attacks::SliderAttacks,
        attacks::AttackTables,
        bitboard::{print_bitboard, Bitboard, EMPTY, LS1B, ONE},
        board::{Board, BoardState, Castle, Color, Piece},
        fen::*,
        move_gen::*,
        r#move::{Move, MoveList},
        square::Square,
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
            let mut tables = AttackTables::new();
            tables.populate();

            let updated_init = FEN("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1 ");

            let mut b = BoardState::new();
            let fen = FEN("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq c6 0 1 ");

            b.board.parse_fen(START_POSITION);
            println!("{}", b.board);
            b.preserve();

            b.board.parse_fen(fen);
            println!("{}", b.board);

            b.restore();
            println!("{}", b.board);
        })
        .unwrap()
        .join()
        .unwrap();
}
