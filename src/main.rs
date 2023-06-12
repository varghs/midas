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
    /*
    for i in 0..64 {
        print_bitboard(KnightAttacks::mask_knight_attacks(
            (i as u64).try_into().unwrap(),
        ));
        println!();
    }
    */

    print_bitboard(KingAttacks::mask_king_attacks(Square::a1));
}
