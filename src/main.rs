use std::mem::size_of;

use midas::{
    engine::{
        attacks::AttackTables,
        attacks::slider_attacks::SliderAttacks,
        bitboard::{print_bitboard, Bitboard, EMPTY, LS1B, ONE},
        board::{Board, Color},
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

            let mut occupancy = EMPTY;
            set_bit!(occupancy, Square::c5);
            set_bit!(occupancy, Square::f2);
            set_bit!(occupancy, Square::g7);
            set_bit!(occupancy, Square::b2);
            set_bit!(occupancy, Square::g5);
            set_bit!(occupancy, Square::e2);
            set_bit!(occupancy, Square::e7);

            print_bitboard(occupancy);
            println!();
            print_bitboard(tables.sliders.bishops.get_bishop_attack(Square::d4, occupancy));
            println!();
            print_bitboard(tables.sliders.rooks.get_rook_attack(Square::e5, occupancy));

        }).unwrap().join().unwrap();
}
