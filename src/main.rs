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
    for i in 0..64 {
        print_bitboard(RookAttacks::mask_rook_attacks((i as usize).try_into().unwrap()));
        println!();
    }

    print_bitboard(BishopAttacks::bishop_attacks_otf(square, block))
}
