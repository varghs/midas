use midas::{
    engine::attacks::bishop_attacks::BishopAttacks,
    engine::{
        attacks::{init_magic_testing, set_occupancy, rook_attacks::RookAttacks},
        bitboard::{print_bitboard, Bitboard, LS1B, ONE, EMPTY},
        board::{Board, Color},
        square::Square,
    },
    set_bit,
};

fn main() {
    /*
    for i in 0..64 {
        print_bitboard(RookAttacks::mask_rook_attacks((i as usize).try_into().unwrap()));
        println!();
    }
    */

    let mut blockers: Bitboard = EMPTY;
    set_bit!(blockers, Square::d5);
    set_bit!(blockers, Square::f4);
    set_bit!(blockers, Square::d2);
    set_bit!(blockers, Square::b4);


    let attack_mask = RookAttacks::get_rook_attack(Square::d4, blockers);
    print_bitboard(attack_mask);
}
