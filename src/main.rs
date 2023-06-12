use midas::{
    engine::attacks::king_attacks::KingAttacks,
    engine::{
        bitboard::{print_bitboard, Bitboard},
        board::{Board, Color},
        square::Square,
    },
    set_bit,
};

fn main() {
    for i in 0..64 {
        print_bitboard(KingAttacks::mask_king_attacks((i as usize).try_into().unwrap()));
        println!();
    }
}
