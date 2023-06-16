use midas::{
    engine::{
        attacks::slider_attacks::{init_magic_testing, set_occupancy},
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
    
    init_magic_testing();
}
