use std::mem::size_of;

use midas::{
    engine::{
        attacks::AttackTables,
        attacks::slider_attacks::SliderAttacks,
        bitboard::{print_bitboard, Bitboard, EMPTY, LS1B, ONE},
        board::{Board, Color, Piece, Castle},
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

            let mut b = Board::new();
            println!("{}", b);

            b.side = Color::Black;
            b.en_passant_sq = Some(Square::f4);
            b.castle.set_castle(Castle::WhiteKing);
            b.castle.set_castle(Castle::BlackQueen);
            println!("{}", b);
        }).unwrap().join().unwrap();
}
