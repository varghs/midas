mod test;

use midas::{
    engine::attacks::pawn::PawnAttacks,
    engine::{
        bitboard::{print_bitboard, Bitboard},
        board::{Board, Color},
        square::Square,
    },
    set_bit,
};

fn main() {
    for i in 0..64 {
        println!("{}", i);
        print_bitboard(PawnAttacks::mask_pawn_attacks(
            (i as u64).try_into().unwrap(),
            Color::Black,
        ));
    }
}
