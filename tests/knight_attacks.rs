use midas::engine::attacks::knight_attacks::KnightAttacks;
use midas::engine::bitboard::{Bitboard, EMPTY};
use midas::engine::board::Color;
use midas::engine::square::Square;
use midas::set_bit;

#[test]
fn a1() {
    let mut bitboard: Bitboard = EMPTY;
    set_bit!(bitboard, Square::c2);
    set_bit!(bitboard, Square::b3);
    assert_eq!(KnightAttacks::mask_knight_attacks(Square::a1), bitboard);
}

#[test]
fn a2() {}

#[test]
fn a3() {}

#[test]
fn a7() {}

#[test]
fn a8() {}

#[test]
fn b1() {}

#[test]
fn b2() {}

#[test]
fn b3() {}

#[test]
fn b7() {}

#[test]
fn b8() {}

#[test]
fn d3() {
    let mut bitboard: Bitboard = EMPTY;
    set_bit!(bitboard, Square::c1);
    set_bit!(bitboard, Square::c5);
    set_bit!(bitboard, Square::e1);
    set_bit!(bitboard, Square::f2);
    set_bit!(bitboard, Square::f4);
    set_bit!(bitboard, Square::e5);
    set_bit!(bitboard, Square::b4);
    set_bit!(bitboard, Square::b2);
    assert_eq!(KnightAttacks::mask_knight_attacks(Square::d3), bitboard);
}
