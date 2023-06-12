use midas::{
    engine::attacks::knight_attacks::KnightAttacks,
    engine::{
        bitboard::{print_bitboard, Bitboard},
        board::{Board, Color},
        square::Square,
    },
    set_bit,
};

fn main() {
    for i in 0..64 {
        print_bitboard(KnightAttacks::mask_knight_attacks(
            (i as u64).try_into().unwrap(),
        ));
        println!();
    }
}
