use midas::{
    engine::attacks::bishop_attacks::BishopAttacks,
    engine::{
        attacks::{init_magic_testing, set_occupancy},
        bitboard::{print_bitboard, Bitboard, LS1B},
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

<<<<<<< HEAD
    print_bitboard(BishopAttacks::bishop_attacks_otf(square, block))
=======
    // print_bitboard(BishopAttacks::mask_bishop_attacks(Square::d4));

    let attack_mask = BishopAttacks::mask_bishop_attacks(Square::d4);

    // the 4095 is some fancy math.. but basically it ensures
    // we go through all the possible occupancies

    init_magic_testing();
>>>>>>> fe2b1d5ba7c3287f20f2bb1ad96f96992bd94ec9
}
