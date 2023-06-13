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
fn a2() {
    let mut bitboard: Bitboard = EMPTY;
    set_bit!(bitboard, Square::c3);
    set_bit!(bitboard, Square::b4);
    set_bit!(bitboard, Square::c1);
    assert_eq!(KnightAttacks::mask_knight_attacks(Square::a2), bitboard);
}
#[test]
fn a3() {
    let mut bitboard: Bitboard = EMPTY;
    set_bit!(bitboard, Square::b5);
    set_bit!(bitboard, Square::c4);
    set_bit!(bitboard, Square::c2);
    set_bit!(bitboard, Square::b1);
    assert_eq!(KnightAttacks::mask_knight_attacks(Square::a3), bitboard);
}
#[test]
fn a7() {
    let mut bitboard: Bitboard = EMPTY;
    set_bit!(bitboard, Square::c8);
    set_bit!(bitboard, Square::c6);
    set_bit!(bitboard, Square::b5);
    assert_eq!(KnightAttacks::mask_knight_attacks(Square::a7), bitboard);
}
#[test]
fn a8() {
    let mut bitboard: Bitboard = EMPTY;
    set_bit!(bitboard, Square::c7);
    set_bit!(bitboard, Square::b6);
    assert_eq!(KnightAttacks::mask_knight_attacks(Square::a8), bitboard);
}

#[test]
fn b1() {
    let mut bitboard: Bitboard = EMPTY;
    set_bit!(bitboard, Square::a3);
    set_bit!(bitboard, Square::c3);
    set_bit!(bitboard, Square::d2);
    assert_eq!(KnightAttacks::mask_knight_attacks(Square::b1), bitboard);
}

#[test]
fn b2() {
    let mut bitboard: Bitboard = EMPTY;
    set_bit!(bitboard, Square::a4);
    set_bit!(bitboard, Square::c4);
    set_bit!(bitboard, Square::d3);
    set_bit!(bitboard, Square::d1);
    assert_eq!(KnightAttacks::mask_knight_attacks(Square::b2), bitboard);
}
#[test]
fn b3() {
    let mut bitboard: Bitboard = EMPTY;
    set_bit!(bitboard, Square::a1);
    set_bit!(bitboard, Square::c1);
    set_bit!(bitboard, Square::a5);
    set_bit!(bitboard, Square::c5);
    set_bit!(bitboard, Square::d4);
    set_bit!(bitboard, Square::d2);
    assert_eq!(KnightAttacks::mask_knight_attacks(Square::b3), bitboard);
}
#[test]
fn b7() {
    let mut bitboard: Bitboard = EMPTY;
    set_bit!(bitboard, Square::c5);
    set_bit!(bitboard, Square::d6);
    set_bit!(bitboard, Square::d8);
    set_bit!(bitboard, Square::a5);
    assert_eq!(KnightAttacks::mask_knight_attacks(Square::b7), bitboard);
}
#[test]
fn b8() {
    let mut bitboard: Bitboard = EMPTY;
    set_bit!(bitboard, Square::a6);
    set_bit!(bitboard, Square::c6);
    set_bit!(bitboard, Square::d7);
    assert_eq!(KnightAttacks::mask_knight_attacks(Square::b8), bitboard);
}

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
