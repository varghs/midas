use std::mem::size_of;

use midas::{
    engine::{
        attacks::slider_attacks::SliderAttacks,
        attacks::AttackTables,
        bitboard::{print_attacked_squares, print_bitboard, Bitboard, EMPTY, LS1B, ONE},
        board::{Board, Castle, Color, Piece},
        fen::*,
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

            let mut board = Board::new();

            board.parse_fen(WHITE_PAWN);
            println!("{}", board);
            println!();
            print_attacked_squares(&board, &tables, Color::White);
        })
        .unwrap()
        .join()
        .unwrap();
}
