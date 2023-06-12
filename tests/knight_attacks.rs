use midas::engine::attacks::knight_attacks::KnightAttacks;
use midas::engine::bitboard::Bitboard;
use midas::set_bit;
use midas::engine::square::Square;

#[test]
fn a1() {
    let mut bitboard: Bitboard = 0;
    set_bit!(bitboard, Square::b3);
    set_bit!(bitboard, Square::c2);
    assert_eq!(bitboard, KnightAttacks::mask_knight_attacks(Square::a1));
}

#[test]
fn a2() {
    let mut bitboard: Bitboard = 0;
}

/*
#[test]
fn a3() { let bitboard: Bitboard = 0; }
#[test]
fn a7() { let bitboard: Bitboard = 0; }
#[test]
fn a8() { let bitboard: Bitboard = 0; }
#[test]
fn b1() { let bitboard: Bitboard = 0; }
#[test]
fn b2() { let bitboard: Bitboard = 0; }
#[test]
fn b3() { let bitboard: Bitboard = 0; }
#[test]
fn b7() { let bitboard: Bitboard = 0; }
#[test]
fn b8() { let bitboard: Bitboard = 0; }
#[test]
fn c1() { let bitboard: Bitboard = 0; }
#[test]
fn c2() { let bitboard: Bitboard = 0; }
#[test]
fn c3() { let bitboard: Bitboard = 0; }
#[test]
fn c7() { let bitboard: Bitboard = 0; }
#[test]
fn c8() { let bitboard: Bitboard = 0; }
#[test]
fn g1() { let bitboard: Bitboard = 0; }
#[test]
fn g2() { let bitboard: Bitboard = 0; }
#[test]
fn g3() { let bitboard: Bitboard = 0; }
#[test]
fn g7() { let bitboard: Bitboard = 0; }
#[test]
fn g8() { let bitboard: Bitboard = 0; }
#[test]
fn h1() { let bitboard: Bitboard = 0; }
#[test]
fn h2() { let bitboard: Bitboard = 0; }
#[test]
fn h3() { let bitboard: Bitboard = 0; }
#[test]
fn h7() { let bitboard: Bitboard = 0; }
#[test]
fn h8() { let bitboard: Bitboard = 0; }
*/
