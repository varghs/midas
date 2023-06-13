use midas::{
    engine::attacks::bishop_attacks::BishopAttacks,
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
        print_bitboard(KingAttacks::mask_king_attacks((i as usize).try_into().unwrap()));
        println!();
    }
    */

    print_bitboard(BishopAttacks::mask_bishop_attacks(Square::d4));
}
