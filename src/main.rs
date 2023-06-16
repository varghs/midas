use std::mem::size_of;

use midas::{
    engine::{
        attacks::AttackTables,
        attacks::slider_attacks::SliderAttacks,
        bitboard::{print_bitboard, Bitboard, EMPTY, LS1B, ONE},
        board::{Board, Color, Piece, Castle},
        square::Square,
        fen::*,
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

            let mut b = Board::new();

            b.parse_fen(FEN("r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9 "));
            println!("{}", b);
        }).unwrap().join().unwrap();
}
